#!/usr/bin/env python3
"""Build Canada node fixture replacement closeout."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-canada-node-fixture-replacement-closeout-001.csv"
NODES = ROOT / "data" / "canada_source_node_candidates.csv"

FIELDS = [
    "closeout_id",
    "replacement_target",
    "replacement_source",
    "row_count",
    "role_review_status",
    "replacement_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

BLOCKED_CLAIMS = (
    "port_endorsement;terminal_performance;node_completeness;throughput_proof;"
    "road_access_proof;construction_ready;guaranteed_sla;roi;compliance;"
    "endorsement;validation;public_readiness;external_readiness"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> None:
    node_rows = read_csv(NODES)
    rows = [
        {
            "closeout_id": "CAN-NODE-FIXTURE-REPLACE-001",
            "replacement_target": "data/canada_source_node_candidates.csv",
            "replacement_source": "data/international-canada-node-source-selection-001.csv",
            "row_count": str(len(node_rows)),
            "role_review_status": "pass_with_holds",
            "replacement_status": "internal_node_fixture_replaced",
            "allowed_use": "internal adapter node-catalog fixture rows only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "use only in Canada internal adapter proof preflight with target and authority holds",
        }
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
