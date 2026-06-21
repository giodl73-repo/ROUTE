#!/usr/bin/env python3
"""Build bounded China source-content sample rows."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-china-source-content-sample-001.csv"

FIELDS = [
    "sample_id",
    "source_id",
    "source_family",
    "source_url_or_status",
    "sample_basis",
    "content_summary",
    "route_or_dataset_hint",
    "source_owner",
    "source_date",
    "sample_status",
    "evidence_label",
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
    "external_readiness;internal_adapter_proof"
)


def main() -> None:
    rows = [
        {
            "sample_id": "CHN-CONTENT-001",
            "source_id": "CHN-SRC-001",
            "source_family": "transport_ministry_context",
            "source_url_or_status": "https://www.mot.gov.cn/",
            "sample_basis": "source-pack URL candidate; payload access cache candidate; evidence not accepted",
            "content_summary": "MOT homepage context can support transport-ministry source ownership and publication-surface vocabulary only; this sample does not accept road rows, route designations, or policy alignment.",
            "route_or_dataset_hint": "transport_ministry_context_not_link_rows",
            "source_owner": "Ministry of Transport of the People's Republic of China",
            "source_date": "2026-06-21",
            "sample_status": "source_content_sampled_context_only",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "select exact MOT road-network, highway, or data table pages before source-row extraction",
        },
        {
            "sample_id": "CHN-CONTENT-002",
            "source_id": "CHN-SRC-002",
            "source_family": "transport_plan_context",
            "source_url_or_status": "https://english.www.gov.cn/policies/latestreleases/202201/18/content_WS61e6830fc6d09c94e48a3dd3.html",
            "sample_basis": "source-pack URL candidate; payload access cache candidate; evidence not accepted",
            "content_summary": "State Council transport-plan context can support bounded planning vocabulary only; this sample does not promote policy alignment, priority corridors, or construction readiness.",
            "route_or_dataset_hint": "planning_context_not_policy_alignment",
            "source_owner": "State Council; People's Republic of China",
            "source_date": "2026-06-21",
            "sample_status": "source_content_sampled_context_only",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "separate plan context from policy-alignment claims before parser extraction",
        },
        {
            "sample_id": "CHN-CONTENT-003",
            "source_id": "CHN-SRC-003",
            "source_family": "transport_statistics_context",
            "source_url_or_status": "https://www.stats.gov.cn/english/",
            "sample_basis": "source-pack URL candidate; payload access cache candidate; evidence not accepted",
            "content_summary": "NBS statistics portal context can support later transport table discovery only; this sample does not parse statistics rows, forecast demand, or support SLA or ROI inference.",
            "route_or_dataset_hint": "transport_statistics_table_selection_needed",
            "source_owner": "National Bureau of Statistics of China",
            "source_date": "2026-06-21",
            "sample_status": "source_content_sampled_inventory_needed",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "select exact transport-statistics tables before need, service-target, or ROI inference",
        },
        {
            "sample_id": "CHN-CONTENT-004",
            "source_id": "CHN-SRC-004",
            "source_family": "highway_standards_context",
            "source_url_or_status": "https://xxgk.mot.gov.cn/jigou/glj/202107/P020210706517905491391.pdf",
            "sample_basis": "source-pack PDF candidate; payload access cache candidate; evidence not accepted",
            "content_summary": "Highway standards context can support future standards vocabulary and geometry-policy review only; this sample does not parse design geometry or engineering requirements for ROUTE use.",
            "route_or_dataset_hint": "standards_context_not_design_geometry",
            "source_owner": "Ministry of Transport of the People's Republic of China",
            "source_date": "2026-06-21",
            "sample_status": "source_content_sampled_context_only",
            "evidence_label": "context-only",
            "blocked_claims": BLOCKED,
            "next_action": "keep standards context separate before engineering, geometry, or fixture-replacement claims",
        },
        {
            "sample_id": "CHN-CONTENT-005",
            "source_id": "CHN-SRC-005",
            "source_family": "port_waterway_context",
            "source_url_or_status": "https://english.www.gov.cn/archive/statistics/202304/21/content_WS6441c8fac6d03ffcca6ec7ef.html",
            "sample_basis": "source-pack URL candidate; payload access cache candidate; evidence not accepted",
            "content_summary": "State Council port and waterway statistics context can support later port-node and waterway table inventory only; this sample does not validate nodes, terminal performance, throughput, or road access.",
            "route_or_dataset_hint": "port_waterway_table_inventory_needed",
            "source_owner": "State Council; People's Republic of China",
            "source_date": "2026-06-21",
            "sample_status": "source_content_sampled_inventory_needed",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "select port-node or waterway table rows before terminal, throughput, node, or access fixture use",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
