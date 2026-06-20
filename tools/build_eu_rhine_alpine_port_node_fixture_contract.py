#!/usr/bin/env python3
"""Build EU Rhine-Alpine port-node fixture contract."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-port-node-fixture-contract-001.csv"

FIELDS = [
    "contract_id",
    "candidate_source",
    "replacement_target",
    "required_rows",
    "geometry_contract",
    "contract_decision",
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
    rows = [
        {
            "contract_id": "EUR-PORT-NODE-CONTRACT-001",
            "candidate_source": "data/international-eu-rhine-alpine-port-node-source-row-validation-001.csv",
            "replacement_target": "data/eu_rhine_alpine_source_node_candidates.csv",
            "required_rows": "NLRTM;BEANR;ITGOA;CHBSL;DEDUI",
            "geometry_contract": "no_geometry_attribute_rows_only",
            "contract_decision": "contract_ready_for_internal_closeout_not_replacement",
            "allowed_use": "internal adapter node-candidate contract only",
            "blocked_claims": BLOCKED,
            "next_action": "write node fixture closeout before any internal adapter proof",
        },
        {
            "contract_id": "EUR-PORT-NODE-CONTRACT-002",
            "candidate_source": "data/international-eu-rhine-alpine-port-node-source-row-validation-001.csv",
            "replacement_target": "maps/international or public media surfaces",
            "required_rows": "none",
            "geometry_contract": "geometry_required_before_map_or_public_use",
            "contract_decision": "contract_blocks_map_public_and_external_use",
            "allowed_use": "none",
            "blocked_claims": BLOCKED,
            "next_action": "run separate geometry topology and publication review before map or public use",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
