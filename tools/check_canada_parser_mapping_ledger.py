#!/usr/bin/env python3
"""Gate Canada parser mapping ledger before fixture replacement."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-canada-parser-mapping-ledger-001.csv"

FIELDS = [
    "mapping_id",
    "source_id",
    "source_field",
    "field_alias",
    "sample_values",
    "target_output_table",
    "target_column",
    "mapping_role",
    "mapping_status",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]
REQUIRED_TARGETS = {"route_id", "route_name", "source_class", "geometry_ref"}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    fields, rows = read_csv(LEDGER)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("parser mapping ledger columns do not match required contract")
    targets = {row["target_column"] for row in rows}
    missing_targets = sorted(REQUIRED_TARGETS - targets)
    if missing_targets:
        failures.append(f"parser mapping ledger missing target columns {missing_targets}")
    for row in rows:
        if row["source_id"] != "CAN-SRC-001":
            failures.append(f"{row['mapping_id']} is not scoped to CAN-SRC-001")
        if row["target_output_table"] != "canada_source_link_candidates":
            failures.append(f"{row['mapping_id']} targets unexpected output table")
        if row["mapping_status"] != "candidate_mapping_not_promoted":
            failures.append(f"{row['mapping_id']} promotes mapping status")
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['mapping_id']} accepts evidence")
        if not row["blocked_claims"]:
            failures.append(f"{row['mapping_id']} missing blocked claims")
    if not any(row["target_column"] == "geometry_ref" and row["sample_values"] == "not-requested" for row in rows):
        failures.append("geometry mapping must remain a reference candidate with no geometry sample")

    if failures:
        print("Canada parser mapping ledger gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada parser mapping ledger gate: PASS")
    print("  checked target coverage, candidate status, no-geometry posture, and not-accepted evidence status")
    return 0


if __name__ == "__main__":
    sys.exit(main())
