#!/usr/bin/env python3
"""Build Japan adapter source-pack preflight ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-japan-adapter-source-pack-001.csv"

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
    "no official Japanese corridor designation ministry approval route "
    "designation geometry acceptance topology proof disaster-readiness "
    "terminal performance construction-ready guaranteed SLA travel-time proof "
    "delivery commitment numeric ROI ROI eligibility compliance endorsement "
    "validation public-readiness or external-readiness claim"
)


def main() -> None:
    rows = [
        {
            "source_family": "road_bureau_context",
            "source_id": "JPN-SRC-001",
            "source_path_or_status": "https://www.mlit.go.jp/road/road_e/index2_e.html",
            "owner_or_publisher": "Road Bureau; Ministry of Land, Infrastructure, Transport and Tourism; Japan",
            "date_accessed": "2026-06-21",
            "required_fields": "road administration context; publication title; source owner; access note",
            "adapter_target": "jurisdiction_scope;governance_ledger",
            "promotion_decision": "source-candidate not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "record road-bureau source scope before any parser contract",
        },
        {
            "source_family": "road_statistics_context",
            "source_id": "JPN-SRC-002",
            "source_path_or_status": "https://www.mlit.go.jp/road/road_e/statistics.html",
            "owner_or_publisher": "Road Bureau; Ministry of Land, Infrastructure, Transport and Tourism; Japan",
            "date_accessed": "2026-06-21",
            "required_fields": "road length category; statistic date; source report; access note",
            "adapter_target": "need_surfaces;road_graph_context",
            "promotion_decision": "source-candidate not parsed",
            "claim_boundary": BLOCKED,
            "next_action": "inventory table fields before any source-row validation",
        },
        {
            "source_family": "road_traffic_census_context",
            "source_id": "JPN-SRC-003",
            "source_path_or_status": "https://www.e-stat.go.jp/en/statistics/00600580",
            "owner_or_publisher": "e-Stat; Ministry of Land, Infrastructure, Transport and Tourism; Japan",
            "date_accessed": "2026-06-21",
            "required_fields": "statistics code; census title; ministry in charge; access note",
            "adapter_target": "traffic_context;need_surfaces",
            "promotion_decision": "source-candidate not parsed",
            "claim_boundary": BLOCKED,
            "next_action": "select extractable census tables before any traffic or SLA inference",
        },
        {
            "source_family": "geospatial_road_context",
            "source_id": "JPN-SRC-004",
            "source_path_or_status": "https://www.gsi.go.jp/kankyochiri/gm_japan_e.html",
            "owner_or_publisher": "Geospatial Information Authority of Japan",
            "date_accessed": "2026-06-21",
            "required_fields": "transportation layer; dataset version; download format; access note",
            "adapter_target": "road_graph;geometry_intake_candidate",
            "promotion_decision": "source-candidate not accepted",
            "claim_boundary": BLOCKED,
            "next_action": "separate geometry intake policy from no-geometry parser contract before any map or fixture use",
        },
        {
            "source_family": "port_system_context",
            "source_id": "JPN-SRC-005",
            "source_path_or_status": "https://www.mlit.go.jp/en/kowan/index.html",
            "owner_or_publisher": "Ports and Harbours Bureau; Ministry of Land, Infrastructure, Transport and Tourism; Japan",
            "date_accessed": "2026-06-21",
            "required_fields": "ports bureau context; port policy page; access note; source owner",
            "adapter_target": "node_catalog;terminal_access",
            "promotion_decision": "source-candidate not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "select port-node source rows before terminal or node fixture use",
        },
        {
            "source_family": "port_classification_context",
            "source_id": "JPN-SRC-006",
            "source_path_or_status": "https://www.mlit.go.jp/en/kowan/kowan_fr4_000004.html",
            "owner_or_publisher": "Ports and Harbours Bureau; Ministry of Land, Infrastructure, Transport and Tourism; Japan",
            "date_accessed": "2026-06-21",
            "required_fields": "port classification; legal context; port type; access note",
            "adapter_target": "node_catalog;governance_ledger",
            "promotion_decision": "source-candidate not parsed",
            "claim_boundary": BLOCKED,
            "next_action": "map classification fields before any port-node candidate promotion",
        },
        {
            "source_family": "hierarchy_fixture_context",
            "source_id": "JPN-SRC-007",
            "source_path_or_status": "data/international-japan-candidate-hierarchy-v2.csv",
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
            "source_id": "JPN-SRC-SLA-001",
            "source_path_or_status": "none",
            "owner_or_publisher": "none",
            "date_accessed": "2026-06-21",
            "required_fields": "target id; target class; assumption label; local basis; numeracy review",
            "adapter_target": "service_target_set",
            "promotion_decision": "held",
            "claim_boundary": "no guaranteed SLA travel-time proof delivery commitment official approval construction ROI compliance endorsement validation public-readiness or external-readiness claim",
            "next_action": "keep Japan service targets assumption-labeled until local evidence and numeracy review close",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
