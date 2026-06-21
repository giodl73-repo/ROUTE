#!/usr/bin/env python3
"""Validate China parser dry-run fixture tables."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "data" / "international-china-parser-output-contract-001.csv"
TABLES = {
    "china_source_link_candidates": ROOT / "data" / "china_source_link_candidates.csv",
    "china_source_need_candidates": ROOT / "data" / "china_source_need_candidates.csv",
    "china_source_node_candidates": ROOT / "data" / "china_source_node_candidates.csv",
    "china_service_target_candidates": ROOT / "data" / "china_service_target_candidates.csv",
    "china_adapter_evidence_labels": ROOT / "data" / "china_adapter_evidence_labels.csv",
    "china_adapter_review_backlog": ROOT / "data" / "china_adapter_review_backlog.csv",
}
ROW_ID_FIELDS = {
    "china_source_link_candidates": "route_or_layer_id",
    "china_source_need_candidates": "need_id",
    "china_source_node_candidates": "node_id",
    "china_service_target_candidates": "target_gap_id",
}
ALLOWED_SOURCES = {
    "china_source_link_candidates": {"CHN-SRC-004", "CHN-SRC-006"},
    "china_source_need_candidates": {"CHN-SRC-001", "CHN-SRC-002", "CHN-SRC-003"},
    "china_source_node_candidates": {"CHN-SRC-005"},
}
ALLOWED_LABELS = {
    "china_source_link_candidates": {"context-only", "heuristic-held"},
    "china_source_need_candidates": {"source-candidate"},
    "china_source_node_candidates": {"source-candidate"},
    "china_service_target_candidates": {"held"},
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    _, contract_rows = read_csv(CONTRACT)
    contracts = {row["output_table"]: row for row in contract_rows}
    failures: list[str] = []
    table_rows: dict[str, list[dict[str, str]]] = {}
    for table, path in TABLES.items():
        fields, rows = read_csv(path)
        table_rows[table] = rows
        contract = contracts[table]
        expected = contract["required_columns"].split(";")
        if fields != expected:
            failures.append(f"{table}: columns do not match contract")
        if len(rows) < int(contract["minimum_rows_allowed"]):
            failures.append(f"{table}: row count below minimum")
        if table in ALLOWED_LABELS:
            for row in rows:
                if row.get("evidence_label") not in ALLOWED_LABELS[table]:
                    failures.append(f"{table}: row label does not match allowed labels")
        for row in rows:
            if "blocked_claims" in fields and not row.get("blocked_claims"):
                failures.append(f"{table}: row missing blocked_claims")
            if table in ALLOWED_SOURCES and row["source_id"] not in ALLOWED_SOURCES[table]:
                failures.append(f"{table}: source {row['source_id']} not allowed")

    labels = {
        (row["artifact_path"], row["row_id"], row["evidence_label"])
        for row in table_rows["china_adapter_evidence_labels"]
    }
    for table, row_id_field in ROW_ID_FIELDS.items():
        for row in table_rows[table]:
            key = (f"data/{table}.csv", row[row_id_field], row["evidence_label"])
            if key not in labels:
                failures.append(f"{table}: missing evidence label for {row[row_id_field]}")
    link_labels = {row["evidence_label"] for row in table_rows["china_source_link_candidates"]}
    if "context-only" not in link_labels:
        failures.append("China link candidates must preserve a context-only standards row")
    if "heuristic-held" not in link_labels:
        failures.append("China link candidates must preserve a heuristic-held hierarchy row")
    for row in table_rows["china_source_link_candidates"]:
        if not row["geometry_ref"].startswith("not_accepted:"):
            failures.append("China link candidate accepted geometry")
    for row in table_rows["china_source_need_candidates"]:
        if "policy" in row["source_quote_or_summary"].lower() and "not policy alignment" not in row["source_quote_or_summary"].lower():
            failures.append(f"{row['need_id']} risks policy-alignment wording")
    for row in table_rows["china_source_node_candidates"]:
        if "not validated" not in row["access_note"]:
            failures.append(f"{row['node_id']} does not preserve node/geometry validation hold")
        for blocked in {"geometry_acceptance", "map_overlay", "terminal_performance", "road_access_proof"}:
            if blocked not in row["blocked_claims"].split(";"):
                failures.append(f"{row['node_id']} missing blocked claim {blocked}")
    for row in table_rows["china_adapter_review_backlog"]:
        if row["result"] != "pending":
            failures.append(f"role backlog result is not pending for {row['role_lane']}")
    if failures:
        print("China parser dry-run gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("China parser dry-run gate: PASS")
    print("  checked columns, labels, source limits, geometry holds, and review backlog")
    return 0


if __name__ == "__main__":
    sys.exit(main())
