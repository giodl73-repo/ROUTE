#!/usr/bin/env python3
"""Validate the generated Canada parser dry-run fixture.

This gate checks the dry-run contract only. It does not validate Canadian source
payloads, promote source-bound rows, or accept any official network or service
claim.
"""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "data" / "international-canada-parser-output-contract-001.csv"
TABLES = {
    "canada_source_link_candidates": ROOT / "data" / "canada_source_link_candidates.csv",
    "canada_source_need_candidates": ROOT / "data" / "canada_source_need_candidates.csv",
    "canada_source_node_candidates": ROOT / "data" / "canada_source_node_candidates.csv",
    "canada_service_target_candidates": ROOT / "data" / "canada_service_target_candidates.csv",
    "canada_adapter_evidence_labels": ROOT / "data" / "canada_adapter_evidence_labels.csv",
    "canada_adapter_review_backlog": ROOT / "data" / "canada_adapter_review_backlog.csv",
}

ALLOWED_SOURCES = {
    "canada_source_link_candidates": {"CAN-SRC-001", "CAN-SRC-003"},
    "canada_source_need_candidates": {"CAN-SRC-002", "CAN-SRC-004"},
    "canada_source_node_candidates": {"CAN-SRC-005"},
}
ROW_ID_FIELDS = {
    "canada_source_link_candidates": "route_id",
    "canada_source_need_candidates": "need_id",
    "canada_source_node_candidates": "node_gap_id",
    "canada_service_target_candidates": "target_gap_id",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def fail(message: str, failures: list[str]) -> None:
    failures.append(message)


def expected_columns(contract_row: dict[str, str]) -> list[str]:
    return contract_row["required_columns"].split(";")


def contract_rows() -> dict[str, dict[str, str]]:
    _, rows = read_csv(CONTRACT)
    return {row["output_table"]: row for row in rows}


def validate_table_shape(
    table: str,
    fields: list[str],
    rows: list[dict[str, str]],
    contract: dict[str, str],
    failures: list[str],
) -> None:
    expected = expected_columns(contract)
    if fields != expected:
        fail(f"{table}: columns {fields} did not match contract {expected}", failures)

    minimum_rows = int(contract["minimum_rows_allowed"])
    if len(rows) < minimum_rows:
        fail(f"{table}: row count {len(rows)} below minimum {minimum_rows}", failures)

    required_label = contract["required_label"]
    if required_label not in {"carry-forward"}:
        for index, row in enumerate(rows, start=1):
            if row.get("evidence_label") != required_label:
                fail(
                    f"{table}: row {index} label {row.get('evidence_label')} "
                    f"did not match {required_label}",
                    failures,
                )

    blocked_values = set(contract["blocked_columns_or_values"].split(";"))
    for index, row in enumerate(rows, start=1):
        if not row.get("blocked_claims") and "blocked_claims" in fields:
            fail(f"{table}: row {index} missing blocked_claims", failures)
        row_values = set((row.get("blocked_claims") or "").split(";"))
        if "blocked_claims" in fields and not row_values.intersection(blocked_values):
            fail(f"{table}: row {index} blocked_claims do not overlap contract values", failures)


def validate_source_limits(table: str, rows: list[dict[str, str]], failures: list[str]) -> None:
    allowed = ALLOWED_SOURCES.get(table)
    if not allowed:
        return
    for row in rows:
        source_id = row["source_id"]
        if source_id not in allowed:
            fail(f"{table}: source_id {source_id} not allowed by dry-run contract", failures)


def validate_evidence_coverage(
    table_rows: dict[str, list[dict[str, str]]],
    evidence_rows: list[dict[str, str]],
    failures: list[str],
) -> None:
    labels = {
        (row["artifact_path"], row["row_id"], row["evidence_label"])
        for row in evidence_rows
    }
    for table, row_id_field in ROW_ID_FIELDS.items():
        artifact = f"data/{table}.csv"
        for row in table_rows[table]:
            key = (artifact, row[row_id_field], row["evidence_label"])
            if key not in labels:
                fail(f"{table}: missing evidence label for {row[row_id_field]}", failures)


def validate_backlog(rows: list[dict[str, str]], failures: list[str]) -> None:
    expected_lanes = {
        "Scope Keeper",
        "Citation Auditor",
        "Numeracy Checker",
        "Schematic Cartographer",
        "V&V",
    }
    actual_lanes = {row["role_lane"] for row in rows}
    missing = sorted(expected_lanes - actual_lanes)
    if missing:
        fail(f"canada_adapter_review_backlog: missing role lanes {missing}", failures)
    for row in rows:
        if row["result"] != "pending":
            fail(
                f"canada_adapter_review_backlog: {row['role_lane']} result is not pending",
                failures,
            )


def main() -> int:
    contracts = contract_rows()
    failures: list[str] = []
    table_rows: dict[str, list[dict[str, str]]] = {}

    for table, path in TABLES.items():
        fields, rows = read_csv(path)
        table_rows[table] = rows
        validate_table_shape(table, fields, rows, contracts[table], failures)
        validate_source_limits(table, rows, failures)

    validate_evidence_coverage(
        table_rows,
        table_rows["canada_adapter_evidence_labels"],
        failures,
    )
    validate_source_derived_link_replacement(
        table_rows["canada_source_link_candidates"],
        failures,
    )
    validate_backlog(table_rows["canada_adapter_review_backlog"], failures)

    if failures:
        print("Canada parser dry-run gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada parser dry-run gate: PASS")
    print("  checked columns, labels, source limits, evidence coverage, and review backlog")
    return 0


def validate_source_derived_link_replacement(
    rows: list[dict[str, str]],
    failures: list[str],
) -> None:
    if len(rows) != 5:
        fail("canada_source_link_candidates: expected 5 source-derived rows", failures)
    for index, row in enumerate(rows, start=1):
        if row["source_id"] != "CAN-SRC-001":
            fail(f"canada_source_link_candidates: row {index} is not CAN-SRC-001", failures)
        if not row["geometry_ref"].startswith("not_requested:"):
            fail(f"canada_source_link_candidates: row {index} accepted geometry", failures)
        if "internal link fixture" not in row["access_note"]:
            fail(
                f"canada_source_link_candidates: row {index} missing internal fixture access note",
                failures,
            )
        if row["route_id"].startswith("CAN-LINK-CAND-"):
            fail(f"canada_source_link_candidates: row {index} is still a placeholder", failures)


if __name__ == "__main__":
    sys.exit(main())
