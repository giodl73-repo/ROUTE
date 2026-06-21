#!/usr/bin/env python3
"""Gate China fixture replacement blocker."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BLOCKER = ROOT / "data" / "international-china-fixture-blocker-001.csv"

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
    "official_corridor_designation",
    "policy_alignment",
    "source_row_validation",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "terminal_performance",
    "road_access_proof",
    "throughput_proof",
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
        failures.append("China fixture blocker columns do not match contract")
    if len(rows) != 1:
        failures.append("China fixture blocker must have one blocker row")
    for row in rows:
        if row["role_review_status"] != "pass_with_holds":
            failures.append("China fixture blocker requires pass-with-holds role review")
        if not row["geometry_status"].startswith("geometry_not_accepted:"):
            failures.append("China fixture blocker requires geometry-not-accepted status")
        if "not_accepted:" not in row["geometry_status"]:
            failures.append("China fixture blocker must preserve not_accepted geometry references")
        if row["replacement_decision"] != "blocked_dry_run_rows_not_source_validated_geometry_not_accepted":
            failures.append("China fixture blocker must block replacement on dry-run/source/geometry grounds")
        if row["allowed_use"] != "gap tracking and source acquisition planning only":
            failures.append("China fixture blocker allowed use is too broad")
        if "context-only" not in row["current_rows"] or "heuristic-held" not in row["current_rows"]:
            failures.append("China fixture blocker must preserve context-only and heuristic-held row labels")
        if "before any fixture replacement contract" not in row["required_next_step"]:
            failures.append("China fixture blocker must preserve before-contract dependency")
        if "source-row validation" not in row["required_next_step"]:
            failures.append("China fixture blocker must require source-row validation")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"China fixture blocker missing blocked claims: {sorted(missing)}")
    if failures:
        print("China fixture blocker gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("China fixture blocker gate: PASS")
    print("  checked role holds, dry-run labels, no-geometry status, replacement block, and claim holds")
    return 0


if __name__ == "__main__":
    sys.exit(main())
