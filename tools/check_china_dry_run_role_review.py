#!/usr/bin/env python3
"""Gate China role review for parser dry-run outputs."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-china-dry-run-role-review-001.csv"

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
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("China dry-run role review columns do not match contract")
    if len(rows) != 5:
        failures.append("China dry-run role review must have five role rows")
    if {row["role_lane"] for row in rows} != REQUIRED_ROLES:
        failures.append("China dry-run role review missing required role lanes")
    if not any("standards context" in row["review_question"].lower() for row in rows):
        failures.append("China dry-run role review must preserve standards-context geometry review")
    for row in rows:
        if row["result"] != "pass_with_holds":
            failures.append(f"{row['review_id']} must pass only with holds")
        if row["allowed_use"] != "internal parser dry-run planning review only":
            failures.append(f"{row['review_id']} has unsupported allowed use")
        if "china_source_link_candidates" not in row["input_artifacts"]:
            failures.append(f"{row['review_id']} must cite China link dry-run input")
        if "china_adapter_evidence_labels" not in row["input_artifacts"]:
            failures.append(f"{row['review_id']} must cite China evidence labels input")
        if "before any fixture replacement contract" not in row["next_action"]:
            failures.append(f"{row['review_id']} must preserve fixture replacement dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['review_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("China dry-run role review gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("China dry-run role review gate: PASS")
    print("  checked role lanes, standards geometry review, pass-with-holds result, replacement dependency, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
