#!/usr/bin/env python3
"""Build bounded EU Rhine-Alpine GISCO Ports 2013 node record sample."""

from __future__ import annotations

import csv
import io
import urllib.request
import zipfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-port-node-record-sample-001.csv"
URL = "https://ec.europa.eu/eurostat/cache/GISCO/geodatafiles/PORT_2013_SH.zip"
USER_AGENT = "ROUTE-EU-GISCO-port-record-sample/0.1 evidence-not-accepted"
TIMEOUT_SECONDS = 30

FIELDS = [
    "sample_id",
    "source_id",
    "package_id",
    "port_id",
    "port_name",
    "country_code",
    "nuts_code",
    "ten_code",
    "port_hierarchy",
    "point_layer_join",
    "sample_reason",
    "evidence_acceptance_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

TARGETS = [
    ("NLRTM", "Rotterdam endpoint context"),
    ("BEANR", "Antwerp endpoint context"),
    ("ITGOA", "Genoa endpoint context via Genova source spelling"),
    ("CHBSL", "Alpine/interior gateway context"),
    ("DEDUI", "Rhine industrial node context"),
]

BLOCKED = (
    "official_network;official_corridor_designation;member_state_approval;"
    "route_designation;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;"
    "endorsement;validation;external_validation;public_readiness;"
    "external_readiness;fixture_replacement;internal_adapter_proof"
)


def dbf_records(dbf: bytes) -> list[dict[str, str]]:
    rec_count = int.from_bytes(dbf[4:8], "little")
    header_len = int.from_bytes(dbf[8:10], "little")
    rec_len = int.from_bytes(dbf[10:12], "little")
    fields: list[tuple[str, int]] = []
    pos = 32
    while pos < header_len - 1:
        desc = dbf[pos : pos + 32]
        if desc[0] == 0x0D:
            break
        name = desc[:11].split(b"\x00", 1)[0].decode("ascii", "replace")
        fields.append((name, desc[16]))
        pos += 32
    rows: list[dict[str, str]] = []
    for index in range(rec_count):
        offset = header_len + index * rec_len
        rec = dbf[offset : offset + rec_len]
        if not rec or rec[0:1] == b"*":
            continue
        cursor = 1
        row: dict[str, str] = {}
        for name, length in fields:
            raw = rec[cursor : cursor + length]
            cursor += length
            row[name] = raw.decode("latin1", "replace").strip()
        rows.append(row)
    return rows


def main() -> None:
    request = urllib.request.Request(URL, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(request, timeout=TIMEOUT_SECONDS) as response:
        payload = response.read()
    with zipfile.ZipFile(io.BytesIO(payload)) as zf:
        attrs = {row["PORT_ID"]: row for row in dbf_records(zf.read("PORT_2013_SH/Data/PORT_AT_2013.dbf"))}
        points = {row["PORT_ID"]: row for row in dbf_records(zf.read("PORT_2013_SH/Data/PORT_PT_2013.dbf"))}
    rows: list[dict[str, str]] = []
    for index, (port_id, reason) in enumerate(TARGETS, start=1):
        attr = attrs[port_id]
        rows.append(
            {
                "sample_id": f"EUR-PORT-RECORD-{index:03d}",
                "source_id": "EUR-SRC-003",
                "package_id": "EUR-PORT-PACKAGE-002",
                "port_id": port_id,
                "port_name": attr["NAME_ASCI"] or attr["PORT_NAME"],
                "country_code": attr["CNTR_CODE"],
                "nuts_code": attr["NUTS_CODE"],
                "ten_code": attr["TEN_CODE"],
                "port_hierarchy": attr["PORT_HIER_"],
                "point_layer_join": "point_record_present_geometry_not_read"
                if port_id in points
                else "point_record_missing",
                "sample_reason": reason,
                "evidence_acceptance_status": "not-accepted",
                "allowed_use": "bounded attribute sample only; geometry and node replacement held",
                "blocked_claims": BLOCKED,
                "next_action": "run node role review and source-row validation before node fixture replacement",
            }
        )
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
