#!/usr/bin/env python3
"""Gate EU source-content row validation."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-source-content-row-validation-001.csv"

FIELDS = [
    "validation_id",
    "candidate_id",
    "source_id",
    "sample_id",
    "candidate_status",
    "sample_status",
    "validation_result",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_BLOCKS = {
    "geometry_acceptance",
    "guaranteed_sla",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("EU source-content row validation columns do not match contract")
    if len(rows) != 2:
        failures.append("EU source-content row validation must cover two extraction candidates")
    for row in rows:
        if row["validation_result"] != "source_content_candidate_row_validated_not_road_feature":
            failures.append(f"{row['candidate_id']} has unsupported validation result")
        if row["allowed_use"] != "internal_parser_inspection_only":
            failures.append(f"{row['candidate_id']} has unsupported allowed use")
        if row["candidate_status"] != "source_content_extraction_candidate_not_promoted":
            failures.append(f"{row['candidate_id']} promotes candidate status")
        if "before fixture replacement" not in row["next_action"]:
            failures.append(f"{row['candidate_id']} must preserve fixture replacement dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['candidate_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine source-content row validation gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine source-content row validation gate: PASS")
    print("  checked row coverage, not-promoted status, and road-feature boundary")
    return 0


if __name__ == "__main__":
    sys.exit(main())
