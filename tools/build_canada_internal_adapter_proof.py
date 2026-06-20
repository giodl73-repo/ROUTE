#!/usr/bin/env python3
"""Build Canada internal adapter proof closeout."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-canada-internal-adapter-proof-001.csv"

FIELDS = [
    "proof_id",
    "proof_surface",
    "input_artifacts",
    "proof_status",
    "allowed_claim",
    "blocked_claims",
    "next_action",
]

BLOCKED_CLAIMS = (
    "official_network;route_designation;geometry_acceptance;topology_proof;"
    "map_overlay;agency_approval;port_endorsement;terminal_performance;"
    "node_completeness;road_access_proof;throughput_proof;construction_ready;"
    "guaranteed_sla;travel_time_proof;delivery_commitment;roi;eligibility;"
    "compliance;endorsement;validation;public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "proof_id": "CAN-INTERNAL-PROOF-001",
            "proof_surface": "link_fixture",
            "input_artifacts": "data/canada_source_link_candidates.csv;data/international-canada-link-fixture-replacement-closeout-001.csv",
            "proof_status": "source_backed_internal_fixture_ready",
            "allowed_claim": "Canada link fixture is source-derived and internal-use only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep map topology official and external claims blocked",
        },
        {
            "proof_id": "CAN-INTERNAL-PROOF-002",
            "proof_surface": "node_fixture",
            "input_artifacts": "data/canada_source_node_candidates.csv;data/international-canada-node-fixture-replacement-closeout-001.csv",
            "proof_status": "source_custody_internal_fixture_ready",
            "allowed_claim": "Canada node fixture has selected public source-custody candidates for internal adapter proof",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "do not claim node completeness terminal performance road access proof or endorsement",
        },
        {
            "proof_id": "CAN-INTERNAL-PROOF-003",
            "proof_surface": "need_and_target_tables",
            "input_artifacts": "data/canada_source_need_candidates.csv;data/canada_service_target_candidates.csv;data/international-canada-target-posture-001.csv",
            "proof_status": "bounded_needs_ready_targets_held",
            "allowed_claim": "Canada needs are bounded source vocabulary and targets remain held planning assumptions",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep SLA travel-time delivery and ROI claims blocked",
        },
        {
            "proof_id": "CAN-INTERNAL-PROOF-004",
            "proof_surface": "adapter_proof_decision",
            "input_artifacts": "npm run check:canada;new Canada proof gates;docs/reviews/international-canada-internal-adapter-proof-001.md",
            "proof_status": "internal_adapter_proof_ready_external_validation_held",
            "allowed_claim": "Canada is internally proven as a source-backed evidence-gated adapter workflow",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "external validation requires named Canadian review outside this internal proof",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
