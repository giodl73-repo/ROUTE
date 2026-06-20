#!/usr/bin/env python3
"""Build India role review for bounded row-validation outputs."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-india-role-review-001.csv"

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
    "official_network;official_corridor_designation;national_approval;"
    "state_approval;route_designation;source_row_validation;"
    "fixture_replacement;parsed_adapter;geometry_acceptance;topology_proof;"
    "map_overlay;terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;"
    "endorsement;validation;external_validation;public_readiness;"
    "external_readiness;internal_adapter_proof"
)


def main() -> None:
    roles = [
        ("Scope Keeper", "Do India rows remain internal dry-run rows rather than fixture replacement?"),
        ("Citation Auditor", "Do India rows carry source, inventory, evidence label, and blocked-claim custody?"),
        ("Schematic Cartographer", "Could metadata or heuristic rows be mistaken for accepted geometry or map proof?"),
        ("Traffic Engineer", "Do rows avoid terminal performance, road access, throughput, SLA, and operational claims?"),
        ("V&V", "Can later gates mechanically distinguish source candidates, heuristic holds, and held target rows?"),
    ]
    rows = [
        {
            "review_id": f"IND-ROLE-{index:03d}",
            "role_lane": role,
            "review_question": question,
            "input_artifacts": "data/international-india-source-row-validation-001.csv;data/india_source_link_candidates.csv;data/india_source_node_candidates.csv;data/india_adapter_evidence_labels.csv",
            "result": "pass_with_holds",
            "allowed_use": "internal row-validation planning review only",
            "blocked_claims": BLOCKED,
            "next_action": "create geometry policy and fixture-replacement blocker before any fixture replacement contract",
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
