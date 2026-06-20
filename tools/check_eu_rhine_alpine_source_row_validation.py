#!/usr/bin/env python3
"""Gate EU Rhine-Alpine bounded source-row validation ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-source-row-validation-001.csv"

FIELDS = [
    "validation_id",
    "artifact_path",
    "row_id",
    "source_id",
    "row_label",
    "inventory_status",
    "validation_result",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
ALLOWED_RESULTS = {
    "bounded_metadata_match_not_validated",
    "source_gap_preserved",
    "held_assumption_preserved",
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("EU source-row validation columns do not match contract")
    if len(rows) != 6:
        failures.append("EU source-row validation must cover six dry-run rows")
    for row in rows:
        if row["validation_result"] not in ALLOWED_RESULTS:
            failures.append(f"{row['row_id']} unsupported validation result")
        if "validated" in row["validation_result"] and row["validation_result"] != "bounded_metadata_match_not_validated":
            failures.append(f"{row['row_id']} overclaims validation")
        if row["allowed_use"] not in {
            "internal_parser_inspection_only",
            "gap_tracking_only",
            "assumption_tracking_only",
        }:
            failures.append(f"{row['row_id']} unsupported allowed use")
        if "before" not in row["next_action"]:
            failures.append(f"{row['row_id']} next action must preserve before dependency")
        if "external_validation" not in row["blocked_claims"]:
            failures.append(f"{row['row_id']} missing external validation block")
    if failures:
        print("EU Rhine-Alpine source-row validation gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine source-row validation gate: PASS")
    print("  checked dry-run row coverage, bounded validation posture, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
