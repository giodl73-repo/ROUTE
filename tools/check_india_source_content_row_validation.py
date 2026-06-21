#!/usr/bin/env python3
"""Gate India source-content row validation before fixture replacement."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-india-source-content-row-validation-001.csv"

FIELDS = [
    "validation_id",
    "candidate_id",
    "target_table",
    "source_id",
    "sample_id",
    "candidate_status",
    "sample_status",
    "validation_result",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_TARGETS = {
    "india_source_link_candidates",
    "india_source_need_candidates",
    "india_source_node_candidates",
}
REQUIRED_BLOCKS = {
    "source_row_validation",
    "fixture_replacement",
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
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("India source-content row validation columns do not match contract")
    if len(rows) != 5:
        failures.append("India source-content row validation must cover five extraction candidates")
    if {row["target_table"] for row in rows} != REQUIRED_TARGETS:
        failures.append("India source-content row validation must cover link, node, and need target tables")
    for row in rows:
        if row["validation_result"] != "source_content_candidate_row_matched_not_source_row_validated":
            failures.append(f"{row['candidate_id']} has unsupported validation result")
        if row["allowed_use"] != "internal_parser_inspection_only":
            failures.append(f"{row['candidate_id']} has unsupported allowed use")
        if row["candidate_status"] != "source_content_extraction_candidate_not_promoted":
            failures.append(f"{row['candidate_id']} promotes candidate status")
        if "before fixture replacement" not in row["next_action"]:
            failures.append(f"{row['candidate_id']} must preserve fixture replacement dependency")
        if "not_source_row_validated" not in row["validation_result"]:
            failures.append(f"{row['candidate_id']} must not claim source-row validation")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['candidate_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("India source-content row validation gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("India source-content row validation gate: PASS")
    print("  checked row coverage, not-promoted status, and source-row validation boundary")
    return 0


if __name__ == "__main__":
    sys.exit(main())
