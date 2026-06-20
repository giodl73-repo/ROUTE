#!/usr/bin/env python3
"""Gate EU current-corridor rebase review."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-current-corridor-rebase-001.csv"

FIELDS = [
    "review_id",
    "current_corridor_source_id",
    "legacy_context_source_id",
    "observed_current_corridor_hint",
    "observed_legacy_context_hint",
    "rebase_decision",
    "blocked_replacement_surface",
    "blocked_claims",
    "required_next_step",
]
REQUIRED_BLOCKS = {
    "fixture_replacement",
    "internal_adapter_proof",
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
        failures.append("EU current-corridor rebase columns do not match contract")
    if len(rows) != 1:
        failures.append("EU current-corridor rebase must have one decision row")
    for row in rows:
        if row["rebase_decision"] != "current_corridor_rebase_required_before_replacement":
            failures.append("EU rebase decision must block replacement")
        if row["current_corridor_source_id"] != "EUR-SRC-001" or row["legacy_context_source_id"] != "EUR-SRC-004":
            failures.append("EU rebase decision must compare current corridor and legacy context sources")
        if "current_corridor_set_rebase_needed" not in row["observed_current_corridor_hint"]:
            failures.append("EU rebase decision must preserve current-corridor warning")
        if "before fixture replacement" not in row["required_next_step"]:
            failures.append("EU rebase next step must block fixture replacement")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"EU rebase decision missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine current-corridor rebase gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine current-corridor rebase gate: PASS")
    print("  checked current-corridor warning, replacement block, and claim boundaries")
    return 0


if __name__ == "__main__":
    sys.exit(main())
