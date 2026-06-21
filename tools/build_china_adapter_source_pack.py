#!/usr/bin/env python3
"""Build China adapter source-pack preflight ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-china-adapter-source-pack-001.csv"

FIELDS = [
    "source_family",
    "source_id",
    "source_path_or_status",
    "owner_or_publisher",
    "date_accessed",
    "required_fields",
    "adapter_target",
    "promotion_decision",
    "claim_boundary",
    "next_action",
]

BLOCKED = (
    "no official Chinese corridor designation policy alignment route "
    "designation geometry acceptance topology proof terminal performance "
    "construction-ready guaranteed SLA travel-time proof delivery commitment "
    "numeric ROI ROI eligibility compliance endorsement validation "
    "public-readiness or external-readiness claim"
)


def main() -> None:
    rows = [
        {
            "source_family": "transport_ministry_context",
            "source_id": "CHN-SRC-001",
            "source_path_or_status": "https://www.mot.gov.cn/",
            "owner_or_publisher": "Ministry of Transport of the People's Republic of China",
            "date_accessed": "2026-06-21",
            "required_fields": "transport ministry context; source owner; access note; publication surface",
            "adapter_target": "jurisdiction_scope;governance_ledger",
            "promotion_decision": "source-candidate not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "record transport-ministry source scope before any parser contract",
        },
        {
            "source_family": "transport_plan_context",
            "source_id": "CHN-SRC-002",
            "source_path_or_status": "https://english.www.gov.cn/policies/latestreleases/202201/18/content_WS61e6830fc6d09c94e48a3dd3.html",
            "owner_or_publisher": "State Council; People's Republic of China",
            "date_accessed": "2026-06-21",
            "required_fields": "plan title; plan period; transport scope; access note",
            "adapter_target": "planning_context;governance_ledger",
            "promotion_decision": "source-candidate not parsed",
            "claim_boundary": BLOCKED,
            "next_action": "separate planning-context inventory from policy-alignment claims before extraction",
        },
        {
            "source_family": "transport_statistics_context",
            "source_id": "CHN-SRC-003",
            "source_path_or_status": "https://www.stats.gov.cn/english/",
            "owner_or_publisher": "National Bureau of Statistics of China",
            "date_accessed": "2026-06-21",
            "required_fields": "statistics portal; transport table lead; publication date; access note",
            "adapter_target": "need_surfaces;traffic_context",
            "promotion_decision": "source-candidate not parsed",
            "claim_boundary": BLOCKED,
            "next_action": "select exact transport-statistics table before need or SLA inference",
        },
        {
            "source_family": "highway_standards_context",
            "source_id": "CHN-SRC-004",
            "source_path_or_status": "https://xxgk.mot.gov.cn/jigou/glj/202107/P020210706517905491391.pdf",
            "owner_or_publisher": "Ministry of Transport of the People's Republic of China",
            "date_accessed": "2026-06-21",
            "required_fields": "standard title; issuing authority; standard code; access note",
            "adapter_target": "standards_context;geometry_policy_input",
            "promotion_decision": "source-candidate not accepted",
            "claim_boundary": BLOCKED,
            "next_action": "treat standards as context only before any design, engineering, or geometry claim",
        },
        {
            "source_family": "port_waterway_context",
            "source_id": "CHN-SRC-005",
            "source_path_or_status": "https://english.www.gov.cn/archive/statistics/202304/21/content_WS6441c8fac6d03ffcca6ec7ef.html",
            "owner_or_publisher": "State Council; People's Republic of China",
            "date_accessed": "2026-06-21",
            "required_fields": "waterway transport context; port throughput context; publication date; access note",
            "adapter_target": "node_catalog;need_surfaces",
            "promotion_decision": "source-candidate not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "select port-node or waterway table rows before terminal or node fixture use",
        },
        {
            "source_family": "hierarchy_fixture_context",
            "source_id": "CHN-SRC-006",
            "source_path_or_status": "data/international-china-candidate-hierarchy-v2.csv",
            "owner_or_publisher": "ROUTE internal held-claim fixture",
            "date_accessed": "2026-06-21",
            "required_fields": "candidate tier; service role; readiness basis; evidence label; claim boundary",
            "adapter_target": "dry_run_fixture;gap_backlog",
            "promotion_decision": "heuristic fixture not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "complete source custody, parser contract, row validation, role review, and geometry policy before replacing fixture rows",
        },
        {
            "source_family": "service_targets",
            "source_id": "CHN-SRC-SLA-001",
            "source_path_or_status": "none",
            "owner_or_publisher": "none",
            "date_accessed": "2026-06-21",
            "required_fields": "target id; target class; assumption label; local basis; numeracy review",
            "adapter_target": "service_target_set",
            "promotion_decision": "held",
            "claim_boundary": "no guaranteed SLA travel-time proof delivery commitment official approval construction ROI compliance endorsement validation public-readiness or external-readiness claim",
            "next_action": "keep China service targets assumption-labeled until local evidence and numeracy review close",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
