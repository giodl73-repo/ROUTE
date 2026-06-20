#!/usr/bin/env python3
"""Gate India fixture replacement blocker."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BLOCKER = ROOT / "data" / "international-india-fixture-blocker-001.csv"

FIELDS = [
    "blocker_id",
    "replacement_target",
    "current_rows",
    "role_review_status",
    "geometry_status",
    "replacement_decision",
    "allowed_use",
    "blocked_claims",
    "required_next_step",
]
REQUIRED_BLOCKS = {
    "source_row_validation",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "guaranteed_sla",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
    "internal_adapter_proof",
}


def main() -> int:
    with BLOCKER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("India fixture blocker columns do not match contract")
    if len(rows) != 1:
        failures.append("India fixture blocker must have one blocker row")
    for row in rows:
        if row["role_review_status"] != "pass_with_holds":
            failures.append("India fixture blocker requires pass-with-holds role review")
        if row["geometry_status"] != "geometry_not_requested_policy_blocks_replacement":
            failures.append("India fixture blocker requires no-geometry policy hold")
        if row["replacement_decision"] != "blocked_source_rows_not_validated_geometry_not_accepted":
            failures.append("India fixture blocker must block replacement")
        if row["allowed_use"] != "gap tracking and source acquisition planning only":
            failures.append("India fixture blocker allowed use is too broad")
        if "before any fixture replacement contract" not in row["required_next_step"]:
            failures.append("India fixture blocker must preserve before-contract dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"India fixture blocker missing blocked claims: {sorted(missing)}")
    if failures:
        print("India fixture blocker gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("India fixture blocker gate: PASS")
    print("  checked role holds, no-geometry status, replacement block, and claim holds")
    return 0


if __name__ == "__main__":
    sys.exit(main())
