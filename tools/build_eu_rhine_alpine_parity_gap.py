#!/usr/bin/env python3
"""Build EU Rhine-Alpine parity gap ledger against Canada internal proof."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-parity-gap-001.csv"

FIELDS = [
    "gap_id",
    "canada_parity_surface",
    "eu_current_artifact",
    "eu_status",
    "parity_decision",
    "blocked_claims",
    "required_next_step",
]

BLOCKED = (
    "official_corridor_designation;member_state_approval;route_designation;"
    "geometry_acceptance;topology_proof;map_overlay;terminal_performance;"
    "node_completeness;road_access_proof;construction_ready;guaranteed_sla;"
    "travel_time_proof;delivery_commitment;numeric_roi;roi;eligibility;"
    "compliance;endorsement;validation;external_validation;public_readiness;"
    "external_readiness;internal_adapter_proof"
)


def main() -> None:
    rows = [
        {
            "gap_id": "EUR-PARITY-001",
            "canada_parity_surface": "source pack plus payload probe",
            "eu_current_artifact": "data/international-eu-rhine-alpine-source-payload-probe-001.csv",
            "eu_status": "parity_reached_evidence_not_accepted",
            "parity_decision": "complete_for_pre_validation_layer",
            "blocked_claims": BLOCKED,
            "required_next_step": "continue to field inventory and source-row validation",
        },
        {
            "gap_id": "EUR-PARITY-002",
            "canada_parity_surface": "parser preflight and dry-run fixture",
            "eu_current_artifact": "docs/reviews/international-eu-rhine-alpine-parser-dry-run-001.md",
            "eu_status": "parity_reached_fixture_replacement_held",
            "parity_decision": "complete_for_dry_run_layer",
            "blocked_claims": BLOCKED,
            "required_next_step": "select exact parse fields before source-derived replacement rows",
        },
        {
            "gap_id": "EUR-PARITY-003",
            "canada_parity_surface": "source field inventory and bounded source-row validation",
            "eu_current_artifact": "docs/reviews/international-eu-rhine-alpine-source-row-validation-001.md",
            "eu_status": "bounded_metadata_validation_only",
            "parity_decision": "partial_not_canada_equivalent",
            "blocked_claims": BLOCKED,
            "required_next_step": "extract source-content rows or selected official node custody before fixture replacement",
        },
        {
            "gap_id": "EUR-PARITY-004",
            "canada_parity_surface": "link fixture replacement closeout",
            "eu_current_artifact": "data/eu_rhine_alpine_source_link_candidates.csv",
            "eu_status": "metadata_rows_not_source_derived_links",
            "parity_decision": "blocked",
            "blocked_claims": BLOCKED,
            "required_next_step": "build source-derived no-geometry EU link candidates before fixture replacement",
        },
        {
            "gap_id": "EUR-PARITY-005",
            "canada_parity_surface": "node fixture replacement closeout",
            "eu_current_artifact": "data/eu_rhine_alpine_source_node_candidates.csv",
            "eu_status": "source_needed_node_gap",
            "parity_decision": "blocked",
            "blocked_claims": BLOCKED,
            "required_next_step": "select official port terminal gateway node sources before node fixture replacement",
        },
        {
            "gap_id": "EUR-PARITY-006",
            "canada_parity_surface": "target posture and internal adapter proof",
            "eu_current_artifact": "data/eu_rhine_alpine_service_target_candidates.csv",
            "eu_status": "held_target_assumption_no_internal_proof",
            "parity_decision": "blocked",
            "blocked_claims": BLOCKED,
            "required_next_step": "close link and node replacement plus EU target posture before internal proof",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
