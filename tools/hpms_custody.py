"""Inspect HPMS custody receipts; capture only a fixed, bounded publisher probe."""
import argparse
import csv
import datetime
import hashlib
import json
from pathlib import Path
import urllib.parse
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data/hpms-custody-investigation.json"
MAX_BODY = 1_000_000
PREDICATE = "f_system = 1 AND aadt IS NOT NULL"
LAYER = "https://geo.dot.gov/server/rest/services/Hosted/California_2018_PR/FeatureServer/0"


def query(**extra):
    parameters = {"where": PREDICATE, "f": "pjson", "returnGeometry": "false"}
    return LAYER + "/query?" + urllib.parse.urlencode(parameters | extra)


URLS = {
    "publisher-page": "https://www.fhwa.dot.gov/policyinformation/hpms/shapefiles.cfm",
    "california-service": LAYER + "?f=pjson",
    "california-count": query(returnCountOnly="true"),
    "california-years": query(outFields="year_record", returnDistinctValues="true"),
    "california-sample": query(
        outFields="objectid,year_record,state_code,route_id,route_number,route_name,route_signing,aadt,aadt_combination,iri,through_lanes,speed_limit,f_system",
        orderByFields="objectid ASC", resultRecordCount="5", resultOffset="0",
    ),
}


def digest(body):
    return hashlib.sha256(body).hexdigest()


def payload(body):
    data = json.loads(body)
    if not isinstance(data, dict) or "error" in data:
        raise ValueError("ArcGIS error or malformed JSON object")
    return data


def reported_year(attributes):
    value = attributes.get("year_record")
    if type(value) is not int or not 1900 <= value <= 2100:
        raise ValueError("Missing or malformed reported year; URL is not year evidence")
    return value


def analyse(bodies):
    if set(bodies) != set(URLS):
        raise ValueError("Expected exactly the five bounded probe responses")
    if b"YEAR_RECORD" not in bodies["publisher-page"]:
        raise ValueError("Publisher field documentation missing")
    metadata = payload(bodies["california-service"])
    if "year_record" not in {f.get("name", "").lower() for f in metadata.get("fields", [])}:
        raise ValueError("Service does not expose the documented year field")
    count = payload(bodies["california-count"]).get("count")
    if type(count) is not int or count < 0:
        raise ValueError("Malformed source-side count")
    years = payload(bodies["california-years"])
    if years.get("exceededTransferLimit") is not False:
        raise ValueError("Distinct-year response is incomplete or unqualified")
    observed_years = sorted({reported_year(f["attributes"]) for f in years["features"]})
    sample = payload(bodies["california-sample"])
    rows = sample["features"]
    if len(rows) > 5 or len(rows) > count or (count > 0 and not observed_years):
        raise ValueError("Sample/count/year scope is inconsistent")
    sample_years = sorted({reported_year(f["attributes"]) for f in rows})
    if not set(sample_years).issubset(observed_years):
        raise ValueError("Sample years disagree with the complete distinct-year query")
    ids = [f["attributes"]["objectid"] for f in rows]
    if any(type(value) is not int for value in ids) or ids != sorted(set(ids)):
        raise ValueError("Expected unique, ordered publisher object IDs")
    return {
        "state": "CA", "predicate": PREDICATE,
        "population": "Current service-side California predicate, before Rust route filtering; not I-80-only",
        "source_side_count": count, "distinct_reported_years": observed_years,
        "distinct_year_query_complete": True,
        "sample_rows": len(rows),
        "sample_complete": len(rows) == count and not sample.get("exceededTransferLimit", False),
        "historical_packet_input_hash": None, "historical_packet_input_year": None,
        "historical_execution_attested": False,
        "normalization_or_packet_replay_performed": False,
    }


def verify_receipt(ledger):
    if ledger["schema"] != "route.hpms-custody-investigation.v1":
        raise ValueError("Unknown custody schema")
    for item in ledger["historical_snapshots"]:
        body = (ROOT / item["frozen_path"]).read_bytes()
        if len(body) != item["bytes"] or digest(body) != item["sha256"]:
            raise ValueError("Historical snapshot hash mismatch")
        if not all(line in body.decode("utf-8").splitlines() for line in item["relevant_lines"]):
            raise ValueError("Historical claim locator missing")
    for item in ledger["preserved_artifacts"] + ledger["normalizer_sources"]:
        if digest((ROOT / item["path"]).read_bytes().replace(b"\r\n", b"\n")) != item["sha256"]:
            raise ValueError("Preserved packet/source boundary changed: " + item["path"])
    observation = ledger["publisher_observation"]
    records = observation["captures"]
    if {r["name"] for r in records} != set(URLS) or len(records) != 5:
        raise ValueError("Capture set is incomplete")
    for record in records:
        if record["url"] != URLS[record["name"]] or record["status"] != 200:
            raise ValueError("Capture URL/status does not match the bounded plan")
        if record["bytes"] > MAX_BODY or len(record["sha256"]) != 64:
            raise ValueError("Invalid capture byte/hash envelope")
        datetime.datetime.fromisoformat(record["retrieved_at_utc"])
    summary = observation["summary"]
    if summary["historical_execution_attested"] or summary["historical_packet_input_hash"] is not None or summary["historical_packet_input_year"] is not None:
        raise ValueError("Prospective capture cannot attest historical execution")
    if ledger["historical_input_recovered"] or ledger["packet_or_cache_regenerated"]:
        raise ValueError("This inventory does not recover or regenerate the historical input")
    return ledger


def verify_raw(directory, records, expected):
    bodies = {}
    for record in records:
        if record["name"] not in URLS:
            raise ValueError("Unknown raw capture name")
        body = (directory / (record["name"] + ".body")).read_bytes()
        if len(body) != record["bytes"] or digest(body) != record["sha256"]:
            raise ValueError("Raw capture digest mismatch")
        bodies[record["name"]] = body
    actual = analyse(bodies)
    if actual != expected:
        raise ValueError("Raw capture semantics differ from the receipt")
    return actual


def verify_normalized(path, expected):
    body = path.read_bytes()
    if digest(body) != expected["sha256"] or len(body) != expected["bytes"]:
        raise ValueError("Normalized candidate differs from this recorded observation; retain and investigate it separately")
    rows = list(csv.DictReader(body.decode("utf-8-sig").splitlines()))
    if len(rows) != expected["rows"] or sorted({r["STATE"] for r in rows}) != expected["states"]:
        raise ValueError("Normalized candidate population changed")
    return {"rows": len(rows), "states": len(expected["states"]), "historical_linkage_attested": False}


def capture(output):
    output = output.resolve()
    if not output.is_relative_to((ROOT / ".cache").resolve()) or output == (ROOT / ".cache").resolve():
        raise ValueError("Capture output must be a new directory inside .cache")
    output.mkdir(parents=True, exist_ok=False)
    records, bodies = [], {}
    for name, url in URLS.items():
        request = urllib.request.Request(url, headers={"User-Agent": "ROUTE-source-custody/1.0"})
        with urllib.request.urlopen(request, timeout=25) as response:
            final = urllib.parse.urlsplit(response.url)
            if final.scheme != "https" or final.hostname not in {"www.fhwa.dot.gov", "geo.dot.gov"}:
                raise ValueError("Publisher redirect left the allowlisted HTTPS hosts")
            body = response.read(MAX_BODY + 1)
            if len(body) > MAX_BODY or response.status != 200:
                raise ValueError("Capture exceeds byte/status limits")
            record = {"name": name, "url": url, "final_url": response.url, "status": response.status,
                      "content_type": response.headers.get("Content-Type"), "bytes": len(body),
                      "sha256": digest(body), "retrieved_at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat()}
        (output / (name + ".body")).write_bytes(body)
        bodies[name] = body
        records.append(record)
    result = {"captures": records, "summary": analyse(bodies)}
    (output / "observation.json").write_bytes((json.dumps(result, indent=2) + "\n").encode())
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["verify", "capture"])
    parser.add_argument("--capture-dir", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--normalized-cache", type=Path)
    args = parser.parse_args()
    if args.command == "capture":
        if args.output is None:
            parser.error("capture requires --output inside .cache")
        result = capture(args.output)
        print(json.dumps(result["summary"], indent=2))
        return
    ledger = verify_receipt(json.loads(LEDGER.read_text(encoding="utf-8")))
    if args.normalized_cache:
        verify_normalized(args.normalized_cache, ledger["available_normalized_candidate"])
        print("Normalized candidate matches the dated local observation; no historical linkage attested")
    if args.capture_dir:
        observation = ledger["publisher_observation"]
        verify_raw(args.capture_dir, observation["captures"], observation["summary"])
        print("Archived raw publisher bodies and query semantics verified; historical input remains unattested")
    else:
        print("First-party history/receipt verified offline; publisher bodies not re-fetched or re-verified")


if __name__ == "__main__":
    main()
