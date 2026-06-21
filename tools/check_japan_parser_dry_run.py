#!/usr/bin/env python3
"""Validate Japan parser dry-run fixture tables."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "data" / "international-japan-parser-output-contract-001.csv"
TABLES = {
    "japan_source_link_candidates": ROOT / "data" / "japan_source_link_candidates.csv",
    "japan_source_need_candidates": ROOT / "data" / "japan_source_need_candidates.csv",
    "japan_source_node_candidates": ROOT / "data" / "japan_source_node_candidates.csv",
    "japan_service_target_candidates": ROOT / "data" / "japan_service_target_candidates.csv",
    "japan_adapter_evidence_labels": ROOT / "data" / "japan_adapter_evidence_labels.csv",
    "japan_adapter_review_backlog": ROOT / "data" / "japan_adapter_review_backlog.csv",
}
ROW_ID_FIELDS = {
    "japan_source_link_candidates": "route_or_layer_id",
    "japan_source_need_candidates": "need_id",
    "japan_source_node_candidates": "node_id",
    "japan_service_target_candidates": "target_gap_id",
}
ALLOWED_SOURCES = {
    "japan_source_link_candidates": {"JPN-SRC-004", "JPN-SRC-007"},
    "japan_source_need_candidates": {"JPN-SRC-001", "JPN-SRC-002", "JPN-SRC-003"},
    "japan_source_node_candidates": {"JPN-SRC-005", "JPN-SRC-006"},
}
ALLOWED_LABELS = {
    "japan_source_link_candidates": {"source-needed", "heuristic-held"},
    "japan_source_need_candidates": {"source-candidate"},
    "japan_source_node_candidates": {"source-candidate"},
    "japan_service_target_candidates": {"held"},
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
        for row in table_rows["japan_adapter_evidence_labels"]
    }
    for table, row_id_field in ROW_ID_FIELDS.items():
        for row in table_rows[table]:
            key = (f"data/{table}.csv", row[row_id_field], row["evidence_label"])
            if key not in labels:
                failures.append(f"{table}: missing evidence label for {row[row_id_field]}")
    link_labels = {row["evidence_label"] for row in table_rows["japan_source_link_candidates"]}
    if "source-needed" not in link_labels:
        failures.append("Japan link candidates must preserve a source-needed road-feature blocker")
    for row in table_rows["japan_source_link_candidates"]:
        if not row["geometry_ref"].startswith("not_accepted:"):
            failures.append("Japan link candidate accepted geometry")
    for row in table_rows["japan_source_node_candidates"]:
        if "not validated" not in row["access_note"]:
            failures.append(f"{row['node_id']} does not preserve node/geometry validation hold")
        for blocked in {"geometry_acceptance", "map_overlay", "terminal_performance", "road_access_proof"}:
            if blocked not in row["blocked_claims"].split(";"):
                failures.append(f"{row['node_id']} missing blocked claim {blocked}")
    for row in table_rows["japan_adapter_review_backlog"]:
        if row["result"] != "pending":
            failures.append(f"role backlog result is not pending for {row['role_lane']}")
    if failures:
        print("Japan parser dry-run gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Japan parser dry-run gate: PASS")
    print("  checked columns, labels, source limits, geometry holds, and review backlog")
    return 0


if __name__ == "__main__":
    sys.exit(main())
