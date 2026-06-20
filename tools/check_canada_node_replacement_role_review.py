#!/usr/bin/env python3
"""Gate Canada node fixture replacement role review."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
REVIEW = ROOT / "data" / "international-canada-node-replacement-role-review-001.csv"

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
    "terminal_performance",
    "node_completeness",
    "road_access_proof",
    "throughput_proof",
    "guaranteed_sla",
    "roi",
    "validation",
    "public_readiness",
    "external_readiness",
}


def main() -> int:
    with REVIEW.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("node role-review columns do not match required contract")
    missing_roles = REQUIRED_ROLES - {row["role_lane"] for row in rows}
    if missing_roles:
        failures.append(f"node role review missing roles: {sorted(missing_roles)}")
    for row in rows:
        if row["result"] != "pass_with_holds":
            failures.append(f"{row['review_id']} did not pass with holds")
        if row["allowed_use"] != "internal node fixture replacement review only":
            failures.append(f"{row['review_id']} allowed use is too broad")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"{row['review_id']} missing blocked claims: {sorted(missing_blocks)}")

    if failures:
        print("Canada node replacement role-review gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada node replacement role-review gate: PASS")
    print("  checked role coverage, pass-with-holds status, and claim blocks")
    return 0


if __name__ == "__main__":
    sys.exit(main())
