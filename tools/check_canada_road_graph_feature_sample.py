#!/usr/bin/env python3
"""Gate bounded Canada road-graph feature sample."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-canada-road-graph-feature-sample-001.csv"

FIELDS = [
    "sample_id",
    "source_id",
    "object_id",
    "route_number_1",
    "route_name_1",
    "road_class",
    "type_code",
    "nhs_description",
    "sample_method",
    "geometry_status",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    fields, rows = read_csv(SAMPLE)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("feature sample columns do not match required contract")
    if not rows:
        failures.append("feature sample has no rows")
    if len(rows) > 5:
        failures.append("feature sample exceeds bounded five-row limit")
    for row in rows:
        if row["source_id"] != "CAN-SRC-001":
            failures.append(f"{row['sample_id']} is not CAN-SRC-001")
        if row["geometry_status"] != "not-requested":
            failures.append(f"{row['sample_id']} requested geometry")
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['sample_id']} accepts evidence")
        if not row["object_id"]:
            failures.append(f"{row['sample_id']} missing object_id")
        if not row["blocked_claims"]:
            failures.append(f"{row['sample_id']} missing blocked claims")

    if failures:
        print("Canada road-graph feature sample gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada road-graph feature sample gate: PASS")
    print("  checked bounded row count, no-geometry posture, and not-accepted evidence status")
    return 0


if __name__ == "__main__":
    sys.exit(main())
