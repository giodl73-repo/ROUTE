#!/usr/bin/env python3
"""Build Canada parser dry-run fixture tables from declared contracts.

The dry run is intentionally source-bounded: it reads the Canada source pack,
parser preflight, and output contract, then emits candidate/gap/held rows for
internal parser inspection. It does not download sources, parse source files, or
promote a source-bound Canada adapter.
"""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE_PACK = ROOT / "data" / "international-canada-adapter-source-pack-001.csv"
PREFLIGHT = ROOT / "data" / "international-canada-parser-preflight-001.csv"
CONTRACT = ROOT / "data" / "international-canada-parser-output-contract-001.csv"
EXTRACTION_CANDIDATES = ROOT / "data" / "international-canada-parser-extraction-candidates-001.csv"

LINKS = ROOT / "data" / "canada_source_link_candidates.csv"
NEEDS = ROOT / "data" / "canada_source_need_candidates.csv"
NODES = ROOT / "data" / "canada_source_node_candidates.csv"
TARGETS = ROOT / "data" / "canada_service_target_candidates.csv"
LABELS = ROOT / "data" / "canada_adapter_evidence_labels.csv"
BACKLOG = ROOT / "data" / "canada_adapter_review_backlog.csv"


LINK_FIELDS = [
    "source_id",
    "source_family",
    "route_id",
    "route_name",
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
    "node_gap_id",
    "source_id",
    "node_class",
    "needed_fields",
    "gap_reason",
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


NEED_TEMPLATES = {
    "CAN-SRC-002": {
        "need_id": "CAN-NEED-CAND-001",
        "need_class": "road_system_context",
        "source_quote_or_summary": (
            "Canada road-system context can inform local vocabulary but cannot "
            "promote a service role"
        ),
        "access_note": "bounded summary only; no SLA or approval inference",
        "blocked_claims": (
            "official_network;country_approval;guaranteed_sla;roi;"
            "funding_eligibility;construction_ready;compliance;endorsement;"
            "validation;public_readiness;external_readiness"
        ),
    },
    "CAN-SRC-004": {
        "need_id": "CAN-NEED-CAND-002",
        "need_class": "trade_corridor_vocabulary",
        "source_quote_or_summary": (
            "Trade-corridor vocabulary can inform held need and constraint rows "
            "but cannot prove funding eligibility or project priority"
        ),
        "access_note": "bounded summary only; no project priority or benefit proof inference",
        "blocked_claims": (
            "freight_benefit_proof;roi;funding_eligibility;project_priority;"
            "bottleneck_proof;resilience_proof;construction_priority;"
            "endorsement;validation;public_readiness;external_readiness"
        ),
    },
}

NODE_TEMPLATES = [
    {
        "node_gap_id": "CAN-NODE-GAP-001",
        "source_id": "CAN-SRC-005",
        "node_class": "port_gateway",
        "needed_fields": "port node; terminal node; access road; owner; source date; access note",
        "gap_reason": "official port or terminal source custody is not attached",
        "blocked_claims": (
            "port_endorsement;terminal_performance;node_completeness;"
            "throughput_proof;road_access_proof;construction_ready;"
            "guaranteed_sla;roi;compliance;endorsement;validation;"
            "public_readiness;external_readiness"
        ),
    },
    {
        "node_gap_id": "CAN-NODE-GAP-002",
        "source_id": "CAN-SRC-005",
        "node_class": "northern_or_rural_access_node",
        "needed_fields": "node id; label; node class; source owner; source date; access note",
        "gap_reason": "non-metro access node source custody is not attached",
        "blocked_claims": (
            "node_completeness;access_adequacy;route_promotion;"
            "construction_ready;guaranteed_sla;roi;compliance;endorsement;"
            "validation;public_readiness;external_readiness"
        ),
    },
]

TARGET_TEMPLATES = [
    {
        "target_gap_id": "CAN-TARGET-GAP-001",
        "role": "T1 national logistics spine",
        "needed_source": "adopted service target source and reliability basis",
    },
    {
        "target_gap_id": "CAN-TARGET-GAP-002",
        "role": "T2 regional connector",
        "needed_source": "adopted service target source and numeracy review",
    },
]

BACKLOG_ROWS = [
    {
        "role_lane": "Scope Keeper",
        "review_question": (
            "Does the dry run remain an internal parser fixture rather than an "
            "adapter promotion?"
        ),
        "trigger_output": "all output tables",
        "required_before": "any use beyond internal inspection",
        "hold_claims": "official_network;route_designation;approval;construction_ready;public_readiness;external_readiness",
        "result": "pending",
    },
    {
        "role_lane": "Citation Auditor",
        "review_question": (
            "Do candidate rows preserve source owner date access note evidence "
            "label and blocked claims?"
        ),
        "trigger_output": "canada_source_link_candidates;canada_source_need_candidates",
        "required_before": "parser implementation closeout",
        "hold_claims": "validation;endorsement;external_validation",
        "result": "pending",
    },
    {
        "role_lane": "Numeracy Checker",
        "review_question": "Are service targets assumption-labeled with no SLA or travel-time proof?",
        "trigger_output": "canada_service_target_candidates",
        "required_before": "any target use in a map or report",
        "hold_claims": "guaranteed_sla;travel_time_proof;delivery_commitment",
        "result": "pending",
    },
    {
        "role_lane": "Schematic Cartographer",
        "review_question": "Could candidate rows be mistaken for a source-bound map?",
        "trigger_output": "canada_source_link_candidates;canada_source_node_candidates",
        "required_before": "any map overlay or fixture replacement",
        "hold_claims": "official_network;map_proof;route_designation",
        "result": "pending",
    },
    {
        "role_lane": "V&V",
        "review_question": "Does every emitted row have a matching evidence-label row?",
        "trigger_output": "canada_adapter_evidence_labels",
        "required_before": "parser dry-run closeout",
        "hold_claims": "validation;public_readiness;external_readiness",
        "result": "pending",
    },
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def write_csv(path: Path, fieldnames: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def contract_for(rows: list[dict[str, str]], output_table: str) -> dict[str, str]:
    for row in rows:
        if row["output_table"] == output_table:
            return row
    raise KeyError(f"missing output contract for {output_table}")


def task_for(rows: list[dict[str, str]], task_id: str) -> dict[str, str]:
    for row in rows:
        if row["task_id"] == task_id:
            return row
    raise KeyError(f"missing preflight task {task_id}")


def main() -> None:
    source_pack = {row["source_id"]: row for row in read_csv(SOURCE_PACK)}
    preflight = read_csv(PREFLIGHT)
    contracts = read_csv(CONTRACT)

    link_contract = contract_for(contracts, "canada_source_link_candidates")
    target_contract = contract_for(contracts, "canada_service_target_candidates")

    link_rows = build_source_derived_link_rows(link_contract)

    need_rows: list[dict[str, str]] = []
    for task_id in ["CAN-PARSE-003", "CAN-PARSE-004"]:
        task = task_for(preflight, task_id)
        source = source_pack[task["source_id"]]
        template = NEED_TEMPLATES[task["source_id"]]
        need_rows.append(
            {
                "source_id": task["source_id"],
                "source_family": task["source_family"],
                "need_id": template["need_id"],
                "need_class": template["need_class"],
                "source_quote_or_summary": template["source_quote_or_summary"],
                "source_owner": source["owner_or_publisher"],
                "source_date": source["date_accessed"],
                "access_note": template["access_note"],
                "evidence_label": task["allowed_output_label"],
                "blocked_claims": template["blocked_claims"],
            }
        )

    node_rows = [
        {
            **template,
            "evidence_label": task_for(preflight, "CAN-PARSE-005")["allowed_output_label"],
        }
        for template in NODE_TEMPLATES
    ]
    target_rows = [
        {
            **template,
            "assumption_label": "planning_assumption_only",
            "evidence_label": task_for(preflight, "CAN-PARSE-006")["allowed_output_label"],
            "blocked_claims": target_contract["blocked_columns_or_values"],
        }
        for template in TARGET_TEMPLATES
    ]

    label_rows: list[dict[str, str]] = []
    for path, row_id_field, rows in [
        ("data/canada_source_link_candidates.csv", "route_id", link_rows),
        ("data/canada_source_need_candidates.csv", "need_id", need_rows),
        ("data/canada_source_node_candidates.csv", "node_gap_id", node_rows),
        ("data/canada_service_target_candidates.csv", "target_gap_id", target_rows),
    ]:
        for row in rows:
            label_rows.append(
                {
                    "artifact_path": path,
                    "row_id": row[row_id_field],
                    "evidence_label": row["evidence_label"],
                    "blocked_claims": row["blocked_claims"],
                    "source_id": row["source_id"] if "source_id" in row else "CAN-SRC-SLA-001",
                    "review_note": review_note_for(path, row[row_id_field]),
                }
            )

    write_csv(LINKS, LINK_FIELDS, link_rows)
    write_csv(NEEDS, NEED_FIELDS, need_rows)
    write_csv(NODES, NODE_FIELDS, node_rows)
    write_csv(TARGETS, TARGET_FIELDS, target_rows)
    write_csv(LABELS, LABEL_FIELDS, label_rows)
    write_csv(BACKLOG, BACKLOG_FIELDS, BACKLOG_ROWS)
    for path in [LINKS, NEEDS, NODES, TARGETS, LABELS, BACKLOG]:
        print(f"wrote {path}")


def review_note_for(path: str, row_id: str) -> str:
    if path.endswith("link_candidates.csv"):
        return "source-derived no-geometry candidate; internal link fixture only"
    if path.endswith("need_candidates.csv") and row_id == "CAN-NEED-CAND-001":
        return "bounded vocabulary only"
    if path.endswith("need_candidates.csv"):
        return "bounded need/constraint vocabulary only"
    if path.endswith("node_candidates.csv"):
        return "gap row only"
    return "assumption row only"


def build_source_derived_link_rows(link_contract: dict[str, str]) -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    for candidate in read_csv(EXTRACTION_CANDIDATES):
        rows.append(
            {
                "source_id": candidate["source_id"],
                "source_family": candidate["source_family"],
                "route_id": candidate["route_id"],
                "route_name": candidate["route_name"],
                "source_class": candidate["source_class"],
                "geometry_ref": candidate["geometry_ref"],
                "source_owner": candidate["source_owner"],
                "source_date": candidate["source_date"],
                "access_note": "source-derived no-geometry internal link fixture; not map or adapter use",
                "evidence_label": candidate["evidence_label"],
                "blocked_claims": link_contract["blocked_columns_or_values"],
            }
        )
    return rows


if __name__ == "__main__":
    main()
