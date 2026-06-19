#!/usr/bin/env python3
"""Gate Canada source-row validation before any fixture replacement."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
VALIDATION = ROOT / "data" / "international-canada-source-row-validation-001.csv"

FIELDS = [
    "validation_id",
    "candidate_id",
    "source_sample_id",
    "object_id",
    "route_id_match",
    "route_name_match",
    "source_class_match",
    "geometry_posture_match",
    "source_id_match",
    "validation_status",
    "blocked_claims",
    "next_action",
]

REQUIRED_BLOCKS = {
    "geometry_acceptance",
    "fixture_replacement",
    "parsed_adapter",
    "official_network",
    "route_designation",
    "engineering_precision",
    "agency_approval",
    "construction_ready",
    "guaranteed_sla",
    "roi",
    "eligibility",
    "compliance",
    "endorsement",
    "validation",
    "public_readiness",
    "external_readiness",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    fields, rows = read_csv(VALIDATION)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("source-row validation columns do not match required contract")
    if len(rows) != 5:
        failures.append("source-row validation must cover the bounded five-row sample")
    for row in rows:
        for field in [
            "route_id_match",
            "route_name_match",
            "source_class_match",
            "geometry_posture_match",
            "source_id_match",
        ]:
            if row[field] != "true":
                failures.append(f"{row['validation_id']} has failed {field}")
        if row["validation_status"] != "candidate_source_row_validated":
            failures.append(f"{row['validation_id']} is not validated")
        blocked = set(row["blocked_claims"].split(";"))
        missing = REQUIRED_BLOCKS - blocked
        if missing:
            failures.append(f"{row['validation_id']} missing blocked claims: {sorted(missing)}")
        if "fixture replacement" not in row["next_action"]:
            failures.append(f"{row['validation_id']} missing fixture replacement hold")

    if failures:
        print("Canada source-row validation gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada source-row validation gate: PASS")
    print("  checked extracted rows against filtered source sample and preserved promotion holds")
    return 0


if __name__ == "__main__":
    sys.exit(main())
