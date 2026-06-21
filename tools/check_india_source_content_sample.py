#!/usr/bin/env python3
"""Gate bounded India source-content sample rows."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-india-source-content-sample-001.csv"

FIELDS = [
    "sample_id",
    "source_id",
    "source_family",
    "source_url",
    "source_line_ref",
    "content_summary",
    "route_or_dataset_hint",
    "source_owner",
    "source_date",
    "sample_status",
    "evidence_label",
    "blocked_claims",
    "next_action",
]
REQUIRED_SOURCES = {"IND-SRC-001", "IND-SRC-002", "IND-SRC-003", "IND-SRC-004"}
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
        failures.append("India source-content sample columns do not match contract")
    if {row["source_id"] for row in rows} != REQUIRED_SOURCES:
        failures.append("India source-content sample must cover the four content-bearing source rows")
    if not any(row["route_or_dataset_hint"] == "document_inventory_needed_before_highway_rows" for row in rows):
        failures.append("India sample must preserve highway document-inventory warning")
    if not any(row["route_or_dataset_hint"] == "major_port_node_candidates_not_validated" for row in rows):
        failures.append("India sample must include port-node candidate context")
    if not any("2024-25" in row["content_summary"] for row in rows):
        failures.append("India sample must preserve current port-statistics publication lead")
    for row in rows:
        if row["evidence_label"] != "source-candidate":
            failures.append(f"{row['sample_id']} has unsupported evidence label")
        if not row["source_url"].startswith("https://"):
            failures.append(f"{row['sample_id']} missing source URL")
        if "before" not in row["next_action"]:
            failures.append(f"{row['sample_id']} next action must preserve before dependency")
        if "validated" in row["sample_status"]:
            failures.append(f"{row['sample_id']} must not validate source content")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['sample_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("India source-content sample gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("India source-content sample gate: PASS")
    print("  checked source coverage, highway inventory warning, port leads, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
