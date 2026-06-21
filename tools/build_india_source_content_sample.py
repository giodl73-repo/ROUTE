#!/usr/bin/env python3
"""Build bounded India source-content sample rows."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-india-source-content-sample-001.csv"

FIELDS = [
    "sample_id",
    "source_id",
    "source_family",
    "source_url",
    "source_line_ref",
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
    "official_corridor_designation;national_approval;state_approval;"
    "route_designation;source_row_validation;fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;"
    "travel_time_proof;delivery_commitment;numeric_roi;roi;eligibility;"
    "compliance;endorsement;validation;external_validation;"
    "public_readiness;external_readiness;internal_adapter_proof"
)


def main() -> None:
    rows = [
        {
            "sample_id": "IND-CONTENT-001",
            "source_id": "IND-SRC-001",
            "source_family": "highway_ministry_context",
            "source_url": "https://morth.nic.in/",
            "source_line_ref": "official home page reachable through redirect; no row-level highway table accepted",
            "content_summary": "MoRTH home source is reachable as ministry context, but this sample does not accept a road-network row or route field from the landing page",
            "route_or_dataset_hint": "document_inventory_needed_before_highway_rows",
            "source_owner": "Ministry of Road Transport and Highways; Government of India",
            "source_date": "2026-06-20",
            "sample_status": "source_content_sampled_inventory_needed",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "select a specific MoRTH report, notification, or table before any highway source-row extraction",
        },
        {
            "sample_id": "IND-CONTENT-002",
            "source_id": "IND-SRC-002",
            "source_family": "highway_authority_context",
            "source_url": "https://nhai.gov.in/nhai/sites/default/files/2025-09/NHAI-Annual_Report_2023-24_English.pdf",
            "source_line_ref": "official NHAI annual report search extract",
            "content_summary": "NHAI annual-report context describes NHAI as responsible for development, maintenance, and management of National Highways entrusted to it by the Central Government",
            "route_or_dataset_hint": "authority_context_not_road_link_rows",
            "source_owner": "National Highways Authority of India",
            "source_date": "2026-06-20",
            "sample_status": "source_content_sampled_context_only",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "identify inspectable NHAI road-link or route-attribute rows before fixture replacement",
        },
        {
            "sample_id": "IND-CONTENT-003",
            "source_id": "IND-SRC-003",
            "source_family": "port_system_context",
            "source_url": "https://shipmin.gov.in/en/division/ports-wing",
            "source_line_ref": "official page lines 116-150",
            "content_summary": "Ports Wing page records major-port governance context and lists major port authorities including Chennai, Cochin, Deendayal, Jawaharlal Nehru, Paradip, Kolkata, Mormugao, Mumbai, New Mangalore, Visakhapatnam, V.O. Chidambarnar, and Kamarajar",
            "route_or_dataset_hint": "major_port_node_candidates_not_validated",
            "source_owner": "Ministry of Ports, Shipping and Waterways; Government of India",
            "source_date": "2026-06-20",
            "sample_status": "source_content_sampled_node_candidate_list",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "map selected major-port names to node-candidate rows before any node fixture use",
        },
        {
            "sample_id": "IND-CONTENT-004",
            "source_id": "IND-SRC-004",
            "source_family": "port_statistics_context",
            "source_url": "https://shipmin.gov.in/en/transport-reseach/basic-port-statistics",
            "source_line_ref": "official page lines 115-122",
            "content_summary": "Basic Port Statistics page exposes a Basic Port Statistics of India 2024-25 publication link suitable for a later table inventory",
            "route_or_dataset_hint": "port_statistics_publication_candidate",
            "source_owner": "Ministry of Ports, Shipping and Waterways; Government of India",
            "source_date": "2026-06-20",
            "sample_status": "source_content_sampled_publication_candidate",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "inventory publication tables before any throughput, need, or service-target inference",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
