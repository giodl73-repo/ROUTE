#!/usr/bin/env python3
"""Build the Canada adapter promotion preflight ledger.

This preflight records which Canada parser gates are closed and why parsed
adapter promotion remains held. It does not create an adapter, accept geometry,
or support map, official-network, service, SLA, ROI, or approval claims.
"""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-canada-adapter-promotion-preflight-001.csv"

FIELDS = [
    "preflight_id",
    "promotion_surface",
    "closed_inputs",
    "current_decision",
    "blocker",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

BLOCKED_CLAIMS = (
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "official_network;route_designation;agency_approval;construction_ready;"
    "guaranteed_sla;roi;eligibility;compliance;endorsement;validation;"
    "public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "preflight_id": "CAN-ADAPTER-PROMO-001",
            "promotion_surface": "link_candidate_fixture",
            "closed_inputs": "source_row_validation;geometry_policy;replacement_contract;link_fixture_replacement",
            "current_decision": "internal_link_fixture_ready",
            "blocker": "parsed_adapter_not_promoted",
            "allowed_use": "internal parser link-candidate fixture rows only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "define parsed adapter output contract before any adapter promotion",
        },
        {
            "preflight_id": "CAN-ADAPTER-PROMO-002",
            "promotion_surface": "geometry_topology",
            "closed_inputs": "geometry_policy",
            "current_decision": "hold",
            "blocker": "geometry_not_accepted;topology_not_validated",
            "allowed_use": "none",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "open a separate geometry intake fixture only if geometry becomes selected work",
        },
        {
            "preflight_id": "CAN-ADAPTER-PROMO-003",
            "promotion_surface": "need_node_target_tables",
            "closed_inputs": "dry_run_gate;node_fixture_replacement;target_posture",
            "current_decision": "hold",
            "blocker": "need_rows_bounded_context_only;service_targets_remain_held_assumptions",
            "allowed_use": "none",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "use internal proof closeout only; do not promote SLA official or external validation claims",
        },
        {
            "preflight_id": "CAN-ADAPTER-PROMO-004",
            "promotion_surface": "authority_operational_public_use",
            "closed_inputs": "none",
            "current_decision": "blocked",
            "blocker": "no_agency_review_no_operational_proof_no_external_validation",
            "allowed_use": "none",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "prepare jurisdiction-specific source packs and role review before public or external claims",
        },
        {
            "preflight_id": "CAN-ADAPTER-PROMO-005",
            "promotion_surface": "promotion_decision",
            "closed_inputs": "link_fixture_replaced",
            "current_decision": "parsed_adapter_promotion_held",
            "blocker": "adapter_contract_not_closed",
            "allowed_use": "none",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "write adapter promotion contract only after geometry need node target and authority blockers are selected or explicitly waived",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
