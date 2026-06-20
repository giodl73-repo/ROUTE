#!/usr/bin/env python3
"""Build EU Rhine-Alpine road-feature and node source-selection ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-road-feature-source-selection-001.csv"

FIELDS = [
    "selection_id",
    "source_id",
    "source_family",
    "selected_for",
    "source_url",
    "source_owner",
    "source_date",
    "observed_source_capability",
    "selection_decision",
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
            "selection_id": "EUR-ROAD-SOURCE-001",
            "source_id": "EUR-SRC-003",
            "source_family": "transport_geodata",
            "selected_for": "road_feature_probe",
            "source_url": "https://ec.europa.eu/eurostat/web/gisco/geodata/transport-networks",
            "source_owner": "Eurostat GISCO; European Commission",
            "source_date": "2026-06-20",
            "observed_source_capability": "GISCO transport networks include downloadable transport datasets and referenced road-link layers in Transport version 3 context",
            "selection_decision": "selected_for_next_probe_only",
            "allowed_use": "source-access planning and no-geometry metadata probe only",
            "blocked_claims": BLOCKED,
            "next_action": "probe GISCO road-link dataset metadata before source-row extraction",
        },
        {
            "selection_id": "EUR-NODE-SOURCE-001",
            "source_id": "EUR-SRC-003",
            "source_family": "transport_geodata",
            "selected_for": "port_node_probe",
            "source_url": "https://ec.europa.eu/eurostat/web/gisco/geodata/transport-networks",
            "source_owner": "Eurostat GISCO; European Commission",
            "source_date": "2026-06-20",
            "observed_source_capability": "GISCO transport networks page lists Ports 2013 point data with GDB and SHP download formats",
            "selection_decision": "selected_for_next_probe_only",
            "allowed_use": "source-access planning and no-geometry node metadata probe only",
            "blocked_claims": BLOCKED,
            "next_action": "probe GISCO ports dataset metadata before node fixture replacement",
        },
        {
            "selection_id": "EUR-CORRIDOR-SOURCE-001",
            "source_id": "EUR-SRC-001",
            "source_family": "corridor_context",
            "selected_for": "scope_rebase_context",
            "source_url": "https://transport.ec.europa.eu/transport-themes/infrastructure-and-investment/trans-european-transport-network-ten-t/tentec-information-system-and-ten-t-map-library/ten-t-maps-european-transport-corridors_en",
            "source_owner": "European Commission; Mobility and Transport",
            "source_date": "2026-06-20",
            "observed_source_capability": "current European Transport Corridors map-library context can inform scope rebase but not road-feature rows by itself",
            "selection_decision": "selected_for_scope_rebase_not_feature_rows",
            "allowed_use": "scope decision and citation context only",
            "blocked_claims": BLOCKED,
            "next_action": "choose current corridor scope before fixture replacement",
        },
        {
            "selection_id": "EUR-LEGACY-CONTEXT-001",
            "source_id": "EUR-SRC-004",
            "source_family": "rhine_alpine_context",
            "selected_for": "legacy_context_only",
            "source_url": "https://transport.ec.europa.eu/transport-modes/rail/ertms/who-involved-ertms-deployment/corridors/rhine-alpine-corridor_en",
            "source_owner": "European Commission; Mobility and Transport",
            "source_date": "2026-06-20",
            "observed_source_capability": "Rhine-Alpine page gives bounded corridor context but is not a road-service feature source",
            "selection_decision": "not_selected_for_road_feature_probe",
            "allowed_use": "legacy context citation only",
            "blocked_claims": BLOCKED,
            "next_action": "do not use legacy corridor context as road-feature proof before fixture replacement",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
