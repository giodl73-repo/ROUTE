#!/usr/bin/env python3
"""Gate bounded Japan source-content sample rows."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-japan-source-content-sample-001.csv"

FIELDS = [
    "sample_id",
    "source_id",
    "source_family",
    "source_url_or_status",
    "sample_basis",
    "content_summary",
    "route_or_dataset_hint",
    "source_owner",
    "source_date",
    "sample_status",
    "evidence_label",
    "blocked_claims",
    "next_action",
]
REQUIRED_SOURCES = {
    "JPN-SRC-001",
    "JPN-SRC-002",
    "JPN-SRC-003",
    "JPN-SRC-004",
    "JPN-SRC-005",
    "JPN-SRC-006",
}
REQUIRED_BLOCKS = {
    "source_row_validation",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "terminal_performance",
    "node_completeness",
    "road_access_proof",
    "throughput_proof",
    "guaranteed_sla",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
    "internal_adapter_proof",
}


def main() -> int:
    with SAMPLE.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("Japan source-content sample columns do not match contract")
    if {row["source_id"] for row in rows} != REQUIRED_SOURCES:
        failures.append("Japan source-content sample must cover the six official URL source rows")
    if not any(row["route_or_dataset_hint"] == "usable_geospatial_metadata_source_needed" for row in rows):
        failures.append("Japan sample must preserve the GSI metadata source-needed blocker")
    if not any(row["route_or_dataset_hint"] == "traffic_census_table_selection_needed" for row in rows):
        failures.append("Japan sample must include traffic census table-selection warning")
    if not any("port-node" in row["next_action"] or "port-node" in row["content_summary"] for row in rows):
        failures.append("Japan sample must preserve port-node promotion warning")
    for row in rows:
        if row["evidence_label"] not in {"source-candidate", "source-needed"}:
            failures.append(f"{row['sample_id']} has unsupported evidence label")
        if row["source_id"] == "JPN-SRC-004" and row["evidence_label"] != "source-needed":
            failures.append("JPN-SRC-004 must remain source-needed")
        if row["source_id"] != "JPN-SRC-004" and row["evidence_label"] != "source-candidate":
            failures.append(f"{row['sample_id']} must remain a source candidate only")
        if "evidence not accepted" not in row["sample_basis"]:
            failures.append(f"{row['sample_id']} must preserve evidence-not-accepted basis")
        if "validated" in row["sample_status"]:
            failures.append(f"{row['sample_id']} must not validate source content")
        if "before" not in row["next_action"]:
            failures.append(f"{row['sample_id']} next action must preserve before dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['sample_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("Japan source-content sample gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Japan source-content sample gate: PASS")
    print("  checked source coverage, GSI blocker, port-node warning, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
