#!/usr/bin/env python3
"""Build EU Rhine-Alpine parser dry-run fixture tables."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE_PACK = ROOT / "data" / "international-eu-rhine-alpine-adapter-source-pack-001.csv"
CONTRACT = ROOT / "data" / "international-eu-rhine-alpine-parser-output-contract-001.csv"

LINKS = ROOT / "data" / "eu_rhine_alpine_source_link_candidates.csv"
NEEDS = ROOT / "data" / "eu_rhine_alpine_source_need_candidates.csv"
NODES = ROOT / "data" / "eu_rhine_alpine_source_node_candidates.csv"
TARGETS = ROOT / "data" / "eu_rhine_alpine_service_target_candidates.csv"
LABELS = ROOT / "data" / "eu_rhine_alpine_adapter_evidence_labels.csv"
BACKLOG = ROOT / "data" / "eu_rhine_alpine_adapter_review_backlog.csv"


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
    "node_gap_id",
    "node_class",
    "needed_source",
    "assumption_label",
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
    link_blocked = contract("eu_rhine_alpine_source_link_candidates")["blocked_columns_or_values"]
    need_blocked = contract("eu_rhine_alpine_source_need_candidates")["blocked_columns_or_values"]
    node_blocked = contract("eu_rhine_alpine_source_node_candidates")["blocked_columns_or_values"]
    target_blocked = contract("eu_rhine_alpine_service_target_candidates")["blocked_columns_or_values"]

    src2 = source("EUR-SRC-002")
    src3 = source("EUR-SRC-003")
    link_rows = [
        {
            "source_id": "EUR-SRC-002",
            "source_family": "network_viewer",
            "route_or_layer_id": "EUR-LAYER-TENTEC-001",
            "route_or_layer_name": "TENtec public network/corridor layer metadata candidate",
            "source_class": "network_viewer_metadata",
            "geometry_ref": "not_accepted:metadata_only",
            "source_owner": src2["owner_or_publisher"],
            "source_date": src2["date_accessed"],
            "access_note": "metadata candidate only; no geometry or topology accepted",
            "evidence_label": "source-candidate",
            "blocked_claims": link_blocked,
        },
        {
            "source_id": "EUR-SRC-003",
            "source_family": "transport_geodata",
            "route_or_layer_id": "EUR-LAYER-GISCO-001",
            "route_or_layer_name": "GISCO transport network dataset candidate",
            "source_class": "transport_geodata_metadata",
            "geometry_ref": "not_accepted:dataset_version_needed",
            "source_owner": src3["owner_or_publisher"],
            "source_date": src3["date_accessed"],
            "access_note": "dataset candidate only; suitability warning required before geometry use",
            "evidence_label": "source-candidate",
            "blocked_claims": link_blocked,
        },
    ]

    src1 = source("EUR-SRC-001")
    src4 = source("EUR-SRC-004")
    need_rows = [
        {
            "source_id": "EUR-SRC-001",
            "source_family": "corridor_context",
            "need_id": "EUR-NEED-CAND-001",
            "need_class": "ten_t_corridor_context",
            "source_quote_or_summary": "TEN-T corridor map-library context can inform scope vocabulary only.",
            "source_owner": src1["owner_or_publisher"],
            "source_date": src1["date_accessed"],
            "access_note": "bounded context summary only; no official or service inference",
            "evidence_label": "source-candidate",
            "blocked_claims": need_blocked,
        },
        {
            "source_id": "EUR-SRC-004",
            "source_family": "rhine_alpine_context",
            "need_id": "EUR-NEED-CAND-002",
            "need_class": "rhine_alpine_corridor_context",
            "source_quote_or_summary": "Rhine-Alpine context can inform region vocabulary but not road/service proof.",
            "source_owner": src4["owner_or_publisher"],
            "source_date": src4["date_accessed"],
            "access_note": "bounded context summary only; rail/corridor context is not road-network proof",
            "evidence_label": "source-candidate",
            "blocked_claims": need_blocked,
        },
    ]

    node_rows = [
        {
            "source_id": "EUR-SRC-005",
            "node_gap_id": "EUR-NODE-GAP-001",
            "node_class": "port_or_terminal_gateway",
            "needed_source": "road/port node source custody for Rotterdam Antwerp Rhine industrial nodes Genoa and Alpine gateways",
            "assumption_label": "source_needed",
            "evidence_label": "source-needed",
            "blocked_claims": node_blocked,
        }
    ]
    target_rows = [
        {
            "target_gap_id": "EUR-TARGET-GAP-001",
            "role": "Rhine-Alpine trunk and gateway access",
            "needed_source": "adopted service target source and numeracy basis",
            "assumption_label": "planning_assumption_only",
            "evidence_label": "held",
            "blocked_claims": target_blocked,
        }
    ]

    label_rows: list[dict[str, str]] = []
    for path, row_id, rows in [
        ("data/eu_rhine_alpine_source_link_candidates.csv", "route_or_layer_id", link_rows),
        ("data/eu_rhine_alpine_source_need_candidates.csv", "need_id", need_rows),
        ("data/eu_rhine_alpine_source_node_candidates.csv", "node_gap_id", node_rows),
        ("data/eu_rhine_alpine_service_target_candidates.csv", "target_gap_id", target_rows),
    ]:
        for row in rows:
            label_rows.append(
                {
                    "artifact_path": path,
                    "row_id": row[row_id],
                    "evidence_label": row["evidence_label"],
                    "blocked_claims": row["blocked_claims"],
                    "source_id": row.get("source_id", "EUR-SRC-SLA-001"),
                    "review_note": "EU dry-run row; internal parser fixture only",
                }
            )

    backlog_rows = [
        {
            "role_lane": "Scope Keeper",
            "review_question": "Does EU remain a source-custody dry run rather than validation?",
            "trigger_output": "all output tables",
            "required_before": "any use beyond internal inspection",
            "hold_claims": "official_corridor_designation;member_state_approval;external_validation",
            "result": "pending",
        },
        {
            "role_lane": "Citation Auditor",
            "review_question": "Do EU rows preserve owner, date, access note, label, and blocked claims?",
            "trigger_output": "all output tables",
            "required_before": "parser implementation closeout",
            "hold_claims": "validation;endorsement;external_validation",
            "result": "pending",
        },
        {
            "role_lane": "Schematic Cartographer",
            "review_question": "Could metadata rows be mistaken for accepted geometry or map proof?",
            "trigger_output": "eu_rhine_alpine_source_link_candidates",
            "required_before": "any map overlay or fixture replacement",
            "hold_claims": "geometry_acceptance;topology_proof;official_corridor_designation",
            "result": "pending",
        },
        {
            "role_lane": "V&V",
            "review_question": "Does every EU row have a matching evidence-label row?",
            "trigger_output": "eu_rhine_alpine_adapter_evidence_labels",
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
