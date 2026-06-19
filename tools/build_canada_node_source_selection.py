#!/usr/bin/env python3
"""Build Canada node source-selection candidates.

This selects public port/terminal source rows for the Canada node-catalog gap.
It does not replace node fixtures, prove node completeness, or promote any
terminal, access-road, throughput, service, SLA, ROI, approval, or validation
claim.
"""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-canada-node-source-selection-001.csv"

FIELDS = [
    "selection_id",
    "source_id",
    "node_id",
    "node_label",
    "node_class",
    "source_url",
    "source_owner",
    "source_date",
    "selected_fields",
    "selection_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

BLOCKED_CLAIMS = (
    "port_endorsement;terminal_performance;node_completeness;"
    "throughput_proof;road_access_proof;construction_ready;guaranteed_sla;"
    "roi;compliance;endorsement;validation;public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "selection_id": "CAN-NODE-SOURCE-001",
            "source_id": "CAN-SRC-005",
            "node_id": "CAN-PORT-VANCOUVER",
            "node_label": "Port of Vancouver",
            "node_class": "port_gateway",
            "source_url": "https://www.portvancouver.com/",
            "source_owner": "Vancouver Fraser Port Authority",
            "source_date": "2026-06-19",
            "selected_fields": "node label; port authority source owner; source url; access note",
            "selection_status": "source-selected-not-promoted",
            "allowed_use": "node source-custody candidate only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "inspect terminal facility pages before replacing node fixture rows",
        },
        {
            "selection_id": "CAN-NODE-SOURCE-002",
            "source_id": "CAN-SRC-005",
            "node_id": "CAN-PORT-MONTREAL",
            "node_label": "Port of Montreal",
            "node_class": "port_gateway",
            "source_url": "https://www.port-montreal.com/en/goods/operations/map-of-port-facilities",
            "source_owner": "Montreal Port Authority",
            "source_date": "2026-06-19",
            "selected_fields": "node label; port authority source owner; source url; access note",
            "selection_status": "source-selected-not-promoted",
            "allowed_use": "node source-custody candidate only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "inspect terminal facility pages before replacing node fixture rows",
        },
        {
            "selection_id": "CAN-NODE-SOURCE-003",
            "source_id": "CAN-SRC-005",
            "node_id": "CAN-PORT-HALIFAX",
            "node_label": "Port of Halifax",
            "node_class": "port_gateway",
            "source_url": "https://www.porthalifax.ca/facilities/hpa-facilities/psa-halifax-south-end-container-terminal/",
            "source_owner": "Halifax Port Authority",
            "source_date": "2026-06-19",
            "selected_fields": "node label; port authority source owner; source url; access note",
            "selection_status": "source-selected-not-promoted",
            "allowed_use": "node source-custody candidate only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "inspect terminal facility pages before replacing node fixture rows",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
