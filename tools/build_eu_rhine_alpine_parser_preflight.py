#!/usr/bin/env python3
"""Build EU Rhine-Alpine parser preflight and output contract ledgers."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PREFLIGHT = ROOT / "data" / "international-eu-rhine-alpine-parser-preflight-001.csv"
CONTRACT = ROOT / "data" / "international-eu-rhine-alpine-parser-output-contract-001.csv"

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
    "official_network;official_corridor_designation;member_state_approval;"
    "route_designation;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;endorsement;"
    "validation;external_validation;public_readiness;external_readiness"
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
            "task_id": "EUR-PARSE-001",
            "source_id": "EUR-SRC-001",
            "source_family": "corridor_context",
            "target_adapter_table": "eu_rhine_alpine_source_need_candidates",
            "required_fields": "corridor name; map publication date; corridor scope; access note",
            "preflight_action": "extract bounded TEN-T corridor vocabulary only",
            "allowed_output_label": "source-candidate",
            "blocked_if_missing": "corridor scope or publication context missing",
            "claim_boundary": BLOCKED,
            "next_action": "write bounded need/context rows before any parser implementation",
        },
        {
            "task_id": "EUR-PARSE-002",
            "source_id": "EUR-SRC-002",
            "source_family": "network_viewer",
            "target_adapter_table": "eu_rhine_alpine_source_link_candidates",
            "required_fields": "TEN-T layer metadata; corridor layer; access note",
            "preflight_action": "inspect layer metadata only; do not accept geometry",
            "allowed_output_label": "source-candidate",
            "blocked_if_missing": "inspectable layer metadata missing",
            "claim_boundary": BLOCKED,
            "next_action": "select fields before parser contract promotion or fixture replacement",
        },
        {
            "task_id": "EUR-PARSE-003",
            "source_id": "EUR-SRC-003",
            "source_family": "transport_geodata",
            "target_adapter_table": "eu_rhine_alpine_source_link_candidates",
            "required_fields": "dataset version; transport network class; scale; access note",
            "preflight_action": "record dataset suitability and candidate fields only",
            "allowed_output_label": "source-candidate",
            "blocked_if_missing": "dataset version or suitability warning missing",
            "claim_boundary": BLOCKED,
            "next_action": "record suitability warning before geometry or node use",
        },
        {
            "task_id": "EUR-PARSE-004",
            "source_id": "EUR-SRC-004",
            "source_family": "rhine_alpine_context",
            "target_adapter_table": "eu_rhine_alpine_source_need_candidates",
            "required_fields": "countries crossed; corridor description; access note",
            "preflight_action": "extract bounded Rhine-Alpine context only",
            "allowed_output_label": "source-candidate",
            "blocked_if_missing": "bounded context missing",
            "claim_boundary": BLOCKED,
            "next_action": "keep road and service inference held before local review",
        },
        {
            "task_id": "EUR-PARSE-005",
            "source_id": "EUR-SRC-003",
            "source_family": "port_geodata",
            "target_adapter_table": "eu_rhine_alpine_source_node_candidates",
            "required_fields": "port id; port name; country code; NUTS code; TEN code; hierarchy; access note",
            "preflight_action": "load validated GISCO Ports 2013 attribute candidates only; do not accept geometry",
            "allowed_output_label": "source-candidate",
            "blocked_if_missing": "validated no-geometry port attribute rows missing",
            "claim_boundary": BLOCKED,
            "next_action": "write node fixture closeout before any internal adapter proof",
        },
        {
            "task_id": "EUR-PARSE-006",
            "source_id": "EUR-SRC-SLA-001",
            "source_family": "service_targets",
            "target_adapter_table": "eu_rhine_alpine_service_target_candidates",
            "required_fields": "target id; role; target hours; basis; assumption label",
            "preflight_action": "do not parse as source-bound target",
            "allowed_output_label": "held",
            "blocked_if_missing": "adopted target source and numeracy basis missing",
            "claim_boundary": BLOCKED,
            "next_action": "keep target assumptions held before target-posture closeout",
        },
        {
            "task_id": "EUR-PARSE-007",
            "source_id": "carry-forward",
            "source_family": "evidence_labels",
            "target_adapter_table": "eu_rhine_alpine_adapter_evidence_labels",
            "required_fields": "artifact path; row id; evidence label; blocked claims",
            "preflight_action": "attach labels to all dry-run rows",
            "allowed_output_label": "carry-forward",
            "blocked_if_missing": "evidence label or blocked claims missing",
            "claim_boundary": BLOCKED,
            "next_action": "block dry-run use before evidence labels attach",
        },
        {
            "task_id": "EUR-PARSE-008",
            "source_id": "internal-roles",
            "source_family": "review_roles",
            "target_adapter_table": "eu_rhine_alpine_adapter_review_backlog",
            "required_fields": "role lane; review question; result; hold",
            "preflight_action": "create role review backlog after dry run",
            "allowed_output_label": "carry-forward",
            "blocked_if_missing": "role lanes missing",
            "claim_boundary": BLOCKED,
            "next_action": "rerun EU-specific roles before stronger claims",
        },
    ]
    contract_rows = [
        {
            "output_table": "eu_rhine_alpine_source_link_candidates",
            "required_columns": "source_id;source_family;route_or_layer_id;route_or_layer_name;source_class;geometry_ref;source_owner;source_date;access_note;evidence_label;blocked_claims",
            "required_label": "source-candidate",
            "minimum_rows_allowed": "2",
            "blocked_columns_or_values": BLOCKED,
            "acceptance_rule": "rows may come only from EUR-SRC-002 or EUR-SRC-003 and must remain metadata/no-geometry candidates",
            "claim_boundary": "no official EU corridor designation member-state approval geometry acceptance topology proof SLA ROI construction endorsement validation public-readiness or external-readiness claim",
        },
        {
            "output_table": "eu_rhine_alpine_source_need_candidates",
            "required_columns": "source_id;source_family;need_id;need_class;source_quote_or_summary;source_owner;source_date;access_note;evidence_label;blocked_claims",
            "required_label": "source-candidate",
            "minimum_rows_allowed": "2",
            "blocked_columns_or_values": BLOCKED,
            "acceptance_rule": "rows may come only from EUR-SRC-001 or EUR-SRC-004 and must summarize bounded vocabulary only",
            "claim_boundary": "no official corridor approval policy alignment SLA ROI construction priority endorsement validation public-readiness or external-readiness claim",
        },
        {
            "output_table": "eu_rhine_alpine_source_node_candidates",
            "required_columns": "source_id;node_id;node_label;node_class;source_owner;source_date;source_url;access_note;evidence_label;blocked_claims",
            "required_label": "source-candidate",
            "minimum_rows_allowed": "5",
            "blocked_columns_or_values": BLOCKED,
            "acceptance_rule": "rows may come only from validated GISCO Ports 2013 attribute samples and must remain no-geometry internal node candidates",
            "claim_boundary": "no geometry acceptance topology map overlay terminal performance road access proof node completeness endorsement validation public-readiness or external-readiness claim",
        },
        {
            "output_table": "eu_rhine_alpine_service_target_candidates",
            "required_columns": "target_gap_id;role;needed_source;assumption_label;evidence_label;blocked_claims",
            "required_label": "held",
            "minimum_rows_allowed": "1",
            "blocked_columns_or_values": BLOCKED,
            "acceptance_rule": "table must contain held assumption rows only until target source and numeracy review close",
            "claim_boundary": "no guaranteed SLA travel-time proof delivery commitment approval construction ROI validation public-readiness or external-readiness claim",
        },
        {
            "output_table": "eu_rhine_alpine_adapter_evidence_labels",
            "required_columns": "artifact_path;row_id;evidence_label;blocked_claims;source_id;review_note",
            "required_label": "carry-forward",
            "minimum_rows_allowed": "0",
            "blocked_columns_or_values": BLOCKED,
            "acceptance_rule": "every emitted dry-run row must have a matching evidence label row",
            "claim_boundary": "no validation approval public-readiness official corridor construction SLA ROI endorsement or external-readiness claim",
        },
        {
            "output_table": "eu_rhine_alpine_adapter_review_backlog",
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
