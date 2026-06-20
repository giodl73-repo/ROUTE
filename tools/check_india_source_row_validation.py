#!/usr/bin/env python3
"""Gate India bounded source-row validation ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-india-source-row-validation-001.csv"

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
    "heuristic_fixture_hold_preserved",
    "held_assumption_preserved",
}
ALLOWED_USE = {
    "internal_parser_inspection_only",
    "fixture_gap_tracking_only",
    "assumption_tracking_only",
}
REQUIRED_BLOCKS = {
    "source_row_validation",
    "fixture_replacement",
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
        failures.append("India source-row validation columns do not match contract")
    if len(rows) != 6:
        failures.append("India source-row validation must cover six dry-run rows")
    labels = {row["row_label"] for row in rows}
    if not {"source-candidate", "heuristic-held", "held"}.issubset(labels):
        failures.append(f"India source-row validation missing expected labels: {sorted(labels)}")
    for row in rows:
        if row["validation_result"] not in ALLOWED_RESULTS:
            failures.append(f"{row['row_id']} unsupported validation result")
        if "validated" in row["validation_result"] and row["validation_result"] != "bounded_metadata_match_not_validated":
            failures.append(f"{row['row_id']} overclaims validation")
        if row["allowed_use"] not in ALLOWED_USE:
            failures.append(f"{row['row_id']} unsupported allowed use")
        if "before" not in row["next_action"]:
            failures.append(f"{row['row_id']} next action must preserve before dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['row_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("India source-row validation gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("India source-row validation gate: PASS")
    print("  checked dry-run row coverage, bounded validation posture, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
