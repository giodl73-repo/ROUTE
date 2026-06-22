#!/usr/bin/env python3
"""Gate Texas source owner and custody docket outputs."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
OWNER_DOCKET = DATA / "state-texas-source-owner-docket-001.csv"
CUSTODY_CHECKLIST = DATA / "state-texas-source-custody-checklist-001.csv"
HANDOFF = DATA / "state-texas-source-handoff-decision-001.csv"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-source-owner-custody-docket-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-source-owner-custody-docket-001.md"

REQUIRED_SURFACES = {
    "segment_inventory",
    "priority_nodes",
    "failure_evidence",
    "terminal_access",
    "non_promotion",
    "claim_boundary",
}
REQUIRED_CUSTODY = {
    "source_identity",
    "row_traceability",
    "scope_label",
    "review_disposition",
    "claim_boundary",
}
REQUIRED_HANDOFF = {
    "owner_assignment_ready",
    "custody_review_ready",
    "role_review_ready",
    "sponsor_closeout_ready",
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
    owner_rows = read_csv(OWNER_DOCKET)
    custody_rows = read_csv(CUSTODY_CHECKLIST)
    handoff_rows = read_csv(HANDOFF)
    brief = BRIEF.read_text(encoding="utf-8")
    review = REVIEW.read_text(encoding="utf-8")

    surfaces = {row["source_surface"] for row in owner_rows}
    if surfaces != REQUIRED_SURFACES:
        failures.append(f"source surfaces mismatch: {sorted(surfaces)}")
    custody = {row["custody_check"] for row in custody_rows}
    if custody != REQUIRED_CUSTODY:
        failures.append(f"custody checks mismatch: {sorted(custody)}")
    handoffs = {row["decision_gate"] for row in handoff_rows}
    if handoffs != REQUIRED_HANDOFF:
        failures.append(f"handoff gates mismatch: {sorted(handoffs)}")
    for row in owner_rows + custody_rows + handoff_rows:
        missing = REQUIRED_BLOCKS - set(row["held_claims"].split("|"))
        if missing:
            failures.append(f"row missing held claims: {sorted(missing)}")
    if not all(row.get("initial_posture") == "source-needed" for row in owner_rows):
        failures.append("all owner rows must start source-needed")
    if not all(row.get("initial_decision") == "hold_awaiting_filled_texas_payload" for row in handoff_rows):
        failures.append("all handoff rows must start held awaiting filled payload")
    if "remains held until owner assignment" not in brief:
        failures.append("brief must keep Texas pilot held until owner/custody gates pass")
    if "not a TxDOT plan" not in brief:
        failures.append("brief must hold TxDOT plan boundary")
    if "texas_source_owner_custody_ready_awaiting_named_owners" not in review:
        failures.append("review missing source owner custody gate decision")

    if failures:
        print("Texas source owner custody docket gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Texas source owner custody docket gate: PASS")
    print("  checked owner surfaces, custody checks, handoff gates, and held claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
