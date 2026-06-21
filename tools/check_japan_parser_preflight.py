#!/usr/bin/env python3
"""Gate Japan parser preflight and output contract."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PREFLIGHT = ROOT / "data" / "international-japan-parser-preflight-001.csv"
CONTRACT = ROOT / "data" / "international-japan-parser-output-contract-001.csv"
FIELD_INVENTORY = ROOT / "data" / "international-japan-source-field-inventory-001.csv"

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
REQUIRED_TASKS = {f"JPN-PARSE-{i:03d}" for i in range(1, 11)}
REQUIRED_TABLES = {
    "japan_source_link_candidates",
    "japan_source_need_candidates",
    "japan_source_node_candidates",
    "japan_service_target_candidates",
    "japan_adapter_evidence_labels",
    "japan_adapter_review_backlog",
}
REQUIRED_BOUNDARY_TOKENS = {
    "official",
    "approval",
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
    _, inventory_rows = read_csv(FIELD_INVENTORY)
    inventory_sources = {row["source_id"] for row in inventory_rows}
    failures: list[str] = []
    if preflight_fields != PREFLIGHT_FIELDS:
        failures.append("Japan parser preflight columns do not match contract")
    if contract_fields != CONTRACT_FIELDS:
        failures.append("Japan parser output contract columns do not match contract")
    task_ids = {row["task_id"] for row in tasks}
    if task_ids != REQUIRED_TASKS:
        failures.append(f"Japan parser preflight task mismatch: {sorted(task_ids)}")
    tables = {row["output_table"] for row in contracts}
    if tables != REQUIRED_TABLES:
        failures.append(f"Japan parser output contract table mismatch: {sorted(tables)}")
    if not any(row["source_id"] == "JPN-SRC-004" and row["allowed_output_label"] == "source-needed" for row in tasks):
        failures.append("Japan parser preflight must preserve GSI source-needed task")
    for row in tasks:
        source_id = row["source_id"]
        if source_id not in inventory_sources and source_id not in {"carry-forward", "internal-roles"}:
            failures.append(f"{row['task_id']} source not covered by field inventory: {source_id}")
        if row["allowed_output_label"] not in {
            "source-candidate",
            "source-needed",
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
        if "geometry" not in row["acceptance_rule"].lower() and row["output_table"].endswith("_candidates"):
            failures.append(f"{row['output_table']} acceptance rule must preserve geometry boundary")
    if failures:
        print("Japan parser preflight gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Japan parser preflight gate: PASS")
    print("  checked parser tasks, output contracts, labels, field coverage, and claim boundaries")
    return 0


if __name__ == "__main__":
    sys.exit(main())
