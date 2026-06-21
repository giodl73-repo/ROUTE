#!/usr/bin/env python3
"""Build China proof-kernel application ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-china-kernel-application-001.csv"

FIELDS = [
    "kernel_step",
    "china_status",
    "china_artifact",
    "canada_eu_india_japan_comparison",
    "promotion_decision",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "official_corridor_designation;policy_alignment;route_designation;"
    "source_row_validation;fixture_replacement;parsed_adapter;"
    "geometry_acceptance;topology_proof;map_overlay;terminal_performance;"
    "node_completeness;road_access_proof;throughput_proof;"
    "construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;"
    "endorsement;validation;external_validation;public_readiness;"
    "external_readiness"
)


def main() -> None:
    rows = [
        {
            "kernel_step": "source_custody",
            "china_status": "source_pack_preflight_declared",
            "china_artifact": "data/international-china-adapter-source-pack-001.csv",
            "canada_eu_india_japan_comparison": "Canada is depth proof; EU, India, and Japan are adaptive proofs; China starts source custody after hierarchy-only replication.",
            "promotion_decision": "preflight_ready_not_promoted",
            "blocked_claims": BLOCKED,
            "next_action": "classify payload access and source availability before parser contract",
        },
        {
            "kernel_step": "parser_contract",
            "china_status": "not_started",
            "china_artifact": "none",
            "canada_eu_india_japan_comparison": "Canada, EU, India, and Japan have parser contracts or closeouts; China must define a no-geometry contract after source inventory.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "define no-geometry output contract before dry-run fixture generation",
        },
        {
            "kernel_step": "fixture_replacement",
            "china_status": "not_started",
            "china_artifact": "data/international-china-candidate-hierarchy-v2.csv",
            "canada_eu_india_japan_comparison": "Canada replaced internal fixtures; EU replaced node fixture only; India and Japan block replacement at adaptive evidence gates; China remains heuristic-held.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "complete source-row validation role review and geometry policy before replacing any China fixture row",
        },
        {
            "kernel_step": "target_posture",
            "china_status": "not_started",
            "china_artifact": "data/international-china-adapter-source-pack-001.csv#CHN-SRC-SLA-001",
            "canada_eu_india_japan_comparison": "Canada and EU carry service targets only as held planning assumptions; India and Japan leave target posture blocked.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "create China target posture before any service reliability or policy-alignment language is promoted",
        },
        {
            "kernel_step": "review_packet",
            "china_status": "not_started",
            "china_artifact": "none",
            "canada_eu_india_japan_comparison": "Canada has media and external pathway preflights; EU, India, and Japan have adaptive closeouts; China has no review packet yet.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "wait until China source-backed internal proof or adaptive closeout exists before media or external review packet work",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
