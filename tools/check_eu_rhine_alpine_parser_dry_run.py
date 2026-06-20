#!/usr/bin/env python3
"""Validate EU Rhine-Alpine parser dry-run fixture tables."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "data" / "international-eu-rhine-alpine-parser-output-contract-001.csv"
TABLES = {
    "eu_rhine_alpine_source_link_candidates": ROOT / "data" / "eu_rhine_alpine_source_link_candidates.csv",
    "eu_rhine_alpine_source_need_candidates": ROOT / "data" / "eu_rhine_alpine_source_need_candidates.csv",
    "eu_rhine_alpine_source_node_candidates": ROOT / "data" / "eu_rhine_alpine_source_node_candidates.csv",
    "eu_rhine_alpine_service_target_candidates": ROOT / "data" / "eu_rhine_alpine_service_target_candidates.csv",
    "eu_rhine_alpine_adapter_evidence_labels": ROOT / "data" / "eu_rhine_alpine_adapter_evidence_labels.csv",
    "eu_rhine_alpine_adapter_review_backlog": ROOT / "data" / "eu_rhine_alpine_adapter_review_backlog.csv",
}
ROW_ID_FIELDS = {
    "eu_rhine_alpine_source_link_candidates": "route_or_layer_id",
    "eu_rhine_alpine_source_need_candidates": "need_id",
    "eu_rhine_alpine_source_node_candidates": "node_id",
    "eu_rhine_alpine_service_target_candidates": "target_gap_id",
}
ALLOWED_SOURCES = {
    "eu_rhine_alpine_source_link_candidates": {"EUR-SRC-002", "EUR-SRC-003"},
    "eu_rhine_alpine_source_need_candidates": {"EUR-SRC-001", "EUR-SRC-004"},
    "eu_rhine_alpine_source_node_candidates": {"EUR-SRC-003"},
}
REQUIRED_NODE_IDS = {"NLRTM", "BEANR", "ITGOA", "CHBSL", "DEDUI"}


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
        if contract["required_label"] != "carry-forward":
            for row in rows:
                if row.get("evidence_label") != contract["required_label"]:
                    failures.append(f"{table}: row label does not match contract")
        for row in rows:
            if "blocked_claims" in fields and not row.get("blocked_claims"):
                failures.append(f"{table}: row missing blocked_claims")
            if table in ALLOWED_SOURCES and row["source_id"] not in ALLOWED_SOURCES[table]:
                failures.append(f"{table}: source {row['source_id']} not allowed")

    labels = {
        (row["artifact_path"], row["row_id"], row["evidence_label"])
        for row in table_rows["eu_rhine_alpine_adapter_evidence_labels"]
    }
    for table, row_id_field in ROW_ID_FIELDS.items():
        for row in table_rows[table]:
            key = (f"data/{table}.csv", row[row_id_field], row["evidence_label"])
            if key not in labels:
                failures.append(f"{table}: missing evidence label for {row[row_id_field]}")

    for row in table_rows["eu_rhine_alpine_source_link_candidates"]:
        if not row["geometry_ref"].startswith("not_accepted:"):
            failures.append("EU link candidate accepted geometry")
    actual_node_ids = {row["node_id"] for row in table_rows["eu_rhine_alpine_source_node_candidates"]}
    if REQUIRED_NODE_IDS - actual_node_ids:
        failures.append(f"EU node candidate fixture missing nodes: {sorted(REQUIRED_NODE_IDS - actual_node_ids)}")
    for row in table_rows["eu_rhine_alpine_source_node_candidates"]:
        if "internal node fixture" not in row["access_note"]:
            failures.append(f"{row['node_id']} missing internal node fixture access note")
        if "geometry not read or accepted" not in row["access_note"]:
            failures.append(f"{row['node_id']} does not preserve no-geometry posture")
        for blocked in {"geometry_acceptance", "map_overlay", "terminal_performance", "road_access_proof"}:
            if blocked not in row["blocked_claims"].split(";"):
                failures.append(f"{row['node_id']} missing blocked claim {blocked}")
    for row in table_rows["eu_rhine_alpine_adapter_review_backlog"]:
        if row["result"] != "pending":
            failures.append(f"role backlog result is not pending for {row['role_lane']}")

    if failures:
        print("EU Rhine-Alpine parser dry-run gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("EU Rhine-Alpine parser dry-run gate: PASS")
    print("  checked columns, labels, source limits, geometry holds, and review backlog")
    return 0


if __name__ == "__main__":
    sys.exit(main())
