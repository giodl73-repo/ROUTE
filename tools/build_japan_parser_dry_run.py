#!/usr/bin/env python3
"""Build Japan parser dry-run fixture tables."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE_PACK = ROOT / "data" / "international-japan-adapter-source-pack-001.csv"
CONTRACT = ROOT / "data" / "international-japan-parser-output-contract-001.csv"

LINKS = ROOT / "data" / "japan_source_link_candidates.csv"
NEEDS = ROOT / "data" / "japan_source_need_candidates.csv"
NODES = ROOT / "data" / "japan_source_node_candidates.csv"
TARGETS = ROOT / "data" / "japan_service_target_candidates.csv"
LABELS = ROOT / "data" / "japan_adapter_evidence_labels.csv"
BACKLOG = ROOT / "data" / "japan_adapter_review_backlog.csv"

LINK_FIELDS = [
    "source_id",
    "source_family",
    "route_or_layer_id",
    "route_or_layer_name",
    "source_class",
    "geometry_ref",
    "source_owner",
    "source_date",
    "access_note",
    "evidence_label",
    "blocked_claims",
]
NEED_FIELDS = [
    "source_id",
    "source_family",
    "need_id",
    "need_class",
    "source_quote_or_summary",
    "source_owner",
    "source_date",
    "access_note",
    "evidence_label",
    "blocked_claims",
]
NODE_FIELDS = [
    "source_id",
    "node_id",
    "node_label",
    "node_class",
    "source_owner",
    "source_date",
    "source_url",
    "access_note",
    "evidence_label",
    "blocked_claims",
]
TARGET_FIELDS = [
    "target_gap_id",
    "role",
    "needed_source",
    "assumption_label",
    "evidence_label",
    "blocked_claims",
]
LABEL_FIELDS = [
    "artifact_path",
    "row_id",
    "evidence_label",
    "blocked_claims",
    "source_id",
    "review_note",
]
BACKLOG_FIELDS = [
    "role_lane",
    "review_question",
    "trigger_output",
    "required_before",
    "hold_claims",
    "result",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def write_csv(path: Path, fieldnames: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {path}")


def contract(output_table: str) -> dict[str, str]:
    for row in read_csv(CONTRACT):
        if row["output_table"] == output_table:
            return row
    raise KeyError(output_table)


def source(source_id: str) -> dict[str, str]:
    for row in read_csv(SOURCE_PACK):
        if row["source_id"] == source_id:
            return row
    raise KeyError(source_id)


def main() -> None:
    link_blocked = contract("japan_source_link_candidates")["blocked_columns_or_values"]
    need_blocked = contract("japan_source_need_candidates")["blocked_columns_or_values"]
    node_blocked = contract("japan_source_node_candidates")["blocked_columns_or_values"]
    target_blocked = contract("japan_service_target_candidates")["blocked_columns_or_values"]

    src1 = source("JPN-SRC-001")
    src2 = source("JPN-SRC-002")
    src3 = source("JPN-SRC-003")
    src4 = source("JPN-SRC-004")
    src5 = source("JPN-SRC-005")
    src6 = source("JPN-SRC-006")
    src7 = source("JPN-SRC-007")

    link_rows = [
        {
            "source_id": "JPN-SRC-004",
            "source_family": "geospatial_road_context",
            "route_or_layer_id": "JPN-GSI-SOURCE-NEEDED-001",
            "route_or_layer_name": "GSI road-feature metadata source needed",
            "source_class": "source_needed_road_feature_metadata",
            "geometry_ref": "not_accepted:source_needed:no_geometry",
            "source_owner": src4["owner_or_publisher"],
            "source_date": src4["date_accessed"],
            "access_note": "GSI bounded probe did not return usable metadata; no geometry or topology accepted",
            "evidence_label": "source-needed",
            "blocked_claims": link_blocked,
        },
        {
            "source_id": "JPN-SRC-007",
            "source_family": "hierarchy_fixture_context",
            "route_or_layer_id": "JPN-HIER2-CARRYFORWARD",
            "route_or_layer_name": "Japan hierarchy v2 heuristic-held carry-forward rows",
            "source_class": "heuristic_fixture_reference",
            "geometry_ref": "not_accepted:local_fixture_only",
            "source_owner": src7["owner_or_publisher"],
            "source_date": src7["date_accessed"],
            "access_note": "local hierarchy fixture reference only; not source-row validation",
            "evidence_label": "heuristic-held",
            "blocked_claims": link_blocked,
        },
    ]
    need_rows = [
        {
            "source_id": "JPN-SRC-001",
            "source_family": "road_bureau_context",
            "need_id": "JPN-NEED-CAND-001",
            "need_class": "road_governance_context",
            "source_quote_or_summary": "MLIT Road Bureau context can inform governance vocabulary only.",
            "source_owner": src1["owner_or_publisher"],
            "source_date": src1["date_accessed"],
            "access_note": "bounded context summary only; no official corridor, route, or service inference",
            "evidence_label": "source-candidate",
            "blocked_claims": need_blocked,
        },
        {
            "source_id": "JPN-SRC-002",
            "source_family": "road_statistics_context",
            "need_id": "JPN-NEED-CAND-002",
            "need_class": "road_statistics_context",
            "source_quote_or_summary": "Road statistics context can inform future table inventory but not need, SLA, or route priority.",
            "source_owner": src2["owner_or_publisher"],
            "source_date": src2["date_accessed"],
            "access_note": "bounded context summary only; no parsed table or need claim",
            "evidence_label": "source-candidate",
            "blocked_claims": need_blocked,
        },
        {
            "source_id": "JPN-SRC-003",
            "source_family": "road_traffic_census_context",
            "need_id": "JPN-NEED-CAND-003",
            "need_class": "traffic_census_context",
            "source_quote_or_summary": "Road Traffic Census context can inform future table selection but not demand forecast or service-target proof.",
            "source_owner": src3["owner_or_publisher"],
            "source_date": src3["date_accessed"],
            "access_note": "bounded context summary only; no traffic row, forecast, or SLA inference",
            "evidence_label": "source-candidate",
            "blocked_claims": need_blocked,
        },
    ]
    node_rows = [
        {
            "source_id": "JPN-SRC-005",
            "node_id": "JPN-PORT-CAND-001",
            "node_label": "Japan port-system node vocabulary candidate",
            "node_class": "port_node_context_candidate",
            "source_owner": src5["owner_or_publisher"],
            "source_date": src5["date_accessed"],
            "source_url": src5["source_path_or_status"],
            "access_note": "port-system context candidate only; node identity and geometry not validated",
            "evidence_label": "source-candidate",
            "blocked_claims": node_blocked,
        },
        {
            "source_id": "JPN-SRC-006",
            "node_id": "JPN-PORT-CLASS-CAND-001",
            "node_label": "Japan port-classification vocabulary candidate",
            "node_class": "port_classification_context_candidate",
            "source_owner": src6["owner_or_publisher"],
            "source_date": src6["date_accessed"],
            "source_url": src6["source_path_or_status"],
            "access_note": "classification context candidate only; port-node completeness and service obligation not validated",
            "evidence_label": "source-candidate",
            "blocked_claims": node_blocked,
        },
    ]
    target_rows = [
        {
            "target_gap_id": "JPN-TARGET-GAP-001",
            "role": "Japan Pacific Belt resilience and port access",
            "needed_source": "local service target source and numeracy basis",
            "assumption_label": "planning_assumption_only",
            "evidence_label": "held",
            "blocked_claims": target_blocked,
        }
    ]

    label_rows: list[dict[str, str]] = []
    for path, row_id, rows in [
        ("data/japan_source_link_candidates.csv", "route_or_layer_id", link_rows),
        ("data/japan_source_need_candidates.csv", "need_id", need_rows),
        ("data/japan_source_node_candidates.csv", "node_id", node_rows),
        ("data/japan_service_target_candidates.csv", "target_gap_id", target_rows),
    ]:
        for row in rows:
            label_rows.append(
                {
                    "artifact_path": path,
                    "row_id": row[row_id],
                    "evidence_label": row["evidence_label"],
                    "blocked_claims": row["blocked_claims"],
                    "source_id": row.get("source_id", "JPN-SRC-SLA-001"),
                    "review_note": "Japan dry-run row; internal parser fixture only",
                }
            )

    backlog_rows = [
        {
            "role_lane": "Scope Keeper",
            "review_question": "Does Japan remain a source-custody dry run rather than validation?",
            "trigger_output": "all output tables",
            "required_before": "any use beyond internal inspection",
            "hold_claims": "official_corridor_designation;ministry_approval;external_validation",
            "result": "pending",
        },
        {
            "role_lane": "Citation Auditor",
            "review_question": "Do Japan rows preserve owner, date, access note, label, and blocked claims?",
            "trigger_output": "all output tables",
            "required_before": "parser implementation closeout",
            "hold_claims": "validation;endorsement;external_validation",
            "result": "pending",
        },
        {
            "role_lane": "Schematic Cartographer",
            "review_question": "Could source-needed or heuristic rows be mistaken for accepted geometry or map proof?",
            "trigger_output": "japan_source_link_candidates",
            "required_before": "any map overlay or fixture replacement",
            "hold_claims": "geometry_acceptance;topology_proof;official_corridor_designation",
            "result": "pending",
        },
        {
            "role_lane": "V&V",
            "review_question": "Does every Japan row have a matching evidence-label row?",
            "trigger_output": "japan_adapter_evidence_labels",
            "required_before": "parser dry-run closeout",
            "hold_claims": "validation;public_readiness;external_readiness",
            "result": "pending",
        },
    ]

    write_csv(LINKS, LINK_FIELDS, link_rows)
    write_csv(NEEDS, NEED_FIELDS, need_rows)
    write_csv(NODES, NODE_FIELDS, node_rows)
    write_csv(TARGETS, TARGET_FIELDS, target_rows)
    write_csv(LABELS, LABEL_FIELDS, label_rows)
    write_csv(BACKLOG, BACKLOG_FIELDS, backlog_rows)


if __name__ == "__main__":
    main()
