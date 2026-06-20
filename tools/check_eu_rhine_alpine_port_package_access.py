#!/usr/bin/env python3
"""Gate EU Rhine-Alpine GISCO Ports 2013 package-access metadata."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-port-package-access-001.csv"

FIELDS = [
    "package_id",
    "metadata_probe_id",
    "source_id",
    "package_format",
    "package_url",
    "http_method",
    "http_status",
    "content_type",
    "content_length_bytes",
    "access_result",
    "evidence_acceptance_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_FORMATS = {"gdb_zip", "shp_zip"}
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
        failures.append("EU port package access columns do not match contract")
    if len(rows) != 2:
        failures.append("EU port package access must have two package rows")
    if {row["package_format"] for row in rows} != REQUIRED_FORMATS:
        failures.append("EU port package access must cover GDB and SHP zip packages")
    for row in rows:
        if row["metadata_probe_id"] != "EUR-METADATA-PROBE-003":
            failures.append(f"{row['package_id']} must trace to port metadata probe")
        if row["source_id"] != "EUR-SRC-003":
            failures.append(f"{row['package_id']} must trace to GISCO source")
        if row["http_method"] != "HEAD":
            failures.append(f"{row['package_id']} must use HEAD metadata probe")
        if row["http_status"] != "200":
            failures.append(f"{row['package_id']} not reachable with HTTP 200")
        if row["content_type"] != "application/zip":
            failures.append(f"{row['package_id']} must remain zip package metadata")
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['package_id']} accepts evidence prematurely")
        if "no download" not in row["allowed_use"]:
            failures.append(f"{row['package_id']} must block download/use promotion")
        if "before node fixture replacement" not in row["next_action"]:
            failures.append(f"{row['package_id']} must preserve node replacement dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['package_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine port package access gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine port package access gate: PASS")
    print("  checked package URLs, HEAD metadata, no-download posture, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
