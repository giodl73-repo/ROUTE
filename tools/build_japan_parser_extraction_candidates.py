#!/usr/bin/env python3
"""Build Japan parser extraction candidates from bounded source content."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-japan-source-content-sample-001.csv"
CONTRACT = ROOT / "data" / "international-japan-parser-output-contract-001.csv"
OUTPUT = ROOT / "data" / "international-japan-parser-extraction-candidates-001.csv"

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
    link_contract = contract_for("japan_source_link_candidates")
    need_contract = contract_for("japan_source_need_candidates")
    node_contract = contract_for("japan_source_node_candidates")
    road = sample_for(samples, "JPN-SRC-001")
    stats = sample_for(samples, "JPN-SRC-002")
    census = sample_for(samples, "JPN-SRC-003")
    gsi = sample_for(samples, "JPN-SRC-004")
    ports = sample_for(samples, "JPN-SRC-005")
    classification = sample_for(samples, "JPN-SRC-006")
    rows = [
        {
            "candidate_id": "JPN-EXTRACT-LINK-BLOCK-001",
            "target_table": "japan_source_link_candidates",
            "source_id": "JPN-SRC-004",
            "source_family": gsi["source_family"],
            "candidate_key": "GSI-ROAD-FEATURE-SOURCE-NEEDED",
            "candidate_label": "GSI road-feature metadata source blocker",
            "candidate_class": "source_needed_not_road_link_row",
            "geometry_ref": "not_requested:source_needed:no_geometry",
            "source_owner": gsi["source_owner"],
            "source_date": gsi["source_date"],
            "access_note": "bounded extraction blocker; usable road-feature metadata source not resolved",
            "evidence_label": "source-needed",
            "candidate_status": "source_content_extraction_blocked_not_promoted",
            "blocked_claims": link_contract["blocked_columns_or_values"],
            "next_action": "resolve usable road-feature metadata source before link extraction or fixture replacement",
        },
        {
            "candidate_id": "JPN-EXTRACT-NEED-001",
            "target_table": "japan_source_need_candidates",
            "source_id": "JPN-SRC-001",
            "source_family": road["source_family"],
            "candidate_key": "MLIT-ROAD-BUREAU-CONTEXT-001",
            "candidate_label": "MLIT Road Bureau governance context",
            "candidate_class": "governance_context_not_need_row",
            "geometry_ref": "not_requested:governance_context:no_geometry",
            "source_owner": road["source_owner"],
            "source_date": road["source_date"],
            "access_note": "bounded governance context candidate; no official corridor, route, or service inference",
            "evidence_label": need_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": need_contract["blocked_columns_or_values"],
            "next_action": "identify exact road-network tables or documents before source-row extraction",
        },
        {
            "candidate_id": "JPN-EXTRACT-NEED-002",
            "target_table": "japan_source_need_candidates",
            "source_id": "JPN-SRC-002",
            "source_family": stats["source_family"],
            "candidate_key": "MLIT-ROAD-STATISTICS-CONTEXT-001",
            "candidate_label": "MLIT road-statistics table inventory lead",
            "candidate_class": "statistics_context_not_need_row",
            "geometry_ref": "not_requested:statistics_context:no_geometry",
            "source_owner": stats["source_owner"],
            "source_date": stats["source_date"],
            "access_note": "bounded statistics context candidate; no parsed table or need inference",
            "evidence_label": need_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": need_contract["blocked_columns_or_values"],
            "next_action": "inventory road-statistics table structure before need, graph, or parser mapping",
        },
        {
            "candidate_id": "JPN-EXTRACT-NEED-003",
            "target_table": "japan_source_need_candidates",
            "source_id": "JPN-SRC-003",
            "source_family": census["source_family"],
            "candidate_key": "ESTAT-ROAD-TRAFFIC-CENSUS-CONTEXT-001",
            "candidate_label": "e-Stat Road Traffic Census table-selection lead",
            "candidate_class": "traffic_context_not_demand_row",
            "geometry_ref": "not_requested:traffic_context:no_geometry",
            "source_owner": census["source_owner"],
            "source_date": census["source_date"],
            "access_note": "bounded traffic context candidate; no census row, forecast, or SLA inference",
            "evidence_label": need_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": need_contract["blocked_columns_or_values"],
            "next_action": "select extractable census tables before traffic, need, or service-target inference",
        },
        {
            "candidate_id": "JPN-EXTRACT-NODE-001",
            "target_table": "japan_source_node_candidates",
            "source_id": "JPN-SRC-005",
            "source_family": ports["source_family"],
            "candidate_key": "MLIT-PORT-SYSTEM-CONTEXT-001",
            "candidate_label": "MLIT port-system node vocabulary lead",
            "candidate_class": "port_context_not_node_row",
            "geometry_ref": "not_requested:port_context:no_geometry",
            "source_owner": ports["source_owner"],
            "source_date": ports["source_date"],
            "access_note": "bounded port context candidate; no node completeness, terminal performance, or road access proof",
            "evidence_label": node_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": node_contract["blocked_columns_or_values"],
            "next_action": "select exact port-node rows before terminal, node, or access fixture use",
        },
        {
            "candidate_id": "JPN-EXTRACT-NODE-002",
            "target_table": "japan_source_node_candidates",
            "source_id": "JPN-SRC-006",
            "source_family": classification["source_family"],
            "candidate_key": "MLIT-PORT-CLASSIFICATION-CONTEXT-001",
            "candidate_label": "MLIT port-classification vocabulary lead",
            "candidate_class": "classification_context_not_node_row",
            "geometry_ref": "not_requested:classification_context:no_geometry",
            "source_owner": classification["source_owner"],
            "source_date": classification["source_date"],
            "access_note": "bounded classification context candidate; no port-node promotion or service obligation",
            "evidence_label": node_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": node_contract["blocked_columns_or_values"],
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
