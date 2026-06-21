#!/usr/bin/env python3
"""Build Japan proof-kernel application ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-japan-kernel-application-001.csv"

FIELDS = [
    "kernel_step",
    "japan_status",
    "japan_artifact",
    "canada_eu_india_comparison",
    "promotion_decision",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "official_corridor_designation;ministry_approval;route_designation;"
    "source_row_validation;fixture_replacement;parsed_adapter;"
    "geometry_acceptance;topology_proof;map_overlay;disaster_readiness;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;"
    "travel_time_proof;delivery_commitment;numeric_roi;roi;eligibility;"
    "compliance;endorsement;validation;external_validation;"
    "public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "kernel_step": "source_custody",
            "japan_status": "source_pack_preflight_declared",
            "japan_artifact": "data/international-japan-adapter-source-pack-001.csv",
            "canada_eu_india_comparison": "Canada is depth proof; EU and India are adaptive proofs; Japan starts the next source-custody branch after map-only hierarchy iteration.",
            "promotion_decision": "preflight_ready_not_promoted",
            "blocked_claims": BLOCKED,
            "next_action": "probe source access and field availability before parser contract",
        },
        {
            "kernel_step": "parser_contract",
            "japan_status": "not_started",
            "japan_artifact": "none",
            "canada_eu_india_comparison": "Canada, EU, and India have parser contracts; Japan must define a no-geometry contract after source inventory.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "define no-geometry output contract before dry-run fixture generation",
        },
        {
            "kernel_step": "fixture_replacement",
            "japan_status": "not_started",
            "japan_artifact": "data/international-japan-candidate-hierarchy-v2.csv",
            "canada_eu_india_comparison": "Canada replaced internal fixtures; EU replaced node fixture only; India blocked replacement at content-row evidence; Japan remains heuristic-held.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "complete source-row validation role review and geometry policy before replacing any Japan fixture row",
        },
        {
            "kernel_step": "target_posture",
            "japan_status": "not_started",
            "japan_artifact": "data/international-japan-adapter-source-pack-001.csv#JPN-SRC-SLA-001",
            "canada_eu_india_comparison": "Canada and EU carry service targets only as held planning assumptions; India left target posture blocked.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "create Japan target posture before any service reliability or disaster-readiness language is promoted",
        },
        {
            "kernel_step": "review_packet",
            "japan_status": "not_started",
            "japan_artifact": "none",
            "canada_eu_india_comparison": "Canada has media and external pathway preflights; EU and India have adaptive closeouts; Japan has no review packet yet.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "wait until Japan source-backed internal proof or adaptive closeout exists before media or external review packet work",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
