#!/usr/bin/env python3
"""Build India proof-kernel application ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-india-kernel-application-001.csv"

FIELDS = [
    "kernel_step",
    "india_status",
    "india_artifact",
    "canada_eu_comparison",
    "promotion_decision",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "official_corridor_designation;national_approval;state_approval;"
    "route_designation;source_row_validation;fixture_replacement;"
    "geometry_acceptance;topology_proof;map_overlay;terminal_performance;"
    "construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;"
    "endorsement;validation;external_validation;public_readiness;"
    "external_readiness"
)


def main() -> None:
    rows = [
        {
            "kernel_step": "source_custody",
            "india_status": "source_pack_preflight_declared",
            "india_artifact": "data/international-india-adapter-source-pack-001.csv",
            "canada_eu_comparison": "Canada is depth proof; EU is adaptive proof with road-link blocker; India starts a third-region source-custody branch.",
            "promotion_decision": "preflight_ready_not_promoted",
            "blocked_claims": BLOCKED,
            "next_action": "probe source access and field availability before parser contract",
        },
        {
            "kernel_step": "parser_contract",
            "india_status": "not_started",
            "india_artifact": "none",
            "canada_eu_comparison": "Canada and EU have parser preflight contracts; India must define contract after source inventory.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "define no-geometry output contract before dry-run fixture generation",
        },
        {
            "kernel_step": "fixture_replacement",
            "india_status": "not_started",
            "india_artifact": "data/international-india-candidate-hierarchy-v2.csv",
            "canada_eu_comparison": "Canada replaced link and node fixtures; EU replaced node fixture only; India remains heuristic-held.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "complete source-row validation role review and geometry policy before replacing any India fixture row",
        },
        {
            "kernel_step": "target_posture",
            "india_status": "not_started",
            "india_artifact": "data/international-india-adapter-source-pack-001.csv#IND-SRC-SLA-001",
            "canada_eu_comparison": "Canada and EU carry service targets only as held planning assumptions.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "create India target posture before any service or reliability language is promoted",
        },
        {
            "kernel_step": "review_packet",
            "india_status": "not_started",
            "india_artifact": "none",
            "canada_eu_comparison": "Canada has media and external pathway preflights; EU has adaptive closeout; India has no review packet yet.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "wait until India source-backed internal proof or adaptive closeout exists before media or external review packet work",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
