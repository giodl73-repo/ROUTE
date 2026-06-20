#!/usr/bin/env python3
"""Build EU Rhine-Alpine GISCO Ports 2013 node field-mapping ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-port-node-field-mapping-001.csv"

FIELDS = [
    "mapping_id",
    "source_table",
    "source_field",
    "candidate_contract_field",
    "mapping_status",
    "evidence_source",
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
            "mapping_id": "EUR-PORT-FIELD-001",
            "source_table": "PORT_PT_2013.dbf",
            "source_field": "PORT_ID",
            "candidate_contract_field": "node_source_id",
            "mapping_status": "field_header_mappable_records_unchecked",
            "evidence_source": "data/international-eu-rhine-alpine-port-package-manifest-001.csv",
            "allowed_use": "node candidate schema planning only",
            "blocked_claims": BLOCKED,
            "next_action": "read bounded attribute records before node candidate selection",
        },
        {
            "mapping_id": "EUR-PORT-FIELD-002",
            "source_table": "PORT_AT_2013.dbf",
            "source_field": "PORT_NAME;NAME_ASCI",
            "candidate_contract_field": "node_name",
            "mapping_status": "field_header_mappable_records_unchecked",
            "evidence_source": "manual DBF header inspection from GISCO Ports 2013 SHP package",
            "allowed_use": "node candidate schema planning only",
            "blocked_claims": BLOCKED,
            "next_action": "read bounded attribute records before node candidate selection",
        },
        {
            "mapping_id": "EUR-PORT-FIELD-003",
            "source_table": "PORT_AT_2013.dbf",
            "source_field": "CNTR_CODE;NUTS_CODE",
            "candidate_contract_field": "jurisdiction_context",
            "mapping_status": "field_header_mappable_records_unchecked",
            "evidence_source": "manual DBF header inspection from GISCO Ports 2013 SHP package",
            "allowed_use": "node candidate schema planning only",
            "blocked_claims": BLOCKED,
            "next_action": "read bounded attribute records before node candidate selection",
        },
        {
            "mapping_id": "EUR-PORT-FIELD-004",
            "source_table": "PORT_PT_2013.dbf;DATA_SRC_AT.dbf",
            "source_field": "DATA_SRC_C;DATA_SRC_I;SRC_NAME;REFR_DATE",
            "candidate_contract_field": "source_custody_note",
            "mapping_status": "field_header_mappable_records_unchecked",
            "evidence_source": "manual DBF header inspection from GISCO Ports 2013 SHP package",
            "allowed_use": "node candidate schema planning only",
            "blocked_claims": BLOCKED,
            "next_action": "read bounded attribute records before node candidate selection",
        },
        {
            "mapping_id": "EUR-PORT-FIELD-005",
            "source_table": "PORT_PT_2013.shp",
            "source_field": "point geometry",
            "candidate_contract_field": "geometry_ref",
            "mapping_status": "geometry_present_not_read_or_accepted",
            "evidence_source": "data/international-eu-rhine-alpine-port-package-manifest-001.csv",
            "allowed_use": "record geometry presence only",
            "blocked_claims": BLOCKED,
            "next_action": "keep geometry blocked before node fixture replacement",
        },
        {
            "mapping_id": "EUR-PORT-FIELD-006",
            "source_table": "PORT_AT_2013.dbf",
            "source_field": "PORT_HIER_;TEN_CODE;PROC_TEN_C",
            "candidate_contract_field": "node_class_context",
            "mapping_status": "field_header_mappable_not_performance_validated",
            "evidence_source": "manual DBF header inspection from GISCO Ports 2013 SHP package",
            "allowed_use": "classification context planning only",
            "blocked_claims": BLOCKED,
            "next_action": "read bounded attribute records before any node class promotion",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
