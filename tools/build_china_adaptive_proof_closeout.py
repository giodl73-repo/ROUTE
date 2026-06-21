#!/usr/bin/env python3
"""Build China adaptive proof closeout ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-china-adaptive-proof-closeout-001.csv"

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
    "canada_depth_equivalence;india_japan_content_depth_equivalence;"
    "internal_adapter_proof;official_network;official_corridor_designation;"
    "policy_alignment;route_designation;source_row_validation;"
    "fixture_replacement;parsed_adapter;geometry_acceptance;topology_proof;"
    "map_overlay;terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;agency_review;named_contact;construction_ready;"
    "guaranteed_sla;travel_time_proof;delivery_commitment;numeric_roi;"
    "roi;eligibility;compliance;endorsement;validation;external_validation;"
    "public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "closeout_id": "CHN-ADAPT-CLOSE-001",
            "proof_surface": "hierarchy_and_map_fixture",
            "input_artifacts": "docs/reviews/international-china-hierarchy-iteration-001.md;maps/international/china-candidate-hierarchy-v2.svg",
            "closeout_status": "adaptive_surface_complete_validation_held",
            "allowed_claim": "China reproduces the generic hierarchy and held-claim map fixture pattern for a large coastal-inland logistics context",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep map as schematic fixture until source-backed geometry and validation exist",
        },
        {
            "closeout_id": "CHN-ADAPT-CLOSE-002",
            "proof_surface": "source_kernel_and_parser_contract",
            "input_artifacts": "docs/reviews/international-china-adapter-source-pack-001.md;docs/reviews/international-china-kernel-application-001.md;docs/reviews/international-china-parser-preflight-001.md;docs/reviews/international-china-parser-dry-run-001.md",
            "closeout_status": "proof_kernel_instantiated_parser_ready_with_holds",
            "allowed_claim": "China instantiates the reusable proof kernel through source custody, payload classification, parser contract, and deterministic dry-run outputs",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "do not promote parsed adapter until source-row validation, geometry intake, and fixture closeout exist",
        },
        {
            "closeout_id": "CHN-ADAPT-CLOSE-003",
            "proof_surface": "dry_run_and_role_review",
            "input_artifacts": "docs/reviews/international-china-parser-dry-run-001.md;docs/reviews/international-china-dry-run-role-review-001.md",
            "closeout_status": "dry_run_rows_reviewed_source_row_validation_blocked",
            "allowed_claim": "China dry-run rows are bounded internal candidates with role-reviewed holds and explicit evidence labels",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep role-reviewed rows internal until accepted source rows are selected",
        },
        {
            "closeout_id": "CHN-ADAPT-CLOSE-004",
            "proof_surface": "geometry_and_fixture_blocker",
            "input_artifacts": "docs/reviews/international-china-geometry-policy-001.md;docs/reviews/international-china-fixture-blocker-001.md",
            "closeout_status": "geometry_rejected_fixture_replacement_blocked",
            "allowed_claim": "China dry-run rows are bounded internal candidates with an explicit no-geometry policy and fixture-replacement blocker",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep fixture replacement blocked until source-row validation and separate geometry intake close",
        },
        {
            "closeout_id": "CHN-ADAPT-CLOSE-005",
            "proof_surface": "fixture_replacement_decision",
            "input_artifacts": "data/international-china-fixture-blocker-001.csv;data/international-china-dry-run-role-review-001.csv;data/international-china-geometry-policy-001.csv",
            "closeout_status": "fixture_replacement_blocked_by_dry_run_only_evidence",
            "allowed_claim": "China safely blocks fixture replacement because current evidence is dry-run context and heuristic carry-forward, not accepted source-row validation",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "write accepted-source-row selection, content sampling, or renewed blocker before any fixture replacement contract",
        },
        {
            "closeout_id": "CHN-ADAPT-CLOSE-006",
            "proof_surface": "adaptive_proof_decision",
            "input_artifacts": "docs/reviews/international-china-adaptive-proof-closeout-001.md;docs/reviews/international-system-flexibility-proof-001.md",
            "closeout_status": "adaptive_proof_complete_at_dry_run_depth_canada_india_japan_depth_not_claimed",
            "allowed_claim": "China is complete as a dry-run-depth adaptive portability proof: the generic system moves through source custody, payload classification, parser contract, dry-run rows, role review, geometry policy, and blockers without overclaiming evidence",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "use China as dry-run-depth adaptive proof beside Canada depth proof and India/Japan content-depth adaptive proofs, not as Canada-equivalent or content-depth proof",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
