#!/usr/bin/env python3
"""Gate candidate tierization rows emitted from state client payload sample."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TIER = ROOT / "data" / "state-payload-candidate-tierization-001.csv"
ROLE_REVIEW = ROOT / "data" / "state-payload-candidate-role-review-001.csv"
REVIEW = ROOT / "docs" / "reviews" / "state-payload-candidate-tierization-001.md"

REQUIRED_ROLES = {"T1", "T2", "T4", "M"}
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
    tier_rows = read_csv(TIER)
    review_rows = read_csv(ROLE_REVIEW)
    review = REVIEW.read_text(encoding="utf-8")

    if len(tier_rows) != 4:
        failures.append(f"expected four candidate tier rows, found {len(tier_rows)}")
    if len(review_rows) != len(tier_rows):
        failures.append("role review row count must match tier rows")
    roles = {row["candidate_role"] for row in tier_rows}
    if roles != REQUIRED_ROLES:
        failures.append(f"candidate roles mismatch: {sorted(roles)}")
    if any(row["evidence_posture"] != "source-needed" for row in tier_rows):
        failures.append("all candidate tier rows must remain source-needed")
    if any(row["review_status"] != "role_review_required" for row in review_rows):
        failures.append("all candidate roles must require role review")
    if not any(row["overlay_roles"] == "R" for row in tier_rows):
        failures.append("candidate tierization must include a resilience overlay")
    for row in tier_rows + review_rows:
        missing = REQUIRED_BLOCKS - set(row["held_claims"].split("|"))
        if missing:
            failures.append(f"{row.get('source_segment_id', row.get('review_id'))} missing held claims: {sorted(missing)}")
    if "state_payload_candidate_tierization_ready_for_filled_payload_role_review" not in review:
        failures.append("review missing candidate tierization gate decision")
    if "does not validate" not in review:
        failures.append("review must preserve validation boundary")
    if failures:
        print("State payload candidate tierization gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("State payload candidate tierization gate: PASS")
    print("  checked candidate roles, source-needed posture, role review, and held claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
