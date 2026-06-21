#!/usr/bin/env python3
"""Build international next source-row gate selection ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-next-source-row-gate-001.csv"

FIELDS = [
    "candidate_id",
    "region_or_lane",
    "current_depth",
    "next_unblocked_gate",
    "why_this_gate",
    "proof_value",
    "risk_or_blocker",
    "recommendation",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "equal_depth_claim;official_network;official_corridor_designation;"
    "country_or_regional_approval;policy_alignment;route_designation;"
    "source_row_validation_until_gate_closes;fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;"
    "travel_time_proof;delivery_commitment;numeric_roi;roi;"
    "eligibility;compliance;endorsement;validation;external_validation;"
    "public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "candidate_id": "NEXT-GATE-001",
            "region_or_lane": "China",
            "current_depth": "dry_run_depth_adaptive_proof",
            "next_unblocked_gate": "china_source_content_sample",
            "why_this_gate": "China is the only completed adaptive branch still below content-depth; sampling bounded source content tests whether the generic process can deepen after dry-run proof.",
            "proof_value": "breadth_deepening",
            "risk_or_blocker": "source pages may remain context-only or inaccessible; source-row validation and fixture replacement stay blocked.",
            "recommendation": "primary",
            "blocked_claims": BLOCKED,
            "next_action": "create China source-content sample before any extraction, source-row validation, or fixture replacement claim",
        },
        {
            "candidate_id": "NEXT-GATE-002",
            "region_or_lane": "EU Rhine-Alpine",
            "current_depth": "adaptive_node_depth_road_link_blocked",
            "next_unblocked_gate": "eu_road_link_endpoint_custody",
            "why_this_gate": "EU has a known hard blocker: exact road-link endpoint custody. Closing it would move EU road-link replacement forward.",
            "proof_value": "hard_blocker_resolution",
            "risk_or_blocker": "endpoint may remain unavailable through public paths; named contact and agency review claims stay blocked.",
            "recommendation": "alternate",
            "blocked_claims": BLOCKED,
            "next_action": "continue EU road-link endpoint custody request before source-row extraction or road-link fixture replacement",
        },
        {
            "candidate_id": "NEXT-GATE-003",
            "region_or_lane": "Canada",
            "current_depth": "depth_proof_external_validation_held",
            "next_unblocked_gate": "canada_external_port_packet",
            "why_this_gate": "Canada is deepest and closest to review, but this improves external pathway rather than proving more portability breadth.",
            "proof_value": "review_readiness",
            "risk_or_blocker": "named venue, review, approval, endorsement, and external validation claims stay blocked.",
            "recommendation": "alternate",
            "blocked_claims": BLOCKED,
            "next_action": "prepare narrow Canada port packet only when external-review work is prioritized over portability-depth work",
        },
        {
            "candidate_id": "NEXT-GATE-004",
            "region_or_lane": "India",
            "current_depth": "content_depth_adaptive_proof",
            "next_unblocked_gate": "india_accepted_source_row_selection",
            "why_this_gate": "India can deepen from content rows toward accepted source rows, but it is already above China in the proof ladder.",
            "proof_value": "adaptive_depth_extension",
            "risk_or_blocker": "accepted road-link, port-node, or statistics row selection may require narrower source custody and geometry policy remains held.",
            "recommendation": "defer",
            "blocked_claims": BLOCKED,
            "next_action": "defer until China reaches content-depth or EU road-link endpoint custody is reprioritized",
        },
        {
            "candidate_id": "NEXT-GATE-005",
            "region_or_lane": "Japan",
            "current_depth": "content_depth_adaptive_proof_with_source_needed_blocker",
            "next_unblocked_gate": "japan_gsi_or_alternative_road_link_custody",
            "why_this_gate": "Japan has a clean source-needed road-link blocker, but resolving it is narrower than lifting China from dry-run-depth.",
            "proof_value": "source_needed_blocker_resolution",
            "risk_or_blocker": "GSI metadata may remain source-needed; disaster-readiness, geometry, and source-row validation stay blocked.",
            "recommendation": "defer",
            "blocked_claims": BLOCKED,
            "next_action": "defer until China content-depth attempt or EU endpoint work is complete",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
