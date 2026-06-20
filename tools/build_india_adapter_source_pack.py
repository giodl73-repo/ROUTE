#!/usr/bin/env python3
"""Build India adapter source-pack preflight ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-india-adapter-source-pack-001.csv"

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
    "no official Indian corridor designation national approval state approval "
    "route designation geometry acceptance topology proof terminal performance "
    "construction-ready guaranteed SLA travel-time proof delivery commitment "
    "numeric ROI ROI eligibility compliance endorsement validation "
    "public-readiness or external-readiness claim"
)


def main() -> None:
    rows = [
        {
            "source_family": "highway_ministry_context",
            "source_id": "IND-SRC-001",
            "source_path_or_status": "https://morth.nic.in/",
            "owner_or_publisher": "Ministry of Road Transport and Highways; Government of India",
            "date_accessed": "2026-06-20",
            "required_fields": "national highway program context; publication date; access note; source owner",
            "adapter_target": "jurisdiction_scope;governance_ledger",
            "promotion_decision": "source-candidate not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "record ministry source scope and document inventory before any parser contract",
        },
        {
            "source_family": "highway_authority_context",
            "source_id": "IND-SRC-002",
            "source_path_or_status": "https://nhai.gov.in/",
            "owner_or_publisher": "National Highways Authority of India",
            "date_accessed": "2026-06-20",
            "required_fields": "highway asset context; network responsibility; source metadata; access note",
            "adapter_target": "road_graph;governance_ledger",
            "promotion_decision": "source-candidate not parsed",
            "claim_boundary": BLOCKED,
            "next_action": "identify downloadable or inspectable road-network attributes before source-row validation",
        },
        {
            "source_family": "port_system_context",
            "source_id": "IND-SRC-003",
            "source_path_or_status": "https://shipmin.gov.in/en/division/ports-wing",
            "owner_or_publisher": "Ministry of Ports, Shipping and Waterways; Government of India",
            "date_accessed": "2026-06-20",
            "required_fields": "major port list; port governance context; access note; source owner",
            "adapter_target": "node_catalog;terminal_access",
            "promotion_decision": "source-candidate not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "separate port-node source custody from terminal performance before node fixture use",
        },
        {
            "source_family": "port_statistics_context",
            "source_id": "IND-SRC-004",
            "source_path_or_status": "https://shipmin.gov.in/en/transport-reseach/basic-port-statistics",
            "owner_or_publisher": "Ministry of Ports, Shipping and Waterways; Government of India",
            "date_accessed": "2026-06-20",
            "required_fields": "port statistics publication; port name; cargo context; publication period; access note",
            "adapter_target": "need_surfaces;node_catalog",
            "promotion_decision": "source-candidate not parsed",
            "claim_boundary": BLOCKED,
            "next_action": "inventory available tables and field names before any port-node or throughput inference",
        },
        {
            "source_family": "hierarchy_fixture_context",
            "source_id": "IND-SRC-005",
            "source_path_or_status": "data/international-india-candidate-hierarchy-v2.csv",
            "owner_or_publisher": "ROUTE internal held-claim fixture",
            "date_accessed": "2026-06-20",
            "required_fields": "candidate tier; service role; readiness basis; evidence label; claim boundary",
            "adapter_target": "dry_run_fixture;gap_backlog",
            "promotion_decision": "heuristic fixture not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "complete source custody, parser contract, row validation, role review, and geometry policy before replacing fixture rows",
        },
        {
            "source_family": "service_targets",
            "source_id": "IND-SRC-SLA-001",
            "source_path_or_status": "none",
            "owner_or_publisher": "none",
            "date_accessed": "2026-06-20",
            "required_fields": "target id; target class; assumption label; local basis; numeracy review",
            "adapter_target": "service_target_set",
            "promotion_decision": "held",
            "claim_boundary": "no guaranteed SLA travel-time proof delivery commitment official approval construction ROI compliance endorsement validation public-readiness or external-readiness claim",
            "next_action": "keep India service targets assumption-labeled until local evidence and numeracy review close",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
