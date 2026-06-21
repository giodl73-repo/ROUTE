#!/usr/bin/env python3
"""Build Japan adaptive proof closeout ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-japan-adaptive-proof-closeout-001.csv"

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
    "official_corridor_designation;ministry_approval;route_designation;"
    "source_row_validation;fixture_replacement;parsed_adapter;"
    "geometry_acceptance;topology_proof;map_overlay;disaster_readiness;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;agency_review;named_contact;construction_ready;"
    "guaranteed_sla;travel_time_proof;delivery_commitment;numeric_roi;"
    "roi;eligibility;compliance;endorsement;validation;external_validation;"
    "public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "closeout_id": "JPN-ADAPT-CLOSE-001",
            "proof_surface": "hierarchy_and_map_fixture",
            "input_artifacts": "docs/reviews/international-japan-hierarchy-iteration-001.md;maps/international/japan-candidate-hierarchy-v2.svg",
            "closeout_status": "adaptive_surface_complete_validation_held",
            "allowed_claim": "Japan reproduces the generic hierarchy and held-claim map fixture pattern for an island and geohazard corridor context",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep map as schematic fixture until source-backed geometry and validation exist",
        },
        {
            "closeout_id": "JPN-ADAPT-CLOSE-002",
            "proof_surface": "source_kernel_and_parser_contract",
            "input_artifacts": "docs/reviews/international-japan-adapter-source-pack-001.md;docs/reviews/international-japan-kernel-application-001.md;docs/reviews/international-japan-parser-preflight-001.md;docs/reviews/international-japan-parser-dry-run-001.md",
            "closeout_status": "proof_kernel_instantiated_parser_ready_with_holds",
            "allowed_claim": "Japan instantiates the reusable proof kernel through source custody, parser contract, and deterministic dry-run outputs",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "do not promote parsed adapter until source-row validation, GSI road-link custody, and fixture closeout exist",
        },
        {
            "closeout_id": "JPN-ADAPT-CLOSE-003",
            "proof_surface": "source_content_branch",
            "input_artifacts": "docs/reviews/international-japan-source-content-sample-001.md;docs/reviews/international-japan-parser-extraction-candidates-001.md;docs/reviews/international-japan-source-content-row-validation-001.md;docs/reviews/international-japan-content-row-role-review-001.md",
            "closeout_status": "content_rows_matched_source_rows_not_validated",
            "allowed_claim": "Japan advances beyond scaffolding into sampled source-content candidates and an explicit GSI source-needed road-link blocker",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "resolve accepted road-link, port-node, or statistics table rows before source-row validation",
        },
        {
            "closeout_id": "JPN-ADAPT-CLOSE-004",
            "proof_surface": "geometry_and_fixture_blocker",
            "input_artifacts": "docs/reviews/international-japan-geometry-policy-001.md;docs/reviews/international-japan-fixture-blocker-001.md",
            "closeout_status": "geometry_rejected_fixture_replacement_blocked",
            "allowed_claim": "Japan dry-run and content rows are bounded internal candidates with role-reviewed holds and an explicit no-geometry policy",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep fixture replacement blocked until source custody or a separate geometry intake closes",
        },
        {
            "closeout_id": "JPN-ADAPT-CLOSE-005",
            "proof_surface": "fixture_replacement_decision",
            "input_artifacts": "data/international-japan-fixture-blocker-001.csv;data/international-japan-source-content-row-validation-001.csv;data/international-japan-content-row-role-review-001.csv",
            "closeout_status": "fixture_replacement_blocked_by_content_only_and_source_needed_evidence",
            "allowed_claim": "Japan safely blocks fixture replacement because current evidence is content-row matching plus a source-needed road-link blocker, not accepted source-row validation",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "write accepted-source-row selection, road-link source acquisition, or renewed blocker before any fixture replacement contract",
        },
        {
            "closeout_id": "JPN-ADAPT-CLOSE-006",
            "proof_surface": "adaptive_proof_decision",
            "input_artifacts": "docs/reviews/international-japan-adaptive-proof-closeout-001.md;docs/reviews/international-system-flexibility-proof-001.md",
            "closeout_status": "adaptive_proof_complete_canada_depth_not_claimed",
            "allowed_claim": "Japan is complete as an adaptive portability proof: the generic system moves through source custody, parser contract, content candidates, role review, geometry policy, and blockers without overclaiming evidence",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "use Japan as adaptive proof beside Canada depth proof, EU adaptive proof, and India adaptive proof, not as Canada-equivalent internal adapter proof",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
