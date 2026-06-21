#!/usr/bin/env python3
"""Build China parser extraction candidates from bounded source content."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-china-source-content-sample-001.csv"
CONTRACT = ROOT / "data" / "international-china-parser-output-contract-001.csv"
OUTPUT = ROOT / "data" / "international-china-parser-extraction-candidates-001.csv"

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


def blocked_claims(row: dict[str, str]) -> str:
    claims = row["blocked_columns_or_values"].split(";")
    if "internal_adapter_proof" not in claims:
        claims.append("internal_adapter_proof")
    return ";".join(claims)


def main() -> None:
    samples = read_csv(SAMPLE)
    link_contract = contract_for("china_source_link_candidates")
    need_contract = contract_for("china_source_need_candidates")
    node_contract = contract_for("china_source_node_candidates")
    ministry = sample_for(samples, "CHN-SRC-001")
    plan = sample_for(samples, "CHN-SRC-002")
    stats = sample_for(samples, "CHN-SRC-003")
    standards = sample_for(samples, "CHN-SRC-004")
    ports = sample_for(samples, "CHN-SRC-005")
    rows = [
        {
            "candidate_id": "CHN-EXTRACT-LINK-001",
            "target_table": "china_source_link_candidates",
            "source_id": "CHN-SRC-004",
            "source_family": standards["source_family"],
            "candidate_key": "MOT-HIGHWAY-STANDARDS-CONTEXT-001",
            "candidate_label": "MOT highway standards context",
            "candidate_class": "standards_context_not_road_link_row",
            "geometry_ref": "not_requested:standards_context:no_geometry",
            "source_owner": standards["source_owner"],
            "source_date": standards["source_date"],
            "access_note": "bounded standards-context extraction candidate; no design geometry or road feature row accepted",
            "evidence_label": "context-only",
            "candidate_status": "source_content_extraction_context_only_not_promoted",
            "blocked_claims": blocked_claims(link_contract),
            "next_action": "select exact source road-link or route-attribute rows before link extraction or fixture replacement",
        },
        {
            "candidate_id": "CHN-EXTRACT-NEED-001",
            "target_table": "china_source_need_candidates",
            "source_id": "CHN-SRC-001",
            "source_family": ministry["source_family"],
            "candidate_key": "MOT-MINISTRY-CONTEXT-001",
            "candidate_label": "MOT ministry publication surface context",
            "candidate_class": "ministry_context_not_need_row",
            "geometry_ref": "not_requested:ministry_context:no_geometry",
            "source_owner": ministry["source_owner"],
            "source_date": ministry["source_date"],
            "access_note": "bounded ministry-context candidate; no road rows, route designations, or policy alignment accepted",
            "evidence_label": need_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": blocked_claims(need_contract),
            "next_action": "select exact MOT road-network, highway, or data table pages before source-row extraction",
        },
        {
            "candidate_id": "CHN-EXTRACT-NEED-002",
            "target_table": "china_source_need_candidates",
            "source_id": "CHN-SRC-002",
            "source_family": plan["source_family"],
            "candidate_key": "STATE-COUNCIL-PLAN-CONTEXT-001",
            "candidate_label": "State Council transport-plan context",
            "candidate_class": "planning_context_not_policy_alignment",
            "geometry_ref": "not_requested:planning_context:no_geometry",
            "source_owner": plan["source_owner"],
            "source_date": plan["source_date"],
            "access_note": "bounded planning-context candidate; no priority-corridor, construction, or policy-alignment claim accepted",
            "evidence_label": need_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": blocked_claims(need_contract),
            "next_action": "separate plan context from policy-alignment claims before parser extraction",
        },
        {
            "candidate_id": "CHN-EXTRACT-NEED-003",
            "target_table": "china_source_need_candidates",
            "source_id": "CHN-SRC-003",
            "source_family": stats["source_family"],
            "candidate_key": "NBS-TRANSPORT-STATS-CONTEXT-001",
            "candidate_label": "NBS transport-statistics table-selection lead",
            "candidate_class": "statistics_context_not_need_row",
            "geometry_ref": "not_requested:statistics_context:no_geometry",
            "source_owner": stats["source_owner"],
            "source_date": stats["source_date"],
            "access_note": "bounded statistics-context candidate; no parsed statistic, forecast demand, SLA, or ROI inference",
            "evidence_label": need_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": blocked_claims(need_contract),
            "next_action": "select exact transport-statistics tables before need, service-target, or ROI inference",
        },
        {
            "candidate_id": "CHN-EXTRACT-NODE-001",
            "target_table": "china_source_node_candidates",
            "source_id": "CHN-SRC-005",
            "source_family": ports["source_family"],
            "candidate_key": "STATE-COUNCIL-PORT-WATERWAY-CONTEXT-001",
            "candidate_label": "State Council port and waterway table-inventory lead",
            "candidate_class": "port_waterway_context_not_node_row",
            "geometry_ref": "not_requested:port_waterway_context:no_geometry",
            "source_owner": ports["source_owner"],
            "source_date": ports["source_date"],
            "access_note": "bounded port/waterway context candidate; no terminal performance, throughput, node completeness, or road access proof accepted",
            "evidence_label": node_contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": blocked_claims(node_contract),
            "next_action": "select port-node or waterway table rows before terminal, throughput, node, or access fixture use",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
