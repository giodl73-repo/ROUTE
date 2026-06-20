#!/usr/bin/env python3
"""Gate EU Rhine-Alpine port-node role review."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-port-node-role-review-001.csv"

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
    "fixture_replacement",
    "internal_adapter_proof",
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
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("EU port-node role review columns do not match contract")
    if len(rows) != 5:
        failures.append("EU port-node role review must have five role rows")
    if {row["role_lane"] for row in rows} != REQUIRED_ROLES:
        failures.append("EU port-node role review missing required role lanes")
    for row in rows:
        if row["result"] != "pass_with_holds":
            failures.append(f"{row['review_id']} must pass only with holds")
        if row["allowed_use"] != "internal node-candidate planning review only":
            failures.append(f"{row['review_id']} has unsupported allowed use")
        if "record-sample" not in row["input_artifacts"] and "record_sample" not in row["input_artifacts"]:
            failures.append(f"{row['review_id']} must cite record sample input")
        if "before any node fixture replacement contract" not in row["next_action"]:
            failures.append(f"{row['review_id']} must preserve replacement-contract dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['review_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine port-node role review gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine port-node role review gate: PASS")
    print("  checked role lanes, pass-with-holds result, replacement dependency, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
