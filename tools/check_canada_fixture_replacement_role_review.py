#!/usr/bin/env python3
"""Gate Canada fixture replacement role review results."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
REVIEW = ROOT / "data" / "international-canada-fixture-replacement-role-review-001.csv"
EXTRACTED_LINKS = ROOT / "data" / "international-canada-parser-extraction-candidates-001.csv"
DRY_RUN_LINKS = ROOT / "data" / "canada_source_link_candidates.csv"

FIELDS = [
    "review_id",
    "role_lane",
    "role_source",
    "input_compared",
    "decision",
    "finding",
    "required_next_step",
    "blocked_claims",
]

REQUIRED_ROLES = {
    "Scope Keeper",
    "Citation Auditor",
    "Schematic Cartographer",
    "Traffic Engineer",
    "State DOT Planner",
}

REQUIRED_BLOCKS = {
    "fixture_replacement",
    "source_row_validation",
    "geometry_acceptance",
    "parsed_adapter",
    "official_network",
    "route_designation",
    "agency_approval",
    "construction_ready",
    "guaranteed_sla",
    "roi",
    "eligibility",
    "compliance",
    "endorsement",
    "validation",
    "public_readiness",
    "external_readiness",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    fields, rows = read_csv(REVIEW)
    _, extracted = read_csv(EXTRACTED_LINKS)
    _, dry_run = read_csv(DRY_RUN_LINKS)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("role-review columns do not match required contract")
    if {row["role_lane"] for row in rows} != REQUIRED_ROLES:
        failures.append("role-review rows do not cover required .roles lanes")
    if not extracted or not dry_run:
        failures.append("comparison inputs are missing rows")

    for row in rows:
        decision = row["decision"]
        if decision not in {
            "pass_with_risk",
            "hold_for_map_or_fixture_use",
            "hold_for_operational_claims",
            "hold_for_authority_and_delivery_claims",
        }:
            failures.append(f"{row['review_id']} has unsupported decision {decision}")
        if "replace" in decision:
            failures.append(f"{row['review_id']} promotes replacement in decision")
        if not row["role_source"].startswith(".roles/"):
            failures.append(f"{row['review_id']} is not tied to a .roles source")
        blocked = set(row["blocked_claims"].split(";"))
        missing = REQUIRED_BLOCKS - blocked
        if missing:
            failures.append(f"{row['review_id']} missing blocked claims: {sorted(missing)}")
        if "fixture" not in row["required_next_step"] and row["role_lane"] == "Scope Keeper":
            failures.append("Scope Keeper next step does not preserve fixture hold")

    if failures:
        print("Canada fixture replacement role-review gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada fixture replacement role-review gate: PASS")
    print("  checked role coverage, comparison inputs, held replacement posture, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
