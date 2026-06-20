#!/usr/bin/env python3
"""Build Canada media proof card with explicit claim holds."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-canada-media-proof-card-001.csv"

FIELDS = [
    "card_id",
    "media_question",
    "safe_answer",
    "cite",
    "status",
    "blocked_claims",
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
            "card_id": "CAN-MEDIA-PROOF-001",
            "media_question": "What does the Canada pilot prove?",
            "safe_answer": "It proves internally that ROUTE can move a non-U.S. country pilot through source custody, parser-shaped fixtures, role review, target holds, and proof closeout.",
            "cite": "docs/reviews/international-canada-internal-adapter-proof-001.md",
            "status": "internal_adapter_proof_ready_external_validation_held",
            "blocked_claims": BLOCKED_CLAIMS,
        },
        {
            "card_id": "CAN-MEDIA-PROOF-002",
            "media_question": "Can it be described as a Canadian transport plan?",
            "safe_answer": "No. It is an internal evidence-gated workflow demonstration, not a government-adopted transport program.",
            "cite": "docs/media/canada-internal-proof-brief.md",
            "status": "official_claims_blocked",
            "blocked_claims": BLOCKED_CLAIMS,
        },
        {
            "card_id": "CAN-MEDIA-PROOF-003",
            "media_question": "Can the Canada maps or nodes be used as proof?",
            "safe_answer": "No. Maps are structural visuals, and node rows are internal source-custody candidates; geometry, topology, terminal performance, road access, and throughput proof remain held.",
            "cite": "docs/reports/maps-are-not-proof-report.md;docs/reviews/international-canada-node-fixture-replacement-closeout-001.md",
            "status": "map_and_node_proof_blocked",
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
