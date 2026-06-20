#!/usr/bin/env python3
"""Build Canada node fixture replacement role review."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-canada-node-replacement-role-review-001.csv"

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

BLOCKED_CLAIMS = (
    "port_endorsement;terminal_performance;node_completeness;throughput_proof;"
    "road_access_proof;construction_ready;guaranteed_sla;roi;compliance;"
    "endorsement;validation;public_readiness;external_readiness"
)


def main() -> None:
    roles = [
        ("Scope Keeper", "Does node replacement stay internal and non-public?"),
        ("Citation Auditor", "Do node rows carry selected source owner, date, and URL?"),
        ("Schematic Cartographer", "Could source-selected nodes be mistaken for map or topology proof?"),
        ("Traffic Engineer", "Do rows avoid terminal performance, access adequacy, and throughput claims?"),
        ("V&V", "Can the parser dry-run gate check node evidence labels mechanically?"),
    ]
    rows = [
        {
            "review_id": f"CAN-NODE-ROLE-{index:03d}",
            "role_lane": role,
            "review_question": question,
            "input_artifacts": "data/international-canada-node-source-selection-001.csv;data/international-canada-node-source-probe-001.csv;data/international-canada-node-fixture-contract-001.csv",
            "result": "pass_with_holds",
            "allowed_use": "internal node fixture replacement review only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "write node fixture replacement closeout with all blocked claims carried forward",
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
