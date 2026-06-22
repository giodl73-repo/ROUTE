#!/usr/bin/env python3
"""Gate Texas source-backed pilot plan outputs."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
PLAN = DATA / "state-texas-source-backed-pilot-plan-001.csv"
ACCEPTANCE = DATA / "state-texas-source-backed-pilot-acceptance-001.csv"
BLOCKERS = DATA / "state-texas-source-backed-pilot-blockers-001.csv"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-source-backed-pilot-plan-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-source-backed-pilot-plan-001.md"

REQUIRED_STEPS = {
    "payload_owner_assignment",
    "payload_receipt",
    "source_custody_review",
    "candidate_generation",
    "role_review",
    "sponsor_closeout",
}
REQUIRED_SURFACES = {
    "segment_inventory",
    "priority_nodes",
    "failure_evidence",
    "terminal_access",
    "non_promotion",
    "claim_boundary",
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


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> int:
    failures: list[str] = []
    plan = read_csv(PLAN)
    acceptance = read_csv(ACCEPTANCE)
    blockers = read_csv(BLOCKERS)
    brief = BRIEF.read_text(encoding="utf-8")
    review = REVIEW.read_text(encoding="utf-8")

    steps = {row["pilot_step"] for row in plan}
    if steps != REQUIRED_STEPS:
        failures.append(f"pilot steps mismatch: {sorted(steps)}")
    surfaces = {row["surface"] for row in acceptance}
    if surfaces != REQUIRED_SURFACES:
        failures.append(f"acceptance surfaces mismatch: {sorted(surfaces)}")
    if len(blockers) != 5:
        failures.append("expected five blocker rows")
    for row in plan + acceptance + blockers:
        missing = REQUIRED_BLOCKS - set(row["held_claims"].split("|"))
        if missing:
            failures.append(f"row missing held claims: {sorted(missing)}")
    if "Run only after Texas source owners are named" not in brief:
        failures.append("brief must require source owners before pilot run")
    if "not a TxDOT plan" not in brief:
        failures.append("brief must hold TxDOT plan boundary")
    if "texas_source_backed_pilot_plan_ready_awaiting_payload" not in review:
        failures.append("review missing pilot plan gate decision")
    if failures:
        print("Texas source-backed pilot plan gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Texas source-backed pilot plan gate: PASS")
    print("  checked pilot steps, acceptance surfaces, blockers, and held claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
