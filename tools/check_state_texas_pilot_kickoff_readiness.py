#!/usr/bin/env python3
"""Gate Texas paid pilot kickoff readiness outputs."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
CHECKLIST = DATA / "state-texas-pilot-kickoff-checklist-001.csv"
RISKS = DATA / "state-texas-pilot-kickoff-risk-register-001.csv"
ARTIFACTS = DATA / "state-texas-pilot-kickoff-artifact-register-001.csv"
EXIT = DATA / "state-texas-pilot-kickoff-exit-criteria-001.csv"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-pilot-kickoff-readiness-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-pilot-kickoff-readiness-001.md"

REQUIRED_CHECKS = {
    "sponsor_confirmed",
    "scope_confirmed",
    "source_owners_named",
    "data_handling_confirmed",
    "review_cadence_confirmed",
    "claim_boundary_confirmed",
}
REQUIRED_RISKS = {
    "source_owner_delay",
    "source_format_mismatch",
    "scope_creep",
    "claim_pressure",
    "role_review_absence",
}
REQUIRED_ARTIFACTS = {
    "pilot_scope_sheet",
    "source_owner_roster",
    "payload_receipt_log",
    "custody_review_log",
    "role_review_log",
    "executive_readout_closeout",
}
REQUIRED_EXITS = {
    "start_paid_pilot",
    "workshop_only",
    "hold_for_procurement",
    "hold_for_claim_pressure",
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
    checklist = read_csv(CHECKLIST)
    risks = read_csv(RISKS)
    artifacts = read_csv(ARTIFACTS)
    exits = read_csv(EXIT)
    brief = BRIEF.read_text(encoding="utf-8")
    review = REVIEW.read_text(encoding="utf-8")

    checks = {row["kickoff_check"] for row in checklist}
    if checks != REQUIRED_CHECKS:
        failures.append(f"kickoff checks mismatch: {sorted(checks)}")
    risk_names = {row["risk"] for row in risks}
    if risk_names != REQUIRED_RISKS:
        failures.append(f"risks mismatch: {sorted(risk_names)}")
    artifact_names = {row["artifact"] for row in artifacts}
    if artifact_names != REQUIRED_ARTIFACTS:
        failures.append(f"artifacts mismatch: {sorted(artifact_names)}")
    exit_paths = {row["exit_path"] for row in exits}
    if exit_paths != REQUIRED_EXITS:
        failures.append(f"exit paths mismatch: {sorted(exit_paths)}")
    if not all(row["initial_status"] == "not_started" for row in checklist):
        failures.append("all kickoff checks must start not_started")
    if not all(row["severity"] == "pilot_blocker" for row in risks):
        failures.append("all kickoff risks must be pilot blockers")
    if not all(row["artifact_posture"] == "internal_pilot_artifact" for row in artifacts):
        failures.append("all artifacts must stay internal pilot posture")
    for row in checklist + risks + artifacts + exits:
        missing = REQUIRED_BLOCKS - set(row["held_claims"].split("|"))
        if missing:
            failures.append(f"row missing held claims: {sorted(missing)}")
    if "Do not start the paid pilot until sponsor, scope, source owners" not in brief:
        failures.append("brief must include start rule")
    if "not a price quote, procurement response, TxDOT plan" not in brief:
        failures.append("brief must hold commercial and TxDOT boundaries")
    if "texas_pilot_kickoff_readiness_ready_awaiting_sponsor_scope_sources" not in review:
        failures.append("review missing kickoff readiness gate decision")

    if failures:
        print("Texas pilot kickoff readiness gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Texas pilot kickoff readiness gate: PASS")
    print("  checked kickoff checks, risks, artifact register, exit paths, and held claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
