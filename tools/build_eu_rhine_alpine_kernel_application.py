#!/usr/bin/env python3
"""Build EU Rhine-Alpine proof-kernel application ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-kernel-application-001.csv"

FIELDS = [
    "kernel_step",
    "eu_status",
    "eu_artifact",
    "canada_comparison",
    "promotion_decision",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "official_corridor_designation;member_state_approval;route_designation;"
    "geometry_acceptance;topology_proof;terminal_performance;construction_ready;"
    "guaranteed_sla;travel_time_proof;delivery_commitment;numeric_roi;roi;"
    "eligibility;compliance;endorsement;validation;external_validation;"
    "public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "kernel_step": "source_custody",
            "eu_status": "source_pack_preflight_declared",
            "eu_artifact": "data/international-eu-rhine-alpine-adapter-source-pack-001.csv",
            "canada_comparison": "Canada has source pack plus payload, field, row-validation, and node closeouts.",
            "promotion_decision": "preflight_ready_not_promoted",
            "blocked_claims": BLOCKED,
            "next_action": "inspect source metadata and select parseable fields before parser contract",
        },
        {
            "kernel_step": "parser_contract",
            "eu_status": "parser_preflight_and_dry_run_ready",
            "eu_artifact": "data/international-eu-rhine-alpine-parser-preflight-001.csv;data/international-eu-rhine-alpine-parser-output-contract-001.csv;docs/reviews/international-eu-rhine-alpine-parser-dry-run-001.md",
            "canada_comparison": "Canada has parser preflight, output contract, dry-run generator, and gate.",
            "promotion_decision": "dry_run_ready_not_promoted",
            "blocked_claims": BLOCKED,
            "next_action": "run payload access and field inventory before source-row validation or fixture replacement",
        },
        {
            "kernel_step": "fixture_replacement",
            "eu_status": "not_started",
            "eu_artifact": "hierarchy fixture remains heuristic-held",
            "canada_comparison": "Canada has link and node fixture replacement closeouts.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "close payload access, field inventory, source-row validation, role review, and geometry policy before replacing EU hierarchy rows",
        },
        {
            "kernel_step": "target_posture",
            "eu_status": "target_assumptions_held",
            "eu_artifact": "data/international-eu-rhine-alpine-adapter-source-pack-001.csv#EUR-SRC-SLA-001",
            "canada_comparison": "Canada has target posture and internal proof closeout.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "create EU target posture before any service or reliability language is promoted",
        },
        {
            "kernel_step": "review_packet",
            "eu_status": "not_started",
            "eu_artifact": "none",
            "canada_comparison": "Canada has media proof, external pathway, and port packet preflight.",
            "promotion_decision": "held",
            "blocked_claims": BLOCKED,
            "next_action": "wait until EU internal proof exists before media or external review packet work",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
