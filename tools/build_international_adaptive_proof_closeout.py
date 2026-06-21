#!/usr/bin/env python3
"""Build international adaptive proof closeout ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-adaptive-proof-closeout-001.csv"

FIELDS = [
    "closeout_id",
    "proof_lane",
    "depth_level",
    "input_artifacts",
    "allowed_claim",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "single_depth_equivalence;official_network;official_corridor_designation;"
    "country_or_regional_approval;policy_alignment;route_designation;"
    "source_row_validation_where_not_closed;fixture_replacement_where_not_closed;"
    "parsed_adapter_where_not_closed;geometry_acceptance;topology_proof;"
    "map_overlay;terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;"
    "endorsement;validation;external_validation;public_readiness;"
    "external_readiness"
)


def main() -> None:
    rows = [
        {
            "closeout_id": "INTL-ADAPT-CLOSE-001",
            "proof_lane": "Canada",
            "depth_level": "depth_proof_external_validation_held",
            "input_artifacts": "docs/reviews/international-canada-internal-adapter-proof-001.md;docs/media/canada-internal-proof-brief.md",
            "allowed_claim": "Canada is the deepest internal adapter proof and demonstrates the reusable kernel can reach bounded internal proof when source rows and replacement gates close",
            "blocked_claims": BLOCKED,
            "next_action": "use Canada as depth reference while keeping official network, approval, external validation, public readiness, SLA, and ROI held",
        },
        {
            "closeout_id": "INTL-ADAPT-CLOSE-002",
            "proof_lane": "EU Rhine-Alpine",
            "depth_level": "adaptive_proof_node_depth_road_link_blocked",
            "input_artifacts": "docs/reviews/international-eu-rhine-alpine-adaptive-proof-closeout-001.md;data/international-eu-rhine-alpine-adaptive-proof-closeout-001.csv",
            "allowed_claim": "EU proves the kernel can adapt under corridor-scope mismatch and partial node fixture closeout while road-link replacement remains blocked",
            "blocked_claims": BLOCKED,
            "next_action": "resolve current corridor scope and road-link endpoint custody before claiming road-link fixture replacement or Canada-depth proof",
        },
        {
            "closeout_id": "INTL-ADAPT-CLOSE-003",
            "proof_lane": "India",
            "depth_level": "adaptive_proof_content_depth_source_row_validation_held",
            "input_artifacts": "docs/reviews/international-india-adaptive-proof-closeout-001.md;data/international-india-adaptive-proof-closeout-001.csv",
            "allowed_claim": "India proves the kernel can progress from source custody into content candidates and role-reviewed blockers without source-row validation",
            "blocked_claims": BLOCKED,
            "next_action": "select accepted road-link, port-node, or statistics rows before source-row validation or fixture replacement",
        },
        {
            "closeout_id": "INTL-ADAPT-CLOSE-004",
            "proof_lane": "Japan",
            "depth_level": "adaptive_proof_content_depth_source_needed_blocker",
            "input_artifacts": "docs/reviews/international-japan-adaptive-proof-closeout-001.md;data/international-japan-adaptive-proof-closeout-001.csv",
            "allowed_claim": "Japan proves the kernel can advance through source-content candidates while preserving a source-needed road-link blocker",
            "blocked_claims": BLOCKED,
            "next_action": "resolve GSI road-link source custody or alternative accepted road-feature rows before source-row validation or fixture replacement",
        },
        {
            "closeout_id": "INTL-ADAPT-CLOSE-005",
            "proof_lane": "China",
            "depth_level": "adaptive_proof_content_depth_started_source_row_validation_held",
            "input_artifacts": "docs/reviews/international-china-source-content-sample-001.md;docs/reviews/international-china-parser-extraction-candidates-001.md",
            "allowed_claim": "China proves the kernel can move beyond dry-run rows into bounded source-content samples and extraction candidates without accepted source rows",
            "blocked_claims": BLOCKED,
            "next_action": "select accepted source rows before source-row validation, fixture replacement, or China depth-proof claims",
        },
        {
            "closeout_id": "INTL-ADAPT-CLOSE-006",
            "proof_lane": "multi-region maps",
            "depth_level": "breadth_fixture_validation_held",
            "input_artifacts": "docs/reviews/international-portability-pilot-map-run-001.md;data/international-portability-pilot-map-index.csv",
            "allowed_claim": "Map fixtures show breadth of adapter-to-render workflow only, not official network or performance proof",
            "blocked_claims": BLOCKED,
            "next_action": "keep maps captioned as held-claim fixtures and route proof claims back to source-bound adapter gates",
        },
        {
            "closeout_id": "INTL-ADAPT-CLOSE-007",
            "proof_lane": "international_system",
            "depth_level": "portfolio_proof_ladder_complete_validation_held",
            "input_artifacts": "docs/reviews/international-system-flexibility-proof-001.md;data/international-system-flexibility-proof-001.csv",
            "allowed_claim": "ROUTE now has a bounded international proof ladder: one Canada depth proof, EU/India/Japan adaptive branches, China content-depth-started branch, and map breadth fixtures",
            "blocked_claims": BLOCKED,
            "next_action": "choose the next region-specific source-row or external-review gate without treating all regions as equally proven",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
