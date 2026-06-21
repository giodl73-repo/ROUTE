#!/usr/bin/env python3
"""Build Japan role review for content-row validation outputs."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-japan-content-row-role-review-001.csv"

FIELDS = [
    "review_id",
    "role_lane",
    "review_question",
    "input_artifacts",
    "result",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "official_network;official_corridor_designation;ministry_approval;"
    "route_designation;source_row_validation;fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "disaster_readiness;terminal_performance;node_completeness;"
    "road_access_proof;throughput_proof;construction_ready;"
    "guaranteed_sla;travel_time_proof;delivery_commitment;numeric_roi;"
    "roi;eligibility;compliance;endorsement;validation;"
    "external_validation;public_readiness;external_readiness;"
    "internal_adapter_proof"
)


def main() -> None:
    roles = [
        ("Scope Keeper", "Do matched Japan content rows remain internal parser-inspection rows?"),
        ("Citation Auditor", "Do Japan extraction candidates trace to sampled content or the GSI blocker without claiming accepted source rows?"),
        ("Schematic Cartographer", "Could GSI blockers, port context, or hierarchy carry-forward be mistaken for geometry, topology, or map proof?"),
        ("Traffic Engineer", "Do Japan content rows avoid disaster-readiness, terminal performance, road access, throughput, SLA, and operational claims?"),
        ("V&V", "Can later gates distinguish content-row matching from source-row validation and fixture replacement?"),
    ]
    rows = [
        {
            "review_id": f"JPN-CONTENT-ROLE-{index:03d}",
            "role_lane": role,
            "review_question": question,
            "input_artifacts": "data/international-japan-source-content-row-validation-001.csv;data/international-japan-parser-extraction-candidates-001.csv;data/international-japan-source-content-sample-001.csv",
            "result": "pass_with_holds",
            "allowed_use": "internal content-row planning review only",
            "blocked_claims": BLOCKED,
            "next_action": "write source-row validation or renewed fixture blocker before any fixture replacement contract",
        }
        for index, (role, question) in enumerate(roles, start=1)
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
