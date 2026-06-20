#!/usr/bin/env python3
"""Build EU Rhine-Alpine adapter source-pack preflight ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-adapter-source-pack-001.csv"

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
    "no official EU corridor designation member-state approval route designation "
    "geometry acceptance topology proof terminal performance construction-ready "
    "guaranteed SLA ROI eligibility compliance endorsement validation "
    "public-readiness or external-readiness claim"
)


def main() -> None:
    rows = [
        {
            "source_family": "corridor_context",
            "source_id": "EUR-SRC-001",
            "source_path_or_status": "https://transport.ec.europa.eu/transport-themes/infrastructure-and-investment/trans-european-transport-network-ten-t/tentec-information-system-and-ten-t-map-library/ten-t-maps-european-transport-corridors_en",
            "owner_or_publisher": "European Commission; Mobility and Transport",
            "date_accessed": "2026-06-20",
            "required_fields": "corridor name; map publication date; corridor scope; access note",
            "adapter_target": "jurisdiction_scope;governance_ledger",
            "promotion_decision": "source-candidate not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "record corridor vocabulary and map-library limits before any parser contract",
        },
        {
            "source_family": "network_viewer",
            "source_id": "EUR-SRC-002",
            "source_path_or_status": "https://webgate.ec.europa.eu/tentec-maps/web/public/",
            "owner_or_publisher": "European Commission; TENtec",
            "date_accessed": "2026-06-20",
            "required_fields": "TEN-T network layer; corridor layer; node/layer metadata; access note",
            "adapter_target": "road_graph;node_catalog;governance_ledger",
            "promotion_decision": "source-candidate not parsed",
            "claim_boundary": BLOCKED,
            "next_action": "identify downloadable or inspectable layer metadata before source-row validation",
        },
        {
            "source_family": "transport_geodata",
            "source_id": "EUR-SRC-003",
            "source_path_or_status": "https://ec.europa.eu/eurostat/web/gisco/geodata/transport-networks",
            "owner_or_publisher": "Eurostat GISCO; European Commission",
            "date_accessed": "2026-06-20",
            "required_fields": "transport network dataset; ports; airports; scale; download format; access note",
            "adapter_target": "road_graph;node_catalog",
            "promotion_decision": "source-candidate not parsed",
            "claim_boundary": BLOCKED,
            "next_action": "select dataset version and suitability warning before any geometry or node replacement",
        },
        {
            "source_family": "rhine_alpine_context",
            "source_id": "EUR-SRC-004",
            "source_path_or_status": "https://transport.ec.europa.eu/transport-modes/rail/ertms/who-involved-ertms-deployment/corridors/rhine-alpine-corridor_en",
            "owner_or_publisher": "European Commission; Mobility and Transport",
            "date_accessed": "2026-06-20",
            "required_fields": "Rhine-Alpine context; countries crossed; corridor description; access note",
            "adapter_target": "need_surfaces;governance_ledger",
            "promotion_decision": "context-source candidate not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "record bounded corridor vocabulary before any road or service inference use",
        },
        {
            "source_family": "rail_freight_context",
            "source_id": "EUR-SRC-005",
            "source_path_or_status": "https://www.corridor-rhine-alpine.eu/",
            "owner_or_publisher": "Rhine-Alpine Rail Freight Corridor organization",
            "date_accessed": "2026-06-20",
            "required_fields": "freight corridor context; organization scope; access note",
            "adapter_target": "need_surfaces;terminal_access",
            "promotion_decision": "context-source candidate not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "separate rail-freight context from road-network inference before any adapter use",
        },
        {
            "source_family": "service_targets",
            "source_id": "EUR-SRC-SLA-001",
            "source_path_or_status": "none",
            "owner_or_publisher": "none",
            "date_accessed": "2026-06-20",
            "required_fields": "target id; role; target hours; basis; assumption label",
            "adapter_target": "service_target_set",
            "promotion_decision": "held",
            "claim_boundary": "no guaranteed SLA travel-time proof delivery commitment official approval construction ROI compliance endorsement validation public-readiness or external-readiness claim",
            "next_action": "keep EU service targets assumption-labeled until local evidence and numeracy review close",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
