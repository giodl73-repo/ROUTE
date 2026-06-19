#!/usr/bin/env python3
"""Gate Canada source field inventory before parser extraction."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ACCESS = ROOT / "data" / "international-canada-source-payload-access-001.csv"
INVENTORY = ROOT / "data" / "international-canada-source-field-inventory-001.csv"

FIELDS = [
    "inventory_id",
    "source_id",
    "source_family",
    "inventory_method",
    "inventory_status",
    "field_name",
    "field_type",
    "field_alias",
    "required_field_match",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    _, access_rows = read_csv(ACCESS)
    fields, rows = read_csv(INVENTORY)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("field inventory columns do not match required contract")
    source_ids = {row["source_id"] for row in access_rows}
    inventory_ids = {row["source_id"] for row in rows}
    missing = sorted(source_ids - inventory_ids)
    if missing:
        failures.append(f"field inventory missing source rows {missing}")

    road_rows = [row for row in rows if row["source_id"] == "CAN-SRC-001"]
    if not road_rows:
        failures.append("CAN-SRC-001 has no field inventory rows")
    if road_rows and all(row["inventory_status"] != "field-candidate-not-accepted" for row in road_rows):
        failures.append("CAN-SRC-001 has no candidate field rows")
    if road_rows and all(row["required_field_match"] == "unmatched" for row in road_rows):
        failures.append("CAN-SRC-001 has no candidate required-field matches")

    for row in rows:
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['inventory_id']} accepts evidence")
        if not row["blocked_claims"]:
            failures.append(f"{row['inventory_id']} missing blocked claims")

    if failures:
        print("Canada source field inventory gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada source field inventory gate: PASS")
    print("  checked source coverage, road-graph field candidates, and not-accepted posture")
    return 0


if __name__ == "__main__":
    sys.exit(main())
