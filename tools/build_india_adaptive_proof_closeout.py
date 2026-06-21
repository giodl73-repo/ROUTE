#!/usr/bin/env python3
"""Build India adaptive proof closeout ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-india-adaptive-proof-closeout-001.csv"

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
    "official_corridor_designation;national_approval;state_approval;"
    "route_designation;source_row_validation;fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;agency_review;named_contact;construction_ready;"
    "guaranteed_sla;travel_time_proof;delivery_commitment;numeric_roi;"
    "roi;eligibility;compliance;endorsement;validation;external_validation;"
    "public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "closeout_id": "IND-ADAPT-CLOSE-001",
            "proof_surface": "hierarchy_and_map_fixture",
            "input_artifacts": "docs/reviews/international-india-hierarchy-iteration-001.md;maps/international/india-candidate-hierarchy-v2.svg",
            "closeout_status": "adaptive_surface_complete_validation_held",
            "allowed_claim": "India reproduces the generic hierarchy and held-claim map fixture pattern for a large multimodal country context",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep map as schematic fixture until source-backed geometry and validation exist",
        },
        {
            "closeout_id": "IND-ADAPT-CLOSE-002",
            "proof_surface": "source_kernel_and_parser_contract",
            "input_artifacts": "docs/reviews/international-india-adapter-source-pack-001.md;docs/reviews/international-india-kernel-application-001.md;docs/reviews/international-india-parser-preflight-001.md;docs/reviews/international-india-parser-dry-run-001.md",
            "closeout_status": "proof_kernel_instantiated_parser_ready_with_holds",
            "allowed_claim": "India instantiates the reusable proof kernel through source custody, parser contract, and deterministic dry-run outputs",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "do not promote parsed adapter until source-row validation and fixture closeout exist",
        },
        {
            "closeout_id": "IND-ADAPT-CLOSE-003",
            "proof_surface": "dry_run_and_geometry_blocker",
            "input_artifacts": "docs/reviews/international-india-source-row-validation-001.md;docs/reviews/international-india-role-review-001.md;docs/reviews/international-india-geometry-policy-001.md;docs/reviews/international-india-fixture-blocker-001.md",
            "closeout_status": "dry_run_rows_reviewed_fixture_replacement_blocked",
            "allowed_claim": "India dry-run rows are bounded internal candidates with role-reviewed holds and an explicit no-geometry policy",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep fixture replacement blocked until accepted source rows or a separate geometry intake close",
        },
        {
            "closeout_id": "IND-ADAPT-CLOSE-004",
            "proof_surface": "source_content_branch",
            "input_artifacts": "docs/reviews/international-india-source-content-sample-001.md;docs/reviews/international-india-parser-extraction-candidates-001.md;docs/reviews/international-india-source-content-row-validation-001.md;docs/reviews/international-india-content-row-role-review-001.md",
            "closeout_status": "content_rows_matched_source_rows_not_validated",
            "allowed_claim": "India advances beyond scaffolding into sampled source-content candidates that trace back to source-content rows",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "select accepted road-link port-node or statistics table rows before source-row validation",
        },
        {
            "closeout_id": "IND-ADAPT-CLOSE-005",
            "proof_surface": "fixture_replacement_decision",
            "input_artifacts": "data/international-india-fixture-blocker-001.csv;data/international-india-source-content-row-validation-001.csv;data/international-india-content-row-role-review-001.csv",
            "closeout_status": "fixture_replacement_blocked_by_content_only_evidence",
            "allowed_claim": "India safely blocks fixture replacement because current evidence is content-row matching, not accepted source-row validation",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "write accepted-source-row selection or renewed blocker before any fixture replacement contract",
        },
        {
            "closeout_id": "IND-ADAPT-CLOSE-006",
            "proof_surface": "adaptive_proof_decision",
            "input_artifacts": "data/international-system-flexibility-proof-001.csv;docs/reviews/international-india-adaptive-proof-closeout-001.md",
            "closeout_status": "adaptive_proof_complete_canada_depth_not_claimed",
            "allowed_claim": "India is complete as an adaptive portability proof: the generic system moves through source custody, parser contract, content candidates, role review, and blockers without overclaiming evidence",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "use India as adaptive proof beside Canada depth proof and EU adaptive proof, not as Canada-equivalent internal adapter proof",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
