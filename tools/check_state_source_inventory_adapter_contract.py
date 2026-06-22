#!/usr/bin/env python3
"""Gate generic state source-inventory adapter contract artifacts."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
FIELD_CONTRACT = DATA / "state-source-inventory-adapter-field-contract-001.csv"
ROW_CONTRACT = DATA / "state-source-inventory-adapter-row-contract-001.csv"
PRECHECK = DATA / "state-source-inventory-adapter-precheck-001.csv"
REVIEW = ROOT / "docs" / "reviews" / "state-source-inventory-adapter-contract-001.md"

REQUIRED_FIELDS = {
    "source_segment_id",
    "route_label",
    "from_ref",
    "to_ref",
    "owner_or_jurisdiction",
    "road_class",
    "priority_node_refs",
    "parallel_or_alternate_refs",
    "restriction_refs",
    "observed_failure_refs",
    "non_promotion_reason",
}
REQUIRED_SURFACES = {
    "state_road_inventory_segment",
    "priority_node_inventory",
    "terminal_access_inventory",
    "restriction_and_failure_inventory",
    "non_promotion_inventory",
}
REQUIRED_BLOCKS = {
    "official_designation",
    "legal_sla",
    "construction",
    "numeric_roi",
    "roi",
    "eligibility",
    "compliance",
    "endorsement",
    "validation",
    "public_readiness",
    "state_approval",
    "source_backed_full_inventory",
}
PROHIBITED_PROMOTION = {
    "guaranteed",
    "approved",
    "validated",
    "construction ready",
    "numeric roi",
    "source-backed full inventory",
}


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def blocked_missing(row: dict[str, str]) -> set[str]:
    return REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))


def main() -> int:
    failures: list[str] = []
    field_rows = read_csv(FIELD_CONTRACT)
    row_rows = read_csv(ROW_CONTRACT)
    precheck_rows = read_csv(PRECHECK)
    review = REVIEW.read_text(encoding="utf-8")

    fields = {row["field_name"] for row in field_rows}
    if fields != REQUIRED_FIELDS:
        failures.append(f"field contract mismatch: {sorted(fields)}")
    surfaces = {row["input_surface"] for row in row_rows}
    if surfaces != REQUIRED_SURFACES:
        failures.append(f"row surfaces mismatch: {sorted(surfaces)}")
    if len(precheck_rows) != 4:
        failures.append("precheck contract must contain four gates")
    for row in field_rows + row_rows + precheck_rows:
        missing = blocked_missing(row)
        if missing:
            failures.append(f"row missing blocked claims: {sorted(missing)}")
    if not any("SV-006" in row["required_for_vectors"] for row in field_rows if row["field_name"] == "non_promotion_reason"):
        failures.append("non_promotion_reason must be tied to SV-006")
    if not any("block_full_coverage_claim" in row["missing_field_behavior"] for row in field_rows):
        failures.append("adapter must block full-coverage claims when non-promotion reason is missing")
    if "state_source_inventory_adapter_contract_ready_for_client_payload" not in review:
        failures.append("review missing adapter contract gate decision")

    promoted_section = review.lower().split("## gate", 1)[0]
    for phrase in PROHIBITED_PROMOTION:
        if phrase in promoted_section and "held until" not in promoted_section:
            failures.append(f"review may promote prohibited phrase: {phrase}")

    if failures:
        print("State source inventory adapter contract gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("State source inventory adapter contract gate: PASS")
    print("  checked source fields, row surfaces, precheck gates, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
