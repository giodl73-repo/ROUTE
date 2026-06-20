#!/usr/bin/env python3
"""Gate EU Rhine-Alpine parser preflight and output contract."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PREFLIGHT = ROOT / "data" / "international-eu-rhine-alpine-parser-preflight-001.csv"
CONTRACT = ROOT / "data" / "international-eu-rhine-alpine-parser-output-contract-001.csv"

REQUIRED_TASKS = {f"EUR-PARSE-{i:03d}" for i in range(1, 9)}
REQUIRED_TABLES = {
    "eu_rhine_alpine_source_link_candidates",
    "eu_rhine_alpine_source_need_candidates",
    "eu_rhine_alpine_source_node_candidates",
    "eu_rhine_alpine_service_target_candidates",
    "eu_rhine_alpine_adapter_evidence_labels",
    "eu_rhine_alpine_adapter_review_backlog",
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
    _, tasks = read_csv(PREFLIGHT)
    _, contracts = read_csv(CONTRACT)
    failures: list[str] = []

    task_ids = {row["task_id"] for row in tasks}
    if task_ids != REQUIRED_TASKS:
        failures.append(f"EU parser preflight task mismatch: {sorted(task_ids)}")
    tables = {row["output_table"] for row in contracts}
    if tables != REQUIRED_TABLES:
        failures.append(f"EU parser output contract table mismatch: {sorted(tables)}")

    for row in tasks:
        if row["allowed_output_label"] not in {
            "source-candidate",
            "source-needed",
            "held",
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

    if failures:
        print("EU Rhine-Alpine parser preflight gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("EU Rhine-Alpine parser preflight gate: PASS")
    print("  checked parser tasks, output contracts, labels, and claim boundaries")
    return 0


if __name__ == "__main__":
    sys.exit(main())
