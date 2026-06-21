#!/usr/bin/env python3
"""Gate China parser preflight and output contract."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PREFLIGHT = ROOT / "data" / "international-china-parser-preflight-001.csv"
CONTRACT = ROOT / "data" / "international-china-parser-output-contract-001.csv"
SOURCE_PACK = ROOT / "data" / "international-china-adapter-source-pack-001.csv"
PAYLOAD_ACCESS = ROOT / "data" / "international-china-source-payload-access-001.csv"

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
REQUIRED_TASKS = {f"CHN-PARSE-{i:03d}" for i in range(1, 10)}
REQUIRED_TABLES = {
    "china_source_link_candidates",
    "china_source_need_candidates",
    "china_source_node_candidates",
    "china_service_target_candidates",
    "china_adapter_evidence_labels",
    "china_adapter_review_backlog",
}
REQUIRED_BOUNDARY_TOKENS = {
    "official",
    "policy",
    "sla",
    "roi",
    "endorsement",
    "validation",
    "public",
    "external",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    preflight_fields, tasks = read_csv(PREFLIGHT)
    contract_fields, contracts = read_csv(CONTRACT)
    _, source_rows = read_csv(SOURCE_PACK)
    _, payload_rows = read_csv(PAYLOAD_ACCESS)
    source_ids = {row["source_id"] for row in source_rows}
    payload_ids = {row["source_id"] for row in payload_rows}
    failures: list[str] = []
    if preflight_fields != PREFLIGHT_FIELDS:
        failures.append("China parser preflight columns do not match contract")
    if contract_fields != CONTRACT_FIELDS:
        failures.append("China parser output contract columns do not match contract")
    task_ids = {row["task_id"] for row in tasks}
    if task_ids != REQUIRED_TASKS:
        failures.append(f"China parser preflight task mismatch: {sorted(task_ids)}")
    tables = {row["output_table"] for row in contracts}
    if tables != REQUIRED_TABLES:
        failures.append(f"China parser output contract table mismatch: {sorted(tables)}")
    if source_ids != payload_ids:
        failures.append("China source-pack and payload-access source IDs differ")
    if not any(row["source_id"] == "CHN-SRC-004" and row["allowed_output_label"] == "context-only" for row in tasks):
        failures.append("China parser preflight must keep highway standards context-only")
    for row in tasks:
        source_id = row["source_id"]
        if source_id not in source_ids and source_id not in {"carry-forward", "internal-roles"}:
            failures.append(f"{row['task_id']} source not covered by China source pack: {source_id}")
        if source_id in source_ids and source_id not in payload_ids:
            failures.append(f"{row['task_id']} source not covered by China payload access: {source_id}")
        if row["allowed_output_label"] not in {
            "source-candidate",
            "context-only",
            "held",
            "heuristic-held",
            "carry-forward",
        }:
            failures.append(f"{row['task_id']} has unsupported label {row['allowed_output_label']}")
        if "before" not in row["next_action"]:
            failures.append(f"{row['task_id']} next_action must name a before dependency")
        boundary = row["claim_boundary"].lower()
        for token in REQUIRED_BOUNDARY_TOKENS:
            if token not in boundary:
                failures.append(f"{row['task_id']} missing boundary token {token}")
    for row in contracts:
        if not row["required_columns"]:
            failures.append(f"{row['output_table']} missing required columns")
        if "claim" not in row["claim_boundary"]:
            failures.append(f"{row['output_table']} missing claim boundary")
        blocked = row["blocked_columns_or_values"].lower()
        boundary = row["claim_boundary"].lower()
        for token in REQUIRED_BOUNDARY_TOKENS:
            if token not in blocked and token not in boundary:
                failures.append(f"{row['output_table']} missing blocked token {token}")
        if row["output_table"].endswith("_candidates") and "geometry" not in row["acceptance_rule"].lower():
            failures.append(f"{row['output_table']} acceptance rule must preserve geometry boundary")
    if failures:
        print("China parser preflight gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("China parser preflight gate: PASS")
    print("  checked parser tasks, output contracts, payload coverage, labels, and claim boundaries")
    return 0


if __name__ == "__main__":
    sys.exit(main())
