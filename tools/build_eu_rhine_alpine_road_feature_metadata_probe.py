#!/usr/bin/env python3
"""Build EU Rhine-Alpine road-feature and port-node metadata probe ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-road-feature-metadata-probe-001.csv"

FIELDS = [
    "probe_id",
    "selection_id",
    "source_id",
    "selected_for",
    "probe_surface",
    "probe_url",
    "probe_result",
    "observed_metadata",
    "evidence_acceptance_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "official_network;official_corridor_designation;member_state_approval;"
    "route_designation;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;"
    "endorsement;validation;external_validation;public_readiness;"
    "external_readiness;fixture_replacement;internal_adapter_proof"
)


def main() -> None:
    rows = [
        {
            "probe_id": "EUR-METADATA-PROBE-001",
            "selection_id": "EUR-ROAD-SOURCE-001",
            "source_id": "EUR-SRC-003",
            "selected_for": "road_feature_probe",
            "probe_surface": "GISCO transport networks public page",
            "probe_url": "https://ec.europa.eu/eurostat/web/gisco/geodata/transport-networks",
            "probe_result": "reachable_page_no_direct_road_link_table_in_sample",
            "observed_metadata": "page confirms transport-network download surface; sampled page table exposes airports and ports but not a direct road-link row",
            "evidence_acceptance_status": "not-accepted",
            "allowed_use": "road-source endpoint triage only",
            "blocked_claims": BLOCKED,
            "next_action": "locate exact GISCO Transport version 3 road-link endpoint before source-row extraction",
        },
        {
            "probe_id": "EUR-METADATA-PROBE-002",
            "selection_id": "EUR-ROAD-SOURCE-001",
            "source_id": "EUR-SRC-003",
            "selected_for": "road_feature_probe",
            "probe_surface": "European Commission JRC EIGL data documentation",
            "probe_url": "https://joint-research-centre.ec.europa.eu/document/download/d99c0bcf-21db-46bf-ba66-fd46fdf5e3de_en?filename=Data+documentation+EIGL+PUBLIC+V1_7.pdf",
            "probe_result": "documentation_confirms_gisco_transport_v3_road_links_candidate",
            "observed_metadata": "JRC documentation describes Eurostat GISCO Transport version 3 containing roads and road junctions and using Road links at 1:4 000 000 resolution",
            "evidence_acceptance_status": "not-accepted",
            "allowed_use": "road-source endpoint search lead only",
            "blocked_claims": BLOCKED,
            "next_action": "locate exact GISCO Transport version 3 download or API endpoint before source-row extraction",
        },
        {
            "probe_id": "EUR-METADATA-PROBE-003",
            "selection_id": "EUR-NODE-SOURCE-001",
            "source_id": "EUR-SRC-003",
            "selected_for": "port_node_probe",
            "probe_surface": "GISCO transport networks public page",
            "probe_url": "https://ec.europa.eu/eurostat/web/gisco/geodata/transport-networks",
            "probe_result": "metadata_confirms_ports_2013_download_options",
            "observed_metadata": "page lists Ports 2013 point data at 1:1 million world coverage with GDB and SHP download formats",
            "evidence_acceptance_status": "not-accepted",
            "allowed_use": "port-node metadata probe only",
            "blocked_claims": BLOCKED,
            "next_action": "probe Ports 2013 package metadata before node fixture replacement",
        },
        {
            "probe_id": "EUR-METADATA-PROBE-004",
            "selection_id": "EUR-CORRIDOR-SOURCE-001",
            "source_id": "EUR-SRC-001",
            "selected_for": "scope_rebase_context",
            "probe_surface": "current European Transport Corridors map-library page",
            "probe_url": "https://transport.ec.europa.eu/transport-themes/infrastructure-and-investment/trans-european-transport-network-ten-t/tentec-information-system-and-ten-t-map-library/ten-t-maps-european-transport-corridors_en",
            "probe_result": "scope_context_only_current_corridor_rebase_still_required",
            "observed_metadata": "current corridor publication context can guide rebase but is not road-feature or port-node source custody",
            "evidence_acceptance_status": "not-accepted",
            "allowed_use": "scope rebase context only",
            "blocked_claims": BLOCKED,
            "next_action": "choose current corridor scope before fixture replacement",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
