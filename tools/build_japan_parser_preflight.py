#!/usr/bin/env python3
"""Build Japan parser preflight and output contract ledgers."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PREFLIGHT = ROOT / "data" / "international-japan-parser-preflight-001.csv"
CONTRACT = ROOT / "data" / "international-japan-parser-output-contract-001.csv"

PREFLIGHT_FIELDS = [
    "task_id",
    "source_id",
    "source_family",
    "target_adapter_table",
    "required_fields",
    "preflight_action",
    "allowed_output_label",
    "blocked_if_missing",
    "claim_boundary",
    "next_action",
]

CONTRACT_FIELDS = [
    "output_table",
    "required_columns",
    "required_label",
    "minimum_rows_allowed",
    "blocked_columns_or_values",
    "acceptance_rule",
    "claim_boundary",
]

BLOCKED = (
    "official_network;official_corridor_designation;ministry_approval;"
    "route_designation;source_row_validation;fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "disaster_readiness;terminal_performance;node_completeness;"
    "road_access_proof;throughput_proof;construction_ready;"
    "guaranteed_sla;travel_time_proof;delivery_commitment;numeric_roi;"
    "roi;eligibility;compliance;endorsement;validation;"
    "external_validation;public_readiness;external_readiness"
)


def write_csv(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {path}")


def main() -> None:
    preflight_rows = [
        {
            "task_id": "JPN-PARSE-001",
            "source_id": "JPN-SRC-001",
            "source_family": "road_bureau_context",
            "target_adapter_table": "japan_source_need_candidates",
            "required_fields": "road administration context; road bureau title; source owner; access note",
            "preflight_action": "extract bounded governance context only",
            "allowed_output_label": "source-candidate",
            "blocked_if_missing": "source owner or road bureau context missing",
            "claim_boundary": BLOCKED,
            "next_action": "write bounded need/context rows before any parser implementation",
        },
        {
            "task_id": "JPN-PARSE-002",
            "source_id": "JPN-SRC-002",
            "source_family": "road_statistics_context",
            "target_adapter_table": "japan_source_need_candidates",
            "required_fields": "road category; road length; statistic period; table title; access note",
            "preflight_action": "record road-statistics context only; do not infer need or SLA",
            "allowed_output_label": "source-candidate",
            "blocked_if_missing": "road-statistics table structure missing",
            "claim_boundary": BLOCKED,
            "next_action": "select exact table fields before need, graph, or parser mapping",
        },
        {
            "task_id": "JPN-PARSE-003",
            "source_id": "JPN-SRC-003",
            "source_family": "road_traffic_census_context",
            "target_adapter_table": "japan_source_need_candidates",
            "required_fields": "statistics code; census title; survey table; ministry in charge; access note",
            "preflight_action": "record traffic-census context only; do not forecast demand",
            "allowed_output_label": "source-candidate",
            "blocked_if_missing": "extractable census table missing",
            "claim_boundary": BLOCKED,
            "next_action": "select extractable census tables before traffic or service-target inference",
        },
        {
            "task_id": "JPN-PARSE-004",
            "source_id": "JPN-SRC-004",
            "source_family": "geospatial_road_context",
            "target_adapter_table": "japan_source_link_candidates",
            "required_fields": "transportation layer; road feature class; dataset version; download format; access note",
            "preflight_action": "hold road-feature extraction until usable metadata source exists",
            "allowed_output_label": "source-needed",
            "blocked_if_missing": "usable road-feature metadata source missing",
            "claim_boundary": BLOCKED,
            "next_action": "resolve usable metadata source before geometry, topology, map, or fixture promotion",
        },
        {
            "task_id": "JPN-PARSE-005",
            "source_id": "JPN-SRC-005",
            "source_family": "port_system_context",
            "target_adapter_table": "japan_source_node_candidates",
            "required_fields": "ports bureau context; port policy title; source owner; access note",
            "preflight_action": "record port-node source-selection context only",
            "allowed_output_label": "source-candidate",
            "blocked_if_missing": "port system context or access note missing",
            "claim_boundary": BLOCKED,
            "next_action": "select exact port-node rows before terminal, node, or access fixture use",
        },
        {
            "task_id": "JPN-PARSE-006",
            "source_id": "JPN-SRC-006",
            "source_family": "port_classification_context",
            "target_adapter_table": "japan_source_node_candidates",
            "required_fields": "port classification; port type; legal context; access note",
            "preflight_action": "record classification vocabulary only",
            "allowed_output_label": "source-candidate",
            "blocked_if_missing": "classification vocabulary missing",
            "claim_boundary": BLOCKED,
            "next_action": "map classification terms before port-node promotion",
        },
        {
            "task_id": "JPN-PARSE-007",
            "source_id": "JPN-SRC-007",
            "source_family": "hierarchy_fixture_context",
            "target_adapter_table": "japan_source_link_candidates",
            "required_fields": "candidate tier; service role; readiness basis; evidence label; claim boundary",
            "preflight_action": "carry forward heuristic-held hierarchy rows only",
            "allowed_output_label": "heuristic-held",
            "blocked_if_missing": "evidence label or claim boundary missing",
            "claim_boundary": BLOCKED,
            "next_action": "complete source-row validation role review and geometry policy before replacing hierarchy fixture",
        },
        {
            "task_id": "JPN-PARSE-008",
            "source_id": "JPN-SRC-SLA-001",
            "source_family": "service_targets",
            "target_adapter_table": "japan_service_target_candidates",
            "required_fields": "target id; target class; assumption label; local basis; numeracy review",
            "preflight_action": "do not parse as source-bound target",
            "allowed_output_label": "held",
            "blocked_if_missing": "local target source and numeracy basis missing",
            "claim_boundary": BLOCKED,
            "next_action": "keep target assumptions held before target-posture closeout",
        },
        {
            "task_id": "JPN-PARSE-009",
            "source_id": "carry-forward",
            "source_family": "evidence_labels",
            "target_adapter_table": "japan_adapter_evidence_labels",
            "required_fields": "artifact path; row id; evidence label; blocked claims",
            "preflight_action": "attach labels to all future dry-run rows",
            "allowed_output_label": "carry-forward",
            "blocked_if_missing": "evidence label or blocked claims missing",
            "claim_boundary": BLOCKED,
            "next_action": "block dry-run use before evidence labels attach",
        },
        {
            "task_id": "JPN-PARSE-010",
            "source_id": "internal-roles",
            "source_family": "review_roles",
            "target_adapter_table": "japan_adapter_review_backlog",
            "required_fields": "role lane; review question; result; hold",
            "preflight_action": "create Japan role review backlog after dry run",
            "allowed_output_label": "carry-forward",
            "blocked_if_missing": "role lanes missing",
            "claim_boundary": BLOCKED,
            "next_action": "run Japan-specific roles before stronger claims",
        },
    ]
    contract_rows = [
        {
            "output_table": "japan_source_link_candidates",
            "required_columns": "source_id;source_family;route_or_layer_id;route_or_layer_name;source_class;geometry_ref;source_owner;source_date;access_note;evidence_label;blocked_claims",
            "required_label": "source-needed or heuristic-held",
            "minimum_rows_allowed": "1",
            "blocked_columns_or_values": BLOCKED,
            "acceptance_rule": "rows may come only from JPN-SRC-004 as source-needed blockers or JPN-SRC-007 as heuristic-held carry-forward; geometry_ref must remain none or metadata-only",
            "claim_boundary": "no official Japanese corridor ministry approval route designation geometry acceptance topology proof disaster-readiness SLA ROI construction endorsement validation public-readiness or external-readiness claim",
        },
        {
            "output_table": "japan_source_need_candidates",
            "required_columns": "source_id;source_family;need_id;need_class;source_quote_or_summary;source_owner;source_date;access_note;evidence_label;blocked_claims",
            "required_label": "source-candidate",
            "minimum_rows_allowed": "1",
            "blocked_columns_or_values": BLOCKED,
            "acceptance_rule": "rows may come only from JPN-SRC-001, JPN-SRC-002, or JPN-SRC-003, must summarize bounded context only, and must not contain geometry",
            "claim_boundary": "no official corridor approval SLA ROI construction priority endorsement validation public-readiness or external-readiness claim",
        },
        {
            "output_table": "japan_source_node_candidates",
            "required_columns": "source_id;node_id;node_label;node_class;source_owner;source_date;source_url;access_note;evidence_label;blocked_claims",
            "required_label": "source-candidate",
            "minimum_rows_allowed": "1",
            "blocked_columns_or_values": BLOCKED,
            "acceptance_rule": "rows may come only from JPN-SRC-005 or JPN-SRC-006 and must remain no-geometry port-node candidates",
            "claim_boundary": "no geometry acceptance topology map overlay terminal performance road access proof node completeness endorsement validation public-readiness or external-readiness claim",
        },
        {
            "output_table": "japan_service_target_candidates",
            "required_columns": "target_gap_id;role;needed_source;assumption_label;evidence_label;blocked_claims",
            "required_label": "held",
            "minimum_rows_allowed": "1",
            "blocked_columns_or_values": BLOCKED,
            "acceptance_rule": "table must contain held assumption rows only, no geometry, until target source and numeracy review close",
            "claim_boundary": "no guaranteed SLA travel-time proof delivery commitment approval construction ROI validation public-readiness or external-readiness claim",
        },
        {
            "output_table": "japan_adapter_evidence_labels",
            "required_columns": "artifact_path;row_id;evidence_label;blocked_claims;source_id;review_note",
            "required_label": "carry-forward",
            "minimum_rows_allowed": "0",
            "blocked_columns_or_values": BLOCKED,
            "acceptance_rule": "every emitted dry-run row must have a matching evidence label row",
            "claim_boundary": "no validation approval public-readiness official corridor construction SLA ROI endorsement or external-readiness claim",
        },
        {
            "output_table": "japan_adapter_review_backlog",
            "required_columns": "role_lane;review_question;trigger_output;required_before;hold_claims;result",
            "required_label": "carry-forward",
            "minimum_rows_allowed": "0",
            "blocked_columns_or_values": BLOCKED,
            "acceptance_rule": "role backlog must exist before any output is used beyond internal parser inspection",
            "claim_boundary": "no external review agency signoff endorsement validation approval public-readiness or external-readiness claim",
        },
    ]
    write_csv(PREFLIGHT, PREFLIGHT_FIELDS, preflight_rows)
    write_csv(CONTRACT, CONTRACT_FIELDS, contract_rows)


if __name__ == "__main__":
    main()
