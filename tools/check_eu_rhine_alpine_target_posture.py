#!/usr/bin/env python3
"""Gate EU Rhine-Alpine target posture closeout."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TARGETS = ROOT / "data" / "eu_rhine_alpine_service_target_candidates.csv"
POSTURE = ROOT / "data" / "international-eu-rhine-alpine-target-posture-001.csv"

FIELDS = [
    "posture_id",
    "target_table",
    "row_count",
    "target_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_BLOCKS = {
    "official_network",
    "official_corridor_designation",
    "member_state_approval",
    "route_designation",
    "guaranteed_sla",
    "travel_time_proof",
    "delivery_commitment",
    "numeric_roi",
    "roi",
    "construction_ready",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    _, target_rows = read_csv(TARGETS)
    fields, posture_rows = read_csv(POSTURE)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("EU target posture columns do not match required contract")
    if len(posture_rows) != 1:
        failures.append("EU target posture must contain one closeout row")
    for row in target_rows:
        if row["evidence_label"] != "held":
            failures.append(f"{row['target_gap_id']} is not held")
        if row["assumption_label"] != "planning_assumption_only":
            failures.append(f"{row['target_gap_id']} is not planning_assumption_only")
        if "adopted service target source" not in row["needed_source"]:
            failures.append(f"{row['target_gap_id']} does not preserve source dependency")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"{row['target_gap_id']} missing blocked claims: {sorted(missing_blocks)}")
    for row in posture_rows:
        if row["target_status"] != "held_planning_assumptions_accepted_for_internal_proof":
            failures.append("EU target posture did not preserve held assumptions")
        if row["allowed_use"] != "internal adapter proof with explicit target holds only":
            failures.append("EU target posture allowed use is too broad")
        if int(row["row_count"]) != len(target_rows):
            failures.append("EU target posture row count does not match target table")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"EU target posture missing blocked claims: {sorted(missing_blocks)}")
        if "until target source and numeracy review close" not in row["next_action"]:
            failures.append("EU target posture next action must preserve source/numeracy dependency")

    if failures:
        print("EU Rhine-Alpine target posture gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("EU Rhine-Alpine target posture gate: PASS")
    print("  checked held target assumptions, internal allowed use, and SLA/ROI claim blocks")
    return 0


if __name__ == "__main__":
    sys.exit(main())
