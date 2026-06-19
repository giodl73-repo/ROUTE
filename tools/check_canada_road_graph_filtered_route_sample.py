#!/usr/bin/env python3
"""Gate bounded Canada road-graph filtered route sample."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-canada-road-graph-filtered-route-sample-001.csv"

FIELDS = [
    "sample_id",
    "source_id",
    "extraction_window",
    "query_ref",
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
        failures.append("filtered route sample columns do not match required contract")
    if not rows:
        failures.append("filtered route sample has no rows")
    if len(rows) > 5:
        failures.append("filtered route sample exceeds bounded five-row limit")
    if not any(row["route_number_1"] not in {"", "None"} for row in rows):
        failures.append("filtered route sample has no usable route number")
    if not any(row["route_name_1"] not in {"", "None"} for row in rows):
        failures.append("filtered route sample has no usable route name")
    for row in rows:
        if row["source_id"] != "CAN-SRC-001":
            failures.append(f"{row['sample_id']} is not CAN-SRC-001")
        if row["geometry_status"] != "not-requested":
            failures.append(f"{row['sample_id']} requested geometry")
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['sample_id']} accepts evidence")
        if not row["blocked_claims"]:
            failures.append(f"{row['sample_id']} missing blocked claims")

    if failures:
        print("Canada road-graph filtered route sample gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada road-graph filtered route sample gate: PASS")
    print("  checked route identifiers, bounded row count, no-geometry posture, and not-accepted evidence status")
    return 0


if __name__ == "__main__":
    sys.exit(main())
