#!/usr/bin/env python3
"""Gate bounded China source-content sample rows."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-china-source-content-sample-001.csv"

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
REQUIRED_SOURCES = {"CHN-SRC-001", "CHN-SRC-002", "CHN-SRC-003", "CHN-SRC-004", "CHN-SRC-005"}
REQUIRED_BLOCKS = {
    "official_corridor_designation",
    "policy_alignment",
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
        failures.append("China source-content sample columns do not match contract")
    if {row["source_id"] for row in rows} != REQUIRED_SOURCES:
        failures.append("China source-content sample must cover the five URL source rows")
    if not any(row["route_or_dataset_hint"] == "planning_context_not_policy_alignment" for row in rows):
        failures.append("China sample must preserve policy-alignment boundary")
    if not any(row["route_or_dataset_hint"] == "transport_statistics_table_selection_needed" for row in rows):
        failures.append("China sample must include transport-statistics table-selection warning")
    if not any(row["route_or_dataset_hint"] == "standards_context_not_design_geometry" for row in rows):
        failures.append("China sample must preserve standards-not-geometry boundary")
    if not any(row["route_or_dataset_hint"] == "port_waterway_table_inventory_needed" for row in rows):
        failures.append("China sample must include port/waterway table inventory warning")
    for row in rows:
        if row["evidence_label"] not in {"source-candidate", "context-only"}:
            failures.append(f"{row['sample_id']} has unsupported evidence label")
        if row["source_id"] == "CHN-SRC-004" and row["evidence_label"] != "context-only":
            failures.append("CHN-SRC-004 must remain context-only")
        if row["source_id"] != "CHN-SRC-004" and row["evidence_label"] != "source-candidate":
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
        print("China source-content sample gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("China source-content sample gate: PASS")
    print("  checked source coverage, policy boundary, standards boundary, table warnings, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
