#!/usr/bin/env python3
"""Build EU Rhine-Alpine port node fixture replacement closeout."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-port-node-fixture-closeout-001.csv"
NODES = ROOT / "data" / "eu_rhine_alpine_source_node_candidates.csv"

FIELDS = [
    "closeout_id",
    "replacement_target",
    "replacement_source",
    "row_count",
    "role_review_status",
    "source_row_validation_status",
    "geometry_contract",
    "replacement_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

BLOCKED_CLAIMS = (
    "official_network;official_corridor_designation;member_state_approval;"
    "route_designation;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;throughput_proof;"
    "construction_ready;guaranteed_sla;travel_time_proof;delivery_commitment;"
    "numeric_roi;roi;eligibility;compliance;endorsement;validation;"
    "external_validation;public_readiness;external_readiness;internal_adapter_proof"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> None:
    node_rows = read_csv(NODES)
    rows = [
        {
            "closeout_id": "EUR-PORT-NODE-FIXTURE-CLOSEOUT-001",
            "replacement_target": "data/eu_rhine_alpine_source_node_candidates.csv",
            "replacement_source": "data/international-eu-rhine-alpine-port-node-source-row-validation-001.csv",
            "row_count": str(len(node_rows)),
            "role_review_status": "pass_with_holds",
            "source_row_validation_status": "candidate_attribute_rows_validated_geometry_held",
            "geometry_contract": "no_geometry_attribute_rows_only",
            "replacement_status": "internal_node_fixture_replaced_no_geometry",
            "allowed_use": "internal adapter node-candidate fixture rows only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "update EU parity gap and target posture before any internal adapter proof",
        }
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
