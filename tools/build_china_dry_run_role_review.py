#!/usr/bin/env python3
"""Build China role review for parser dry-run outputs."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-china-dry-run-role-review-001.csv"

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
    "official_network;official_corridor_designation;policy_alignment;"
    "route_designation;source_row_validation;fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;"
    "travel_time_proof;delivery_commitment;numeric_roi;roi;"
    "eligibility;compliance;endorsement;validation;external_validation;"
    "public_readiness;external_readiness;internal_adapter_proof"
)


def main() -> None:
    roles = [
        ("Scope Keeper", "Do China rows remain internal dry-run rows rather than source-row validation or fixture replacement?"),
        ("Citation Auditor", "Do China rows carry source, owner, date, access note, evidence label, and blocked-claim custody?"),
        ("Schematic Cartographer", "Could standards context or hierarchy carry-forward be mistaken for accepted geometry, topology, or map proof?"),
        ("Traffic Engineer", "Do China rows avoid terminal performance, road access, throughput, SLA, and operational claims?"),
        ("V&V", "Can later gates mechanically distinguish context-only, source-candidate, heuristic-held, held, and carry-forward rows?"),
    ]
    rows = [
        {
            "review_id": f"CHN-DRYRUN-ROLE-{index:03d}",
            "role_lane": role,
            "review_question": question,
            "input_artifacts": "data/china_source_link_candidates.csv;data/china_source_need_candidates.csv;data/china_source_node_candidates.csv;data/china_service_target_candidates.csv;data/china_adapter_evidence_labels.csv;data/china_adapter_review_backlog.csv",
            "result": "pass_with_holds",
            "allowed_use": "internal parser dry-run planning review only",
            "blocked_claims": BLOCKED,
            "next_action": "create source-row validation or fixture-replacement blocker before any fixture replacement contract",
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
