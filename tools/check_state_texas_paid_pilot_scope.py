#!/usr/bin/env python3
"""Gate Texas paid pilot scope outputs."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
PHASES = DATA / "state-texas-paid-pilot-phases-001.csv"
DELIVERABLES = DATA / "state-texas-paid-pilot-deliverables-001.csv"
ACCEPTANCE = DATA / "state-texas-paid-pilot-acceptance-001.csv"
NON_FIT = DATA / "state-texas-paid-pilot-non-fit-001.csv"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-paid-pilot-scope-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-paid-pilot-scope-001.md"

REQUIRED_PHASES = {
    "scope_lock",
    "source_intake",
    "custody_and_fit",
    "client_review",
    "executive_readout",
}
REQUIRED_DELIVERABLES = {
    "pilot_scope_sheet",
    "source_custody_ledger",
    "candidate_service_hierarchy",
    "failure_mode_scorecard",
    "investment_question_backlog",
    "executive_readout",
}
REQUIRED_GATES = {
    "scope_acceptance",
    "payload_acceptance",
    "custody_acceptance",
    "role_review_acceptance",
    "readout_acceptance",
}
REQUIRED_BLOCKS = {
    "official_designation",
    "legal_sla",
    "construction",
    "cost",
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
    phases = read_csv(PHASES)
    deliverables = read_csv(DELIVERABLES)
    acceptance = read_csv(ACCEPTANCE)
    non_fit = read_csv(NON_FIT)
    brief = BRIEF.read_text(encoding="utf-8")
    review = REVIEW.read_text(encoding="utf-8")

    phase_names = {row["phase"] for row in phases}
    if phase_names != REQUIRED_PHASES:
        failures.append(f"phase mismatch: {sorted(phase_names)}")
    deliverable_names = {row["deliverable"] for row in deliverables}
    if deliverable_names != REQUIRED_DELIVERABLES:
        failures.append(f"deliverable mismatch: {sorted(deliverable_names)}")
    gates = {row["gate"] for row in acceptance}
    if gates != REQUIRED_GATES:
        failures.append(f"acceptance gate mismatch: {sorted(gates)}")
    if len(non_fit) != 5:
        failures.append("expected five non-fit rows")
    if not all(row["pricing_posture"] == "commercial_scope_no_price_claim" for row in phases):
        failures.append("phase rows must hold price claims")
    if not all(row["decision"] == "do_not_start_paid_pilot_until_resolved" for row in non_fit):
        failures.append("non-fit rows must block paid pilot start")
    for row in phases + deliverables + acceptance + non_fit:
        missing = REQUIRED_BLOCKS - set(row["held_claims"].split("|"))
        if missing:
            failures.append(f"row missing held claims: {sorted(missing)}")
    if "not a price quote" not in brief:
        failures.append("brief must hold price quote boundary")
    if "not a price quote, procurement response, TxDOT plan" not in brief:
        failures.append("brief must hold commercial and TxDOT boundaries")
    if "texas_paid_pilot_scope_ready_for_buyer_review" not in review:
        failures.append("review missing paid pilot gate decision")

    if failures:
        print("Texas paid pilot scope gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Texas paid pilot scope gate: PASS")
    print("  checked phases, deliverables, acceptance gates, non-fit rows, and held claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
