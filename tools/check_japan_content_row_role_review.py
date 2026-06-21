#!/usr/bin/env python3
"""Gate Japan role review for content-row validation outputs."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-japan-content-row-role-review-001.csv"

FIELDS = [
    "review_id",
    "role_lane",
    "review_question",
    "input_artifacts",
    "result",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_ROLES = {
    "Scope Keeper",
    "Citation Auditor",
    "Schematic Cartographer",
    "Traffic Engineer",
    "V&V",
}
REQUIRED_BLOCKS = {
    "source_row_validation",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "terminal_performance",
    "node_completeness",
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
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("Japan content-row role review columns do not match contract")
    if len(rows) != 5:
        failures.append("Japan content-row role review must have five role rows")
    if {row["role_lane"] for row in rows} != REQUIRED_ROLES:
        failures.append("Japan content-row role review missing required role lanes")
    if not any("GSI" in row["review_question"] for row in rows):
        failures.append("Japan content-row role review must preserve GSI blocker review")
    for row in rows:
        if row["result"] != "pass_with_holds":
            failures.append(f"{row['review_id']} must pass only with holds")
        if row["allowed_use"] != "internal content-row planning review only":
            failures.append(f"{row['review_id']} has unsupported allowed use")
        if "source-content-row-validation" not in row["input_artifacts"]:
            failures.append(f"{row['review_id']} must cite content-row validation input")
        if "before any fixture replacement contract" not in row["next_action"]:
            failures.append(f"{row['review_id']} must preserve fixture replacement dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['review_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("Japan content-row role review gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Japan content-row role review gate: PASS")
    print("  checked role lanes, GSI blocker review, pass-with-holds result, replacement dependency, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
