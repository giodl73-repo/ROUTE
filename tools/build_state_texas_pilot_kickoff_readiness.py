#!/usr/bin/env python3
"""Build Texas paid pilot kickoff readiness package."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-pilot-kickoff-readiness-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-pilot-kickoff-readiness-001.md"

CHECKLIST = DATA / "state-texas-pilot-kickoff-checklist-001.csv"
RISKS = DATA / "state-texas-pilot-kickoff-risk-register-001.csv"
ARTIFACTS = DATA / "state-texas-pilot-kickoff-artifact-register-001.csv"
EXIT = DATA / "state-texas-pilot-kickoff-exit-criteria-001.csv"

HELD = (
    "official_designation|legal_sla|construction|cost|numeric_roi|roi|"
    "eligibility|compliance|endorsement|validation|public_readiness|"
    "state_approval|source_backed_full_inventory"
)


def write_csv(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def checklist_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-KICKOFF-001", "sponsor_confirmed", "named sponsor and decision owner", "kickoff allowed", "hold kickoff"),
        ("TX-KICKOFF-002", "scope_confirmed", "pilot geography, included source systems, excluded claims, and deliverables", "open source intake", "hold paid pilot start"),
        ("TX-KICKOFF-003", "source_owners_named", "owners for segment inventory, priority nodes, failure evidence, terminal access, non-promotion, and claim boundary", "open payload receipt", "hold source-backed fit"),
        ("TX-KICKOFF-004", "data_handling_confirmed", "permitted source formats, custody metadata, cache rules, and non-public handling expectations", "open custody review", "hold source custody"),
        ("TX-KICKOFF-005", "review_cadence_confirmed", "working sessions, role review session, executive readout date, and decision checkpoints", "open delivery calendar", "hold delivery start"),
        ("TX-KICKOFF-006", "claim_boundary_confirmed", "buyer accepts official, SLA, construction, ROI, approval, validation, public-readiness, and full-inventory holds", "open internal diagnostic posture", "hold all external claims"),
    ]
    return [
        {
            "check_id": check_id,
            "kickoff_check": check,
            "required_evidence": evidence,
            "pass_behavior": pass_behavior,
            "hold_behavior": hold_behavior,
            "initial_status": "not_started",
            "held_claims": HELD,
        }
        for check_id, check, evidence, pass_behavior, hold_behavior in rows
    ]


def risk_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-KRISK-001", "source_owner_delay", "named source owner unavailable or cannot deliver payload", "keep workshop posture and reschedule source intake"),
        ("TX-KRISK-002", "source_format_mismatch", "available source format cannot fill required adapter fields", "create mapping gap and hold source-backed fit"),
        ("TX-KRISK-003", "scope_creep", "buyer expands pilot geography or asks for full Texas inventory midstream", "split into next-phase backlog and hold full-inventory claim"),
        ("TX-KRISK-004", "claim_pressure", "buyer asks to use output as public validation, official plan, SLA, ROI, or construction support", "apply objection response and hold public claims"),
        ("TX-KRISK-005", "role_review_absence", "client owner cannot review candidate T1 T2 T3 T4 R M X roles", "hold promotion and close with unresolved role-review gap"),
    ]
    return [
        {
            "risk_id": risk_id,
            "risk": risk,
            "trigger": trigger,
            "required_response": response,
            "severity": "pilot_blocker",
            "held_claims": HELD,
        }
        for risk_id, risk, trigger, response in rows
    ]


def artifact_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-KART-001", "pilot_scope_sheet", "pilot scope, source systems, deliverables, holds", "required_before_kickoff"),
        ("TX-KART-002", "source_owner_roster", "named owners, roles, contact path, delivery date", "required_before_source_intake"),
        ("TX-KART-003", "payload_receipt_log", "payload names, row counts, missing fields, source-needed flags", "required_before_custody_review"),
        ("TX-KART-004", "custody_review_log", "source metadata, traceability, review disposition, gaps", "required_before_candidate_generation"),
        ("TX-KART-005", "role_review_log", "candidate row, client disposition, next evidence, held claims", "required_before_executive_readout"),
        ("TX-KART-006", "executive_readout_closeout", "internal findings, unresolved holds, next pilot or stop decision", "required_for_closeout"),
    ]
    return [
        {
            "artifact_id": artifact_id,
            "artifact": artifact,
            "contents": contents,
            "required_timing": timing,
            "artifact_posture": "internal_pilot_artifact",
            "held_claims": HELD,
        }
        for artifact_id, artifact, contents, timing in rows
    ]


def exit_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-KEXIT-001", "start_paid_pilot", "all kickoff checks pass", "begin source intake", "do not start"),
        ("TX-KEXIT-002", "workshop_only", "sponsor interested but source owners or scope are missing", "run workshop and source-owner follow-up", "do not call it paid pilot kickoff"),
        ("TX-KEXIT-003", "hold_for_procurement", "buyer needs procurement, price quote, or contracting artifacts", "handoff to commercial process", "do not imply procurement readiness"),
        ("TX-KEXIT-004", "hold_for_claim_pressure", "buyer requires official, SLA, ROI, construction, validation, approval, or public claim", "record non-fit and held-claim response", "do not proceed"),
    ]
    return [
        {
            "exit_id": exit_id,
            "exit_path": path,
            "condition": condition,
            "next_action": action,
            "stop_rule": stop,
            "held_claims": HELD,
        }
        for exit_id, path, condition, action, stop in rows
    ]


def write_docs() -> None:
    BRIEF.write_text(
        """---
name: Texas Pilot Kickoff Readiness 001
slug: state-texas-pilot-kickoff-readiness-001
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-texas-pilot-kickoff-checklist-001.csv
  - data/state-texas-pilot-kickoff-risk-register-001.csv
  - data/state-texas-pilot-kickoff-artifact-register-001.csv
  - data/state-texas-pilot-kickoff-exit-criteria-001.csv
  - data/state-texas-buyer-review-decision-001.csv
---

# Texas Pilot Kickoff Readiness 001

## Purpose

Use this package after a Texas buyer review produces interest in starting the
paid pilot. It confirms whether kickoff is ready, should remain workshop-only,
or must be held for procurement, source, or claim-boundary reasons.

## Start Rule

Do not start the paid pilot until sponsor, scope, source owners, data handling,
review cadence, and claim boundary are all confirmed.

## Boundary

This package is not a price quote, procurement response, TxDOT plan, official
route designation, legal SLA, construction package, numeric ROI proof, state
approval, endorsement, validation, public-readiness packet, or source-backed full
inventory.
""",
        encoding="utf-8",
    )
    REVIEW.write_text(
        """---
name: Texas Pilot Kickoff Readiness 001
slug: state-texas-pilot-kickoff-readiness-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - docs/briefs/state-texas-pilot-kickoff-readiness-001.md
  - data/state-texas-pilot-kickoff-checklist-001.csv
  - data/state-texas-pilot-kickoff-risk-register-001.csv
  - data/state-texas-pilot-kickoff-artifact-register-001.csv
  - data/state-texas-pilot-kickoff-exit-criteria-001.csv
---

# Texas Pilot Kickoff Readiness 001

## Scope

This review confirms the Texas buyer review now has a kickoff readiness package
with start checks, risks, artifact register, exit paths, and held claims.

## Gate

Decision: **texas_pilot_kickoff_readiness_ready_awaiting_sponsor_scope_sources**
""",
        encoding="utf-8",
    )


def main() -> None:
    write_csv(CHECKLIST, ["check_id", "kickoff_check", "required_evidence", "pass_behavior", "hold_behavior", "initial_status", "held_claims"], checklist_rows())
    write_csv(RISKS, ["risk_id", "risk", "trigger", "required_response", "severity", "held_claims"], risk_rows())
    write_csv(ARTIFACTS, ["artifact_id", "artifact", "contents", "required_timing", "artifact_posture", "held_claims"], artifact_rows())
    write_csv(EXIT, ["exit_id", "exit_path", "condition", "next_action", "stop_rule", "held_claims"], exit_rows())
    write_docs()
    for path in [CHECKLIST, RISKS, ARTIFACTS, EXIT, BRIEF, REVIEW]:
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
