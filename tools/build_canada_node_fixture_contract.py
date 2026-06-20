#!/usr/bin/env python3
"""Build the Canada node fixture replacement contract."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-canada-node-fixture-contract-001.csv"

FIELDS = [
    "contract_id",
    "replacement_target",
    "replacement_source",
    "required_rows",
    "contract_decision",
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
    rows = [
        {
            "contract_id": "CAN-NODE-CONTRACT-001",
            "replacement_target": "data/canada_source_node_candidates.csv",
            "replacement_source": "data/international-canada-node-source-selection-001.csv",
            "required_rows": "CAN-PORT-VANCOUVER;CAN-PORT-MONTREAL;CAN-PORT-HALIFAX",
            "contract_decision": "node_fixture_contract_ready_for_internal_closeout",
            "allowed_use": "internal adapter node-catalog fixture rows only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "run node role review before node fixture replacement closeout",
        }
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
