#!/usr/bin/env python3
"""Gate Texas client-like payload pilot outputs."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
CANDIDATES = DATA / "state-texas-client-like-candidate-tierization-001.csv"
ROLE_REVIEW = DATA / "state-texas-client-like-role-review-001.csv"
CLOSEOUT = DATA / "state-texas-client-like-closeout-001.csv"
PREFLIGHT = DATA / "state-texas-client-like-preflight-001.csv"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-client-like-payload-pilot-001.md"

REQUIRED_ROLES = {"T1", "T2", "T3", "T4", "M"}
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
    candidates = read_csv(CANDIDATES)
    role_review = read_csv(ROLE_REVIEW)
    closeout = read_csv(CLOSEOUT)
    preflight = read_csv(PREFLIGHT)
    review = REVIEW.read_text(encoding="utf-8")
    roles = {row["candidate_role"] for row in candidates}
    if len(candidates) != 6:
        failures.append(f"expected six Texas candidate rows, found {len(candidates)}")
    if not REQUIRED_ROLES <= roles:
        failures.append(f"Texas role spread incomplete: {sorted(roles)}")
    if any(row["evidence_posture"] != "source-needed" for row in candidates):
        failures.append("all Texas candidate rows must remain source-needed")
    if any(row["promotion_status"] != "promotion_hold" for row in role_review):
        failures.append("all Texas role-review rows must remain promotion_hold")
    if not any(row["check_area"] == "source_custody" and row["status"] == "hold" for row in preflight):
        failures.append("Texas source custody must be held")
    if closeout[0]["decision"] != "texas_client_like_pipeline_passed_promotion_held":
        failures.append("Texas closeout decision mismatch")
    for row in candidates + role_review + closeout + preflight:
        field = "held_claims"
        missing = REQUIRED_BLOCKS - set(row[field].split("|"))
        if missing:
            failures.append(f"row missing held claims: {sorted(missing)}")
    if "texas_client_like_payload_pipeline_passed_promotion_held" not in review:
        failures.append("review missing Texas pilot gate decision")
    if "not a TxDOT plan" not in review:
        failures.append("review must hold TxDOT/official-plan boundary")
    if failures:
        print("Texas client-like payload pilot gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Texas client-like payload pilot gate: PASS")
    print("  checked role spread, source-needed posture, promotion hold, and held claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
