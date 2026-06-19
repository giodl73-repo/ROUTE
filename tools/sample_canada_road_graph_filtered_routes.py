#!/usr/bin/env python3
"""Sample Canada road-graph route identifiers with bounded filters."""

from __future__ import annotations

import csv
import json
import urllib.parse
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
RESOLUTION = ROOT / "data" / "international-canada-source-payload-resolution-001.csv"
OUTPUT = ROOT / "data" / "international-canada-road-graph-filtered-route-sample-001.csv"
USER_AGENT = "ROUTE-Canada-filtered-route-sample/0.1 evidence-not-accepted"

FIELDS = [
    "sample_id",
    "source_id",
    "extraction_window",
    "query_ref",
    "object_id",
    "route_number_1",
    "route_name_1",
    "road_class",
    "type_code",
    "nhs_description",
    "sample_method",
    "geometry_status",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]

OUT_FIELDS = "OBJECTID,rtnumber1,rtename1,roadclass,type_code,desc_en"
BLOCKED = (
    "official_network;route_designation;construction_ready;engineering_precision;"
    "guaranteed_sla;roi;eligibility;compliance;endorsement;validation;"
    "public_readiness;external_readiness"
)
OBJECT_IDS = ["17", "18", "19", "20", "21"]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def write_csv(path: Path, rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def layer_query_url() -> str:
    for row in read_csv(RESOLUTION):
        if row["source_id"] == "CAN-SRC-001" and row["resolution_type"] == "esri-rest-layer":
            return row["resolved_url"].replace("?f=pjson", "/query")
    raise RuntimeError("missing CAN-SRC-001 ESRI REST layer resolution")


def fetch_features() -> list[dict]:
    params = {
        "objectIds": ",".join(OBJECT_IDS),
        "outFields": OUT_FIELDS,
        "returnGeometry": "false",
        "f": "json",
    }
    url = layer_query_url() + "?" + urllib.parse.urlencode(params)
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(request, timeout=30) as response:
        payload = json.loads(response.read().decode("utf-8"))
    return payload.get("features", [])


def main() -> None:
    rows: list[dict[str, str]] = []
    for index, feature in enumerate(fetch_features(), start=1):
        attributes = feature.get("attributes", {})
        rows.append(
            {
                "sample_id": f"CAN-FILTERED-ROUTE-SAMPLE-{index:03d}",
                "source_id": "CAN-SRC-001",
                "extraction_window": "object_ids_17_18_19_20_21",
                "query_ref": "bounded-route-identifier-window",
                "object_id": str(attributes.get("OBJECTID", "")),
                "route_number_1": str(attributes.get("rtnumber1", "")),
                "route_name_1": str(attributes.get("rtename1", "")),
                "road_class": str(attributes.get("roadclass", "")),
                "type_code": str(attributes.get("type_code", "")),
                "nhs_description": str(attributes.get("desc_en", "")),
                "sample_method": "esri-rest-objectid-query-no-geometry-limit-5",
                "geometry_status": "not-requested",
                "evidence_acceptance_status": "not-accepted",
                "blocked_claims": BLOCKED,
                "next_action": "use filtered sample to refine parser extraction before fixture replacement",
            }
        )

    write_csv(OUTPUT, rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
