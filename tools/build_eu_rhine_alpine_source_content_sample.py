#!/usr/bin/env python3
"""Build bounded EU Rhine-Alpine source-content sample rows."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-source-content-sample-001.csv"

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
    "official_route_designation;member_state_approval;route_designation;"
    "geometry_acceptance;topology_proof;map_overlay;terminal_performance;"
    "node_completeness;road_access_proof;construction_ready;guaranteed_sla;"
    "travel_time_proof;delivery_commitment;numeric_roi;roi;eligibility;"
    "compliance;endorsement;validation;external_validation;public_readiness;"
    "external_readiness;internal_adapter_proof"
)


def main() -> None:
    rows = [
        {
            "sample_id": "EUR-CONTENT-001",
            "source_id": "EUR-SRC-001",
            "source_family": "corridor_context",
            "source_url": "https://transport.ec.europa.eu/transport-themes/infrastructure-and-investment/trans-european-transport-network-ten-t/tentec-information-system-and-ten-t-map-library/ten-t-maps-european-transport-corridors_en",
            "source_line_ref": "official page lines 116-156",
            "content_summary": "current European Transport Corridors map-library rows are dated 2 July 2024 and list current corridor publications including North Sea - Rhine - Mediterranean",
            "route_or_dataset_hint": "current_corridor_set_rebase_needed",
            "source_owner": "European Commission; Mobility and Transport",
            "source_date": "2026-06-20",
            "sample_status": "source_content_sampled_rebase_needed",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "decide current European Transport Corridor rebase before any Rhine-Alpine fixture replacement",
        },
        {
            "sample_id": "EUR-CONTENT-002",
            "source_id": "EUR-SRC-002",
            "source_family": "network_viewer",
            "source_url": "https://transport.ec.europa.eu/transport-themes/infrastructure-and-investment/trans-european-transport-network-ten-t/tentec-information-system-and-ten-t-map-library_en",
            "source_line_ref": "official page lines 115-120",
            "content_summary": "TENtec is described as the Commission information system for TEN-T policy support, public interactive maps, reports, maps, and API exchange",
            "route_or_dataset_hint": "tentec_network_viewer_and_api_context",
            "source_owner": "European Commission; Mobility and Transport",
            "source_date": "2026-06-20",
            "sample_status": "source_content_sampled_not_parsed_layer",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "select inspectable TENtec layer fields before source-derived road rows",
        },
        {
            "sample_id": "EUR-CONTENT-003",
            "source_id": "EUR-SRC-003",
            "source_family": "transport_geodata",
            "source_url": "https://ec.europa.eu/eurostat/web/gisco/geodata/transport-networks",
            "source_line_ref": "official page lines 183-197",
            "content_summary": "GISCO transport networks page lists downloadable transport datasets and formats, including Ports 2013 point data with GDB and SHP options",
            "route_or_dataset_hint": "ports_dataset_node_source_candidate",
            "source_owner": "Eurostat GISCO; European Commission",
            "source_date": "2026-06-20",
            "sample_status": "source_content_sampled_node_dataset_candidate",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "select dataset version and node custody fields before node fixture replacement",
        },
        {
            "sample_id": "EUR-CONTENT-004",
            "source_id": "EUR-SRC-004",
            "source_family": "rhine_alpine_context",
            "source_url": "https://transport.ec.europa.eu/transport-modes/rail/ertms/who-involved-ertms-deployment/corridors/rhine-alpine-corridor_en",
            "source_line_ref": "official page lines 117-121",
            "content_summary": "the Rhine - Alpine corridor context page describes a route crossing the Netherlands, Belgium, Germany, Switzerland, and Italy and connecting Rotterdam and Antwerp to Genoa",
            "route_or_dataset_hint": "rhine_alpine_rail_corridor_context_not_road_service_network",
            "source_owner": "European Commission; Mobility and Transport",
            "source_date": "2026-06-20",
            "sample_status": "source_content_sampled_context_only",
            "evidence_label": "source-candidate",
            "blocked_claims": BLOCKED,
            "next_action": "separate rail corridor context from road service inference before link fixture replacement",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
