#!/usr/bin/env python3
"""Gate the bounded Iowa state-highway-system pilot ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "state-highway-system-pilot-iowa-001.csv"

FIELDS = [
    "pilot_id",
    "state",
    "surface",
    "input_artifacts",
    "candidate_network_role",
    "state_use_case",
    "evidence_status",
    "review_roles",
    "blocked_claims",
    "next_action",
]
REQUIRED_SURFACES = {
    "state_value_intake",
    "des_moines_i35_i80_operating_context",
    "statewide_trunk_and_connector_hypothesis",
    "scenario_game_fixture",
    "state_review_packet",
}
REQUIRED_BLOCKS = {
    "official_state_plan",
    "state_dot_endorsement",
    "fhwa_approval",
    "route_designation",
    "construction_ready",
    "funding_commitment",
    "guaranteed_sla",
    "numeric_roi",
    "roi",
    "environmental_clearance",
    "right_of_way_clearance",
    "maintenance_commitment",
    "source_row_validation",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "throughput_proof",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
}
PROHIBITED = {
    "official state plan",
    "iowa dot approved",
    "fhwa approved",
    "guaranteed sla",
    "construction ready",
    "proves roi",
    "validated by",
    "public ready",
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("Iowa state-highway pilot columns do not match contract")
    if len(rows) != 5:
        failures.append("Iowa state-highway pilot must have five bounded surfaces")
    if {row["surface"] for row in rows} != REQUIRED_SURFACES:
        failures.append("Iowa state-highway pilot surfaces do not match required set")
    if not all(row["state"] == "Iowa" for row in rows):
        failures.append("Iowa state-highway pilot must stay scoped to Iowa")
    if not any("State DOT Planner" in row["review_roles"] for row in rows):
        failures.append("Iowa state-highway pilot must include State DOT Planner review")
    if not any(row["evidence_status"] == "snapshot_window_candidate_promotion_blocked" for row in rows):
        failures.append("Iowa pilot must preserve Iowa 511 snapshot-window blocker")
    if not any(row["evidence_status"] == "scenario_fixture_publication_locked" for row in rows):
        failures.append("Iowa pilot must preserve game/scenario publication lock")
    for row in rows:
        if not row["input_artifacts"]:
            failures.append(f"{row['pilot_id']} missing input artifacts")
        if "before" not in row["next_action"]:
            failures.append(f"{row['pilot_id']} next action must preserve before dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['pilot_id']} missing blocked claims: {sorted(missing)}")
        text = " ".join(
            [
                row["candidate_network_role"],
                row["state_use_case"],
                row["evidence_status"],
                row["next_action"],
            ]
        ).lower()
        for phrase in PROHIBITED:
            if phrase in text:
                failures.append(f"{row['pilot_id']} promotes prohibited phrase: {phrase}")
    if failures:
        print("Iowa state-highway-system pilot gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Iowa state-highway-system pilot gate: PASS")
    print("  checked state scope, five surfaces, review roles, blockers, and prohibited claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
