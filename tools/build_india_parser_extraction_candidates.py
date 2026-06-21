#!/usr/bin/env python3
"""Build India parser extraction candidates from bounded source content."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-india-source-content-sample-001.csv"
CONTRACT = ROOT / "data" / "international-india-parser-output-contract-001.csv"
OUTPUT = ROOT / "data" / "international-india-parser-extraction-candidates-001.csv"

FIELDS = [
    "candidate_id",
    "target_table",
    "source_id",
    "source_family",
    "candidate_key",
    "candidate_label",
    "candidate_class",
    "geometry_ref",
    "source_owner",
    "source_date",
    "access_note",
    "evidence_label",
    "candidate_status",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def contract_for(output_table: str) -> dict[str, str]:
    for row in read_csv(CONTRACT):
        if row["output_table"] == output_table:
            return row
    raise RuntimeError(f"missing {output_table} contract")


def sample_for(rows: list[dict[str, str]], source_id: str) -> dict[str, str]:
    for row in rows:
        if row["source_id"] == source_id:
            return row
    raise RuntimeError(f"missing source content sample for {source_id}")


def main() -> None:
    samples = read_csv(SAMPLE)
    link_contract = contract_for("india_source_link_candidates")
    need_contract = contract_for("india_source_need_candidates")
    node_contract = contract_for("india_source_node_candidates")
    nhai = sample_for(samples, "IND-SRC-002")
    ports = sample_for(samples, "IND-SRC-003")
    stats = sample_for(samples, "IND-SRC-004")
    rows = [
        {
            "candidate_id": "IND-EXTRACT-LINK-001",
            "target_table": "india_source_link_candidates",
            "source_id": "IND-SRC-002",
            "source_family": nhai["source_family"],
            "candidate_key": "NHAI-AUTHORITY-CONTEXT-001",
            "candidate_label": "NHAI national-highway authority context",
            "candidate_class": "authority_context_not_road_link_row",
            "geometry_ref": "not_requested:authority_context:no_geometry",
            "source_owner": nhai["source_owner"],
            "source_date": nhai["source_date"],
            "access_note": "bounded extraction candidate; no road feature row accepted",
            "evidence_label": "source-candidate",
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": link_contract["blocked_columns_or_values"],
            "next_action": "identify inspectable road-link or route-attribute rows before fixture replacement",
        },
        {
            "candidate_id": "IND-EXTRACT-NODE-001",
            "target_table": "india_source_node_candidates",
            "source_id": "IND-SRC-003",
            "source_family": ports["source_family"],
            "candidate_key": "PORT-JNPA-001",
            "candidate_label": "Jawaharlal Nehru Port Authority",
            "candidate_class": "major_port_node_candidate_not_validated",
            "geometry_ref": "not_requested:major_port_list:no_geometry",
            "source_owner": ports["source_owner"],
            "source_date": ports["source_date"],
            "access_note": "bounded major-port name extraction candidate; no terminal geometry or performance accepted",
            "evidence_label": node_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": node_contract["blocked_columns_or_values"],
            "next_action": "run node source-row validation and role review before any node fixture use",
        },
        {
            "candidate_id": "IND-EXTRACT-NODE-002",
            "target_table": "india_source_node_candidates",
            "source_id": "IND-SRC-003",
            "source_family": ports["source_family"],
            "candidate_key": "PORT-MUMBAI-001",
            "candidate_label": "Mumbai Port Authority",
            "candidate_class": "major_port_node_candidate_not_validated",
            "geometry_ref": "not_requested:major_port_list:no_geometry",
            "source_owner": ports["source_owner"],
            "source_date": ports["source_date"],
            "access_note": "bounded major-port name extraction candidate; no terminal geometry or performance accepted",
            "evidence_label": node_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": node_contract["blocked_columns_or_values"],
            "next_action": "run node source-row validation and role review before any node fixture use",
        },
        {
            "candidate_id": "IND-EXTRACT-NODE-003",
            "target_table": "india_source_node_candidates",
            "source_id": "IND-SRC-003",
            "source_family": ports["source_family"],
            "candidate_key": "PORT-VISAKHAPATNAM-001",
            "candidate_label": "Visakhapatnam Port Authority",
            "candidate_class": "major_port_node_candidate_not_validated",
            "geometry_ref": "not_requested:major_port_list:no_geometry",
            "source_owner": ports["source_owner"],
            "source_date": ports["source_date"],
            "access_note": "bounded major-port name extraction candidate; no terminal geometry or performance accepted",
            "evidence_label": node_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": node_contract["blocked_columns_or_values"],
            "next_action": "run node source-row validation and role review before any node fixture use",
        },
        {
            "candidate_id": "IND-EXTRACT-NEED-001",
            "target_table": "india_source_need_candidates",
            "source_id": "IND-SRC-004",
            "source_family": stats["source_family"],
            "candidate_key": "PORT-STATS-2024-25",
            "candidate_label": "Basic Port Statistics of India 2024-25 publication lead",
            "candidate_class": "publication_lead_not_throughput_row",
            "geometry_ref": "not_requested:publication_lead:no_geometry",
            "source_owner": stats["source_owner"],
            "source_date": stats["source_date"],
            "access_note": "bounded publication lead; no throughput, demand, or service target inferred",
            "evidence_label": need_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": need_contract["blocked_columns_or_values"],
            "next_action": "inventory publication tables before any need or service-target inference",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
