#!/usr/bin/env python3
"""Build reusable international adapter proof-kernel ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-adapter-proof-kernel-001.csv"

FIELDS = [
    "kernel_step",
    "generic_function",
    "canada_instance",
    "evidence_artifact",
    "status",
    "reusable_for",
    "blocked_claims",
]

BLOCKED_CLAIMS = (
    "official_network;route_designation;agency_approval;external_validation;"
    "geometry_acceptance;topology_proof;map_overlay;terminal_performance;"
    "node_completeness;road_access_proof;throughput_proof;construction_ready;"
    "guaranteed_sla;travel_time_proof;delivery_commitment;numeric_roi;roi;"
    "eligibility;compliance;endorsement;validation;public_readiness;"
    "external_readiness"
)


def main() -> None:
    rows = [
        {
            "kernel_step": "source_custody",
            "generic_function": "declare source owners, dates, URLs, access notes, and evidence labels before adapter promotion",
            "canada_instance": "Canada source pack, payload access, payload probe, field inventory, node-source selection, and node-source probe",
            "evidence_artifact": "docs/reviews/international-canada-adapter-source-pack-001.md;docs/reviews/international-canada-source-payload-access-001.md;docs/reviews/international-canada-node-source-selection-001.md",
            "status": "generic_kernel_instantiated_by_canada",
            "reusable_for": "country_or_region_adapter",
            "blocked_claims": BLOCKED_CLAIMS,
        },
        {
            "kernel_step": "parser_contract",
            "generic_function": "define adapter output columns, evidence labels, and acceptance rules before parsing or fixture replacement",
            "canada_instance": "Canada parser preflight, parser output contract, dry-run generator, and dry-run gate",
            "evidence_artifact": "docs/reviews/international-canada-parser-output-contract-001.md;docs/reviews/international-canada-parser-dry-run-gate-001.md",
            "status": "generic_kernel_instantiated_by_canada",
            "reusable_for": "country_or_region_adapter",
            "blocked_claims": BLOCKED_CLAIMS,
        },
        {
            "kernel_step": "fixture_replacement",
            "generic_function": "replace internal rows only after source-row validation, role review, geometry policy, and closeout",
            "canada_instance": "Canada source-row validation, geometry policy, link fixture replacement, and node fixture replacement",
            "evidence_artifact": "docs/reviews/international-canada-source-row-validation-001.md;docs/reviews/international-canada-link-fixture-replacement-closeout-001.md;docs/reviews/international-canada-node-fixture-replacement-closeout-001.md",
            "status": "generic_kernel_instantiated_by_canada",
            "reusable_for": "country_or_region_adapter",
            "blocked_claims": BLOCKED_CLAIMS,
        },
        {
            "kernel_step": "target_posture",
            "generic_function": "hold service targets as planning assumptions until local evidence, authority, and numeracy close",
            "canada_instance": "Canada target posture and internal adapter proof",
            "evidence_artifact": "docs/reviews/international-canada-target-posture-001.md;docs/reviews/international-canada-internal-adapter-proof-001.md",
            "status": "generic_kernel_instantiated_by_canada",
            "reusable_for": "country_or_region_adapter",
            "blocked_claims": BLOCKED_CLAIMS,
        },
        {
            "kernel_step": "review_packet",
            "generic_function": "convert internal proof into bounded review/media/external packet surfaces without claiming external validation",
            "canada_instance": "Canada media proof, external review pathway, and port-authority packet preflight",
            "evidence_artifact": "docs/media/canada-internal-proof-brief.md;docs/reviews/international-canada-external-review-pathway-001.md;docs/reviews/international-canada-port-authority-packet-preflight-001.md",
            "status": "generic_kernel_instantiated_by_canada",
            "reusable_for": "country_or_region_adapter",
            "blocked_claims": BLOCKED_CLAIMS,
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
