#!/usr/bin/env python3
"""Build EU Rhine-Alpine port-node role review."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-port-node-role-review-001.csv"

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
    "official_network;official_corridor_designation;member_state_approval;"
    "route_designation;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;"
    "endorsement;validation;external_validation;public_readiness;"
    "external_readiness;fixture_replacement;internal_adapter_proof"
)


def main() -> None:
    roles = [
        ("Scope Keeper", "Do sampled port records stay internal and non-public?"),
        ("Citation Auditor", "Do rows carry source package, source ID, and package trace?"),
        ("Schematic Cartographer", "Could point-layer joins be mistaken for map or topology proof?"),
        ("Traffic Engineer", "Do rows avoid terminal performance, road access, throughput, and node completeness claims?"),
        ("V&V", "Can a later gate mechanically distinguish attribute samples from replacement rows?"),
    ]
    rows = [
        {
            "review_id": f"EUR-PORT-NODE-ROLE-{index:03d}",
            "role_lane": role,
            "review_question": question,
            "input_artifacts": "data/international-eu-rhine-alpine-port-node-record-sample-001.csv;data/international-eu-rhine-alpine-port-node-field-mapping-001.csv;data/international-eu-rhine-alpine-port-package-manifest-001.csv",
            "result": "pass_with_holds",
            "allowed_use": "internal node-candidate planning review only",
            "blocked_claims": BLOCKED,
            "next_action": "write source-row validation before any node fixture replacement contract",
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
