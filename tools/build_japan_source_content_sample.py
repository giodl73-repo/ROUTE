#!/usr/bin/env python3
"""Build bounded Japan source-content sample rows."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-japan-source-content-sample-001.csv"

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
    "official_corridor_designation;ministry_approval;route_designation;"
    "source_row_validation;fixture_replacement;parsed_adapter;"
    "geometry_acceptance;topology_proof;map_overlay;disaster_readiness;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;"
    "travel_time_proof;delivery_commitment;numeric_roi;roi;eligibility;"
    "compliance;endorsement;validation;external_validation;"
    "public_readiness;external_readiness;internal_adapter_proof"
)


def main() -> None:
    rows = [
        {
            "sample_id": "JPN-CONTENT-001",
            "source_id": "JPN-SRC-001",
            "source_family": "road_bureau_context",
            "source_url_or_status": "https://www.mlit.go.jp/road/road_e/index2_e.html",
            "sample_basis": "payload probe http 200; field inventory candidate; evidence not accepted",
            "content_summary": "MLIT Road Bureau context is reachable as a governance and road-administration source, but this sample does not accept an official corridor row, route designation, or parser field.",
            "route_or_dataset_hint": "road_bureau_context_not_link_rows",
            "source_owner": "Road Bureau; Ministry of Land, Infrastructure, Transport and Tourism; Japan",
            "source_date": "2026-06-21",
            "sample_status": "source_content_sampled_context_only",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "identify exact road-network tables or documents before source-row extraction",
        },
        {
            "sample_id": "JPN-CONTENT-002",
            "source_id": "JPN-SRC-002",
            "source_family": "road_statistics_context",
            "source_url_or_status": "https://www.mlit.go.jp/road/road_e/statistics.html",
            "sample_basis": "payload probe http 200; field inventory candidate; evidence not accepted",
            "content_summary": "MLIT road-statistics context is reachable as a possible road length and category source, but this sample does not parse a table or infer service need.",
            "route_or_dataset_hint": "road_statistics_table_inventory_needed",
            "source_owner": "Road Bureau; Ministry of Land, Infrastructure, Transport and Tourism; Japan",
            "source_date": "2026-06-21",
            "sample_status": "source_content_sampled_inventory_needed",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "inventory road-statistics table structure before need, graph, or parser mapping",
        },
        {
            "sample_id": "JPN-CONTENT-003",
            "source_id": "JPN-SRC-003",
            "source_family": "road_traffic_census_context",
            "source_url_or_status": "https://www.e-stat.go.jp/en/statistics/00600580",
            "sample_basis": "payload probe http 200; field inventory candidate; evidence not accepted",
            "content_summary": "e-Stat Road Traffic Census context is reachable as a traffic-source candidate, but this sample does not extract census rows, forecast demand, or support SLA inference.",
            "route_or_dataset_hint": "traffic_census_table_selection_needed",
            "source_owner": "e-Stat; Ministry of Land, Infrastructure, Transport and Tourism; Japan",
            "source_date": "2026-06-21",
            "sample_status": "source_content_sampled_inventory_needed",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "select extractable census tables before traffic, need, or service-target inference",
        },
        {
            "sample_id": "JPN-CONTENT-004",
            "source_id": "JPN-SRC-004",
            "source_family": "geospatial_road_context",
            "source_url_or_status": "https://www.gsi.go.jp/kankyochiri/gm_japan_e.html",
            "sample_basis": "payload probe failed with URLError; field inventory source-needed; evidence not accepted",
            "content_summary": "GSI transportation context remains unresolved for the current gate because the bounded probe did not produce a usable metadata sample.",
            "route_or_dataset_hint": "usable_geospatial_metadata_source_needed",
            "source_owner": "Geospatial Information Authority of Japan",
            "source_date": "2026-06-21",
            "sample_status": "source_content_not_sampled_source_needed",
            "evidence_label": "source-needed",
            "blocked_claims": BLOCKED,
            "next_action": "resolve a usable GSI or alternative road-feature metadata source before geometry, topology, map, or fixture promotion",
        },
        {
            "sample_id": "JPN-CONTENT-005",
            "source_id": "JPN-SRC-005",
            "source_family": "port_system_context",
            "source_url_or_status": "https://www.mlit.go.jp/en/kowan/index.html",
            "sample_basis": "payload probe http 200; field inventory candidate; evidence not accepted",
            "content_summary": "MLIT port-system context is reachable as a port-governance source candidate, but this sample does not establish port-node completeness, terminal performance, or road-access proof.",
            "route_or_dataset_hint": "port_node_source_selection_needed",
            "source_owner": "Ports and Harbours Bureau; Ministry of Land, Infrastructure, Transport and Tourism; Japan",
            "source_date": "2026-06-21",
            "sample_status": "source_content_sampled_context_only",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "select exact port-node rows before terminal, node, or access fixture use",
        },
        {
            "sample_id": "JPN-CONTENT-006",
            "source_id": "JPN-SRC-006",
            "source_family": "port_classification_context",
            "source_url_or_status": "https://www.mlit.go.jp/en/kowan/kowan_fr4_000004.html",
            "sample_basis": "payload probe http 200; field inventory candidate; evidence not accepted",
            "content_summary": "MLIT port-classification context is reachable as a governance and port-type source candidate, but this sample does not promote any port node or service obligation.",
            "route_or_dataset_hint": "port_classification_mapping_needed",
            "source_owner": "Ports and Harbours Bureau; Ministry of Land, Infrastructure, Transport and Tourism; Japan",
            "source_date": "2026-06-21",
            "sample_status": "source_content_sampled_context_only",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "map classification terms to candidate node-review fields before port-node promotion",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
