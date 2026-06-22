#!/usr/bin/env python3
"""Gate state payload role-review evaluation outputs."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
EVAL = ROOT / "data" / "state-payload-role-review-evaluation-001.csv"
SUMMARY = ROOT / "data" / "state-payload-role-review-summary-001.csv"
REVIEW = ROOT / "docs" / "reviews" / "state-payload-role-review-evaluation-001.md"

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


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> int:
    failures: list[str] = []
    rows = read_csv(EVAL)
    summary = read_csv(SUMMARY)
    review = REVIEW.read_text(encoding="utf-8")

    if len(rows) != 4:
        failures.append(f"expected four role-review rows, found {len(rows)}")
    if len(summary) != 1:
        failures.append("expected one role-review summary row")
    if any(row["fit_status"] != "fit_pass" for row in rows):
        failures.append("all sample candidate rows should fit_pass")
    if any(row["promotion_status"] != "promotion_hold" for row in rows):
        failures.append("all sample candidate rows must remain promotion_hold")
    if any(row["allowed_use"] != "internal_candidate_transform_and_client_workshop_prompt" for row in rows):
        failures.append("allowed use must stay internal/workshop only")
    for row in rows + summary:
        missing = REQUIRED_BLOCKS - set(row["held_claims"].split("|"))
        if missing:
            failures.append(f"row missing held claims: {sorted(missing)}")
    if summary:
        item = summary[0]
        if item["decision"] != "candidate_fit_passed_promotion_held":
            failures.append("summary decision must hold promotion")
        if item["candidate_rows"] != "4" or item["fit_pass_rows"] != "4" or item["promotion_hold_rows"] != "4":
            failures.append("summary counts do not match expected four held candidate rows")
    if "state_payload_role_review_passed_for_internal_candidate_only" not in review:
        failures.append("review missing role-review gate decision")
    if "does not validate client data" not in review:
        failures.append("review must preserve client-data validation boundary")
    if failures:
        print("State payload role review evaluation gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("State payload role review evaluation gate: PASS")
    print("  checked fit pass, promotion hold, allowed use, summary, and held claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
