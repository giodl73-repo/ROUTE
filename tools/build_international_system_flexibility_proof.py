#!/usr/bin/env python3
"""Build international system flexibility proof ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-system-flexibility-proof-001.csv"

FIELDS = [
    "proof_id",
    "region_or_surface",
    "kernel_step",
    "observed_variation",
    "system_response",
    "flexibility_decision",
    "evidence_artifacts",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "official_network;official_corridor_designation;route_designation;"
    "agency_approval;member_state_approval;external_validation;"
    "geometry_acceptance;topology_proof;map_overlay;terminal_performance;"
    "node_completeness;road_access_proof;throughput_proof;"
    "construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;"
    "endorsement;validation;public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "proof_id": "FLEX-001",
            "region_or_surface": "Canada",
            "kernel_step": "source_custody_to_internal_proof",
            "observed_variation": "public road-graph and selected port source-custody rows can support bounded no-geometry internal fixture replacement",
            "system_response": "Canada proceeds through source rows, geometry policy, link and node fixture closeout, target posture, and internal proof",
            "flexibility_decision": "depth_instance_complete_external_validation_held",
            "evidence_artifacts": "docs/reviews/international-canada-internal-adapter-proof-001.md;docs/media/canada-internal-proof-brief.md",
            "blocked_claims": BLOCKED,
            "next_action": "use Canada as depth proof while keeping official and external claims held",
        },
        {
            "proof_id": "FLEX-002",
            "region_or_surface": "EU Rhine-Alpine",
            "kernel_step": "source_custody_to_extraction_candidate",
            "observed_variation": "official source content is available but road-service fixture scope diverges between current European Transport Corridors and Rhine-Alpine legacy context",
            "system_response": "EU branches into source-content sample, extraction candidates, content row validation, and current-corridor rebase gate",
            "flexibility_decision": "adaptive_branch_complete_fixture_replacement_held",
            "evidence_artifacts": "docs/reviews/international-eu-rhine-alpine-source-content-sample-001.md;docs/reviews/international-eu-rhine-alpine-parser-extraction-candidates-001.md;docs/reviews/international-eu-rhine-alpine-current-corridor-rebase-001.md",
            "blocked_claims": BLOCKED,
            "next_action": "choose current corridor scope or legacy context before road-feature selection and fixture replacement",
        },
        {
            "proof_id": "FLEX-003",
            "region_or_surface": "EU Rhine-Alpine",
            "kernel_step": "parity_gap_detection",
            "observed_variation": "EU reaches pre-validation and source-content layers but lacks Canada-equivalent road-feature rows and selected node custody",
            "system_response": "parity ledger names exact blocked surfaces instead of promoting internal adapter proof",
            "flexibility_decision": "gap_detected_without_false_promotion",
            "evidence_artifacts": "docs/reviews/international-eu-rhine-alpine-parity-gap-001.md;data/international-eu-rhine-alpine-parity-gap-001.csv",
            "blocked_claims": BLOCKED,
            "next_action": "select road-feature and node-custody sources before replacement contract",
        },
        {
            "proof_id": "FLEX-004",
            "region_or_surface": "India",
            "kernel_step": "source_custody_to_content_row_traceability",
            "observed_variation": "official highway and port source content supports bounded content candidates but does not yet provide accepted road-link, terminal, or statistics table rows",
            "system_response": "India advances through source pack, parser contract, fixture blocker, source-content sample, extraction candidates, content-row validation, role review, and adaptive closeout",
            "flexibility_decision": "adaptive_branch_complete_source_row_validation_held",
            "evidence_artifacts": "docs/reviews/international-india-adaptive-proof-closeout-001.md;data/international-india-adaptive-proof-closeout-001.csv",
            "blocked_claims": BLOCKED,
            "next_action": "select accepted road-link port-node or statistics table rows before source-row validation or fixture replacement",
        },
        {
            "proof_id": "FLEX-005",
            "region_or_surface": "Japan",
            "kernel_step": "source_custody_to_source_needed_blocker",
            "observed_variation": "MLIT and e-Stat content supports bounded content candidates while GSI road-link metadata remains source-needed for link extraction",
            "system_response": "Japan advances through source pack, parser contract, dry run, extraction candidates, content-row validation, role review, geometry policy, fixture blocker, and adaptive closeout",
            "flexibility_decision": "adaptive_branch_complete_source_row_validation_held",
            "evidence_artifacts": "docs/reviews/international-japan-adaptive-proof-closeout-001.md;data/international-japan-adaptive-proof-closeout-001.csv",
            "blocked_claims": BLOCKED,
            "next_action": "resolve GSI road-link source custody or alternative accepted road-feature rows before source-row validation or fixture replacement",
        },
        {
            "proof_id": "FLEX-006",
            "region_or_surface": "multi-region maps",
            "kernel_step": "adapter_to_map_fixture",
            "observed_variation": "Canada, EU, India, Japan, and China produce comparable held-claim schematic fixtures from different geography and governance patterns",
            "system_response": "pilot maps remain replicability fixtures with validation held and official-network and SLA claims blocked",
            "flexibility_decision": "breadth_instance_complete_validation_held",
            "evidence_artifacts": "docs/reviews/international-portability-pilot-map-run-001.md;data/international-portability-pilot-map-index.csv",
            "blocked_claims": BLOCKED,
            "next_action": "continue source-backed adapter proof per region rather than treating maps as proof",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
