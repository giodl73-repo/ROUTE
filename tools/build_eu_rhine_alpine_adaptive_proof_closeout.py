#!/usr/bin/env python3
"""Build EU Rhine-Alpine adaptive proof closeout ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-adaptive-proof-closeout-001.csv"

FIELDS = [
    "closeout_id",
    "proof_surface",
    "input_artifacts",
    "closeout_status",
    "allowed_claim",
    "blocked_claims",
    "next_action",
]

BLOCKED_CLAIMS = (
    "canada_depth_equivalence;internal_adapter_proof;official_network;"
    "official_corridor_designation;member_state_approval;route_designation;"
    "source_row_validation_for_road_links;link_fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "agency_review;named_contact;construction_ready;guaranteed_sla;"
    "travel_time_proof;delivery_commitment;numeric_roi;roi;eligibility;"
    "compliance;endorsement;validation;external_validation;public_readiness;"
    "external_readiness"
)


def main() -> None:
    rows = [
        {
            "closeout_id": "EUR-ADAPT-CLOSE-001",
            "proof_surface": "hierarchy_and_map_fixture",
            "input_artifacts": "docs/reviews/international-eu-rhine-alpine-hierarchy-iteration-001.md;maps/international/eu-rhine-alpine-candidate-hierarchy-v2.svg",
            "closeout_status": "adaptive_surface_complete_validation_held",
            "allowed_claim": "EU Rhine-Alpine reproduces the generic hierarchy and held-claim map fixture pattern for a second region",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep map as schematic fixture until source-backed geometry and validation exist",
        },
        {
            "closeout_id": "EUR-ADAPT-CLOSE-002",
            "proof_surface": "source_kernel_and_parser_contract",
            "input_artifacts": "docs/reviews/international-eu-rhine-alpine-adapter-source-pack-001.md;docs/reviews/international-eu-rhine-alpine-parser-preflight-001.md;docs/reviews/international-eu-rhine-alpine-parser-dry-run-001.md",
            "closeout_status": "proof_kernel_instantiated_parser_ready",
            "allowed_claim": "EU Rhine-Alpine instantiates the reusable proof kernel through source custody, parser contract, and deterministic dry-run outputs",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "do not promote parsed adapter until source-derived link replacement exists",
        },
        {
            "closeout_id": "EUR-ADAPT-CLOSE-003",
            "proof_surface": "node_fixture_branch",
            "input_artifacts": "docs/reviews/international-eu-rhine-alpine-port-node-fixture-closeout-001.md;data/eu_rhine_alpine_source_node_candidates.csv",
            "closeout_status": "internal_node_fixture_replaced_with_holds",
            "allowed_claim": "EU port-node branch replaced the internal no-geometry node fixture from bounded GISCO Ports 2013 attribute candidates",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep terminal performance road access throughput geometry and validation claims blocked",
        },
        {
            "closeout_id": "EUR-ADAPT-CLOSE-004",
            "proof_surface": "target_posture",
            "input_artifacts": "docs/reviews/international-eu-rhine-alpine-target-posture-001.md;data/international-eu-rhine-alpine-target-posture-001.csv",
            "closeout_status": "held_targets_accepted_for_future_internal_proof",
            "allowed_claim": "EU service targets are held planning assumptions that can be carried only into a future internal proof",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "do not claim adopted service targets SLA travel-time delivery commitment or ROI",
        },
        {
            "closeout_id": "EUR-ADAPT-CLOSE-005",
            "proof_surface": "road_link_blocker",
            "input_artifacts": "docs/reviews/international-eu-rhine-alpine-link-fixture-blocker-001.md;docs/reviews/international-eu-rhine-alpine-road-link-source-disposition-001.md;docs/reviews/international-eu-rhine-alpine-road-link-endpoint-request-001.md",
            "closeout_status": "link_fixture_blocked_by_missing_endpoint",
            "allowed_claim": "EU safely blocks link fixture replacement because no exact official road-link endpoint has been acquired",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "pursue endpoint request or alternative source selection before road-link source-row extraction",
        },
        {
            "closeout_id": "EUR-ADAPT-CLOSE-006",
            "proof_surface": "adaptive_proof_decision",
            "input_artifacts": "data/international-system-flexibility-proof-001.csv;data/international-eu-rhine-alpine-parity-gap-001.csv;docs/reviews/international-eu-rhine-alpine-adaptive-proof-closeout-001.md",
            "closeout_status": "adaptive_proof_complete_canada_depth_not_claimed",
            "allowed_claim": "EU is complete as an adaptive portability proof: the generic system advances where evidence exists and blocks where source custody is missing",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "use EU as adaptive proof beside Canada depth proof, not as Canada-equivalent internal adapter proof",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
