#!/usr/bin/env python3
"""Gate China parser extraction candidates before fixture replacement."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CANDIDATES = ROOT / "data" / "international-china-parser-extraction-candidates-001.csv"

FIELDS = [
    "candidate_id",
    "target_table",
    "source_id",
    "source_family",
    "candidate_key",
    "candidate_label",
    "candidate_class",
    "geometry_ref",
    "source_owner",
    "source_date",
    "access_note",
    "evidence_label",
    "candidate_status",
    "blocked_claims",
    "next_action",
]
REQUIRED_SOURCES = {"CHN-SRC-001", "CHN-SRC-002", "CHN-SRC-003", "CHN-SRC-004", "CHN-SRC-005"}
REQUIRED_TARGETS = {
    "china_source_link_candidates",
    "china_source_need_candidates",
    "china_source_node_candidates",
}
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
    with CANDIDATES.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("China parser extraction candidate columns do not match contract")
    if len(rows) != 5:
        failures.append("China parser extraction candidate table must have five bounded rows")
    if {row["source_id"] for row in rows} != REQUIRED_SOURCES:
        failures.append("China extraction candidates must cover the five sampled source contexts")
    if {row["target_table"] for row in rows} != REQUIRED_TARGETS:
        failures.append("China extraction candidates must cover link, need, and node target tables")
    if not any(row["source_id"] == "CHN-SRC-004" and row["evidence_label"] == "context-only" for row in rows):
        failures.append("China extraction candidates must keep the standards link row context-only")
    if sum(1 for row in rows if row["target_table"] == "china_source_need_candidates") != 3:
        failures.append("China extraction candidates must include three need/context candidates")
    if not any(row["candidate_class"] == "planning_context_not_policy_alignment" for row in rows):
        failures.append("China extraction candidates must preserve planning-not-policy-alignment boundary")
    if not any(row["candidate_class"] == "port_waterway_context_not_node_row" for row in rows):
        failures.append("China extraction candidates must preserve port/waterway node boundary")
    for row in rows:
        if row["source_id"] == "CHN-SRC-004":
            if row["candidate_status"] != "source_content_extraction_context_only_not_promoted":
                failures.append("CHN-SRC-004 standards candidate must remain context-only and not promoted")
        elif row["evidence_label"] != "source-candidate":
            failures.append(f"{row['candidate_id']} has wrong evidence label")
        if not row["candidate_status"].endswith("_not_promoted"):
            failures.append(f"{row['candidate_id']} promotes candidate status")
        if not row["geometry_ref"].startswith("not_requested:"):
            failures.append(f"{row['candidate_id']} accepts geometry")
        if "before" not in row["next_action"]:
            failures.append(f"{row['candidate_id']} next action must preserve before dependency")
        if "validated" in row["candidate_class"]:
            failures.append(f"{row['candidate_id']} implies validation")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['candidate_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("China parser extraction candidate gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("China parser extraction candidate gate: PASS")
    print("  checked target coverage, no-geometry posture, context-only link row, and not-promoted status")
    return 0


if __name__ == "__main__":
    sys.exit(main())
