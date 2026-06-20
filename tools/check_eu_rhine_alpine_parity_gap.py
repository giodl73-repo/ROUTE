#!/usr/bin/env python3
"""Gate EU Rhine-Alpine parity gap ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-parity-gap-001.csv"

FIELDS = [
    "gap_id",
    "canada_parity_surface",
    "eu_current_artifact",
    "eu_status",
    "parity_decision",
    "blocked_claims",
    "required_next_step",
]
REQUIRED_DECISIONS = {
    "complete_for_pre_validation_layer",
    "complete_for_dry_run_layer",
    "partial_not_canada_equivalent",
    "blocked",
}
REQUIRED_BLOCKS = {
    "internal_adapter_proof",
    "official_corridor_designation",
    "member_state_approval",
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
        failures.append("EU parity gap columns do not match contract")
    if len(rows) != 6:
        failures.append("EU parity gap must have six rows")
    if not any(row["parity_decision"] == "blocked" for row in rows):
        failures.append("EU parity gap must preserve blocked parity rows")
    for row in rows:
        if row["parity_decision"] not in REQUIRED_DECISIONS:
            failures.append(f"{row['gap_id']} unsupported parity decision")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['gap_id']} missing blocked claims: {sorted(missing)}")
        if "before" not in row["required_next_step"] and not row["required_next_step"].startswith("continue"):
            failures.append(f"{row['gap_id']} next step must preserve before-promotion dependency")
        if "internal_adapter_proof" not in row["blocked_claims"]:
            failures.append(f"{row['gap_id']} must block internal proof claim")
    if failures:
        print("EU Rhine-Alpine parity gap gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine parity gap gate: PASS")
    print("  checked parity decisions, blocked claims, and next required steps")
    return 0


if __name__ == "__main__":
    sys.exit(main())
