#!/usr/bin/env python3
"""Gate EU Rhine-Alpine GISCO Ports 2013 package manifest ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-port-package-manifest-001.csv"

FIELDS = [
    "manifest_id",
    "package_id",
    "package_format",
    "package_url",
    "bytes_read",
    "manifest_result",
    "key_entries",
    "dbf_fields",
    "geometry_files_present",
    "evidence_acceptance_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_BLOCKS = {
    "fixture_replacement",
    "internal_adapter_proof",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "terminal_performance",
    "node_completeness",
    "road_access_proof",
    "guaranteed_sla",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("EU port package manifest columns do not match contract")
    if len(rows) != 2:
        failures.append("EU port package manifest must have two rows")
    shp_rows = [row for row in rows if row["package_format"] == "shp_zip"]
    if len(shp_rows) != 1:
        failures.append("EU port package manifest must include one SHP row")
    else:
        shp = shp_rows[0]
        for required in ["PORT_PT_2013.dbf", "PORT_PT_2013.shp", "PORT_PT_2013.shx", "PORT_PT_2013.prj"]:
            if required not in shp["key_entries"]:
                failures.append(f"SHP manifest missing {required}")
        for required_field in ["PORT_ID:C:5", "DATA_SRC_C:N:10", "PORT_COOR_:C:1"]:
            if required_field not in shp["dbf_fields"]:
                failures.append(f"SHP DBF header missing {required_field}")
        if shp["manifest_result"] != "point_layer_manifest_and_dbf_header_read":
            failures.append("SHP row must preserve point-layer manifest result")
    for row in rows:
        if int(row["bytes_read"]) <= 0:
            failures.append(f"{row['manifest_id']} did not read package bytes")
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['manifest_id']} accepts evidence prematurely")
        if "geometry not read or accepted" not in row["allowed_use"]:
            failures.append(f"{row['manifest_id']} must block geometry acceptance")
        if "before node fixture replacement" not in row["next_action"]:
            failures.append(f"{row['manifest_id']} must preserve node replacement dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['manifest_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine port package manifest gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine port package manifest gate: PASS")
    print("  checked package manifests, DBF fields, geometry hold, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
