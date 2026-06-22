#!/usr/bin/env python3
"""Build Texas paid pilot scope package."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-paid-pilot-scope-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-paid-pilot-scope-001.md"

PHASES = DATA / "state-texas-paid-pilot-phases-001.csv"
DELIVERABLES = DATA / "state-texas-paid-pilot-deliverables-001.csv"
ACCEPTANCE = DATA / "state-texas-paid-pilot-acceptance-001.csv"
NON_FIT = DATA / "state-texas-paid-pilot-non-fit-001.csv"

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


def phase_rows() -> list[dict[str, str]]:
    rows = [
        (
            "TX-SCOPE-PHASE-001",
            "scope_lock",
            "Confirm Texas pilot geography, priority nodes, source owners, and claims that must stay held.",
            "state-texas-source-owner-docket-001.csv",
            "signed pilot scope and owner roster",
            "block paid pilot start",
        ),
        (
            "TX-SCOPE-PHASE-002",
            "source_intake",
            "Receive filled segment, priority-node, failure, terminal, and non-promotion payloads.",
            "state client payload templates",
            "payload receipt ledger and source custody queue",
            "hold source-backed fit",
        ),
        (
            "TX-SCOPE-PHASE-003",
            "custody_and_fit",
            "Check source identity, row traceability, pilot scope labels, and run candidate tierization.",
            "state-texas-source-custody-checklist-001.csv",
            "candidate service hierarchy with evidence posture",
            "hold role review",
        ),
        (
            "TX-SCOPE-PHASE-004",
            "client_review",
            "Review T1 T2 T3 T4 R M X rows with client owners and record pass hold fail decisions.",
            "state-texas-source-handoff-decision-001.csv",
            "role review ledger and revised priority backlog",
            "hold promotion",
        ),
        (
            "TX-SCOPE-PHASE-005",
            "executive_readout",
            "Package service-network findings, failure modes, evidence gaps, and next investment questions.",
            "role review and closeout outputs",
            "internal executive readout and next-pilot recommendation",
            "hold public claims",
        ),
    ]
    return [
        {
            "phase_id": phase_id,
            "phase": phase,
            "work": work,
            "primary_input": primary_input,
            "client_output": output,
            "hold_behavior": hold_behavior,
            "pricing_posture": "commercial_scope_no_price_claim",
            "held_claims": HELD,
        }
        for phase_id, phase, work, primary_input, output, hold_behavior in rows
    ]


def deliverable_rows() -> list[dict[str, str]]:
    rows = [
        (
            "TX-DELIV-001",
            "pilot_scope_sheet",
            "Bounded Texas geography, owner roster, source surfaces, and claim boundary.",
            "buyer knows exactly what is in and out of the pilot",
            "not a statewide plan or procurement document",
        ),
        (
            "TX-DELIV-002",
            "source_custody_ledger",
            "Source identity, row traceability, review disposition, and unresolved gaps.",
            "buyer sees which rows can support analysis and which remain held",
            "not a source-backed full inventory",
        ),
        (
            "TX-DELIV-003",
            "candidate_service_hierarchy",
            "Candidate T1 T2 T3 T4 R M X assignments for the pilot scope.",
            "buyer sees roles and non-promotion reasons, not just a route map",
            "not an official tier designation",
        ),
        (
            "TX-DELIV-004",
            "failure_mode_scorecard",
            "Closure, restriction, bottleneck, terminal, recovery, and access gaps tied to rows where evidence exists.",
            "buyer sees why the current network fails a promise",
            "not a guaranteed SLA or engineering finding",
        ),
        (
            "TX-DELIV-005",
            "investment_question_backlog",
            "Questions grouped by service role, failure mode, source gap, and next owner decision.",
            "buyer gets a sequence for studies, pilots, and decision packages",
            "not a construction or funding commitment",
        ),
        (
            "TX-DELIV-006",
            "executive_readout",
            "Leadership-ready summary with evidence boundary and next-pilot recommendation.",
            "buyer can brief leadership without overstating the result",
            "not public-readiness, endorsement, approval, ROI proof, or validation",
        ),
    ]
    return [
        {
            "deliverable_id": deliverable_id,
            "deliverable": deliverable,
            "contents": contents,
            "client_value": value,
            "boundary": boundary,
            "held_claims": HELD,
        }
        for deliverable_id, deliverable, contents, value, boundary in rows
    ]


def acceptance_rows() -> list[dict[str, str]]:
    rows = [
        (
            "TX-PAID-ACCEPT-001",
            "scope_acceptance",
            "client confirms pilot scope, source owner roles, and held claims",
            "continue",
            "hold kickoff",
        ),
        (
            "TX-PAID-ACCEPT-002",
            "payload_acceptance",
            "required payloads are supplied or explicitly marked source-needed",
            "continue to custody",
            "hold fit",
        ),
        (
            "TX-PAID-ACCEPT-003",
            "custody_acceptance",
            "source refs resolve to source metadata or reviewed gaps",
            "continue to candidate hierarchy",
            "hold source-backed posture",
        ),
        (
            "TX-PAID-ACCEPT-004",
            "role_review_acceptance",
            "client owners mark each candidate role pass hold or fail",
            "continue to readout",
            "hold promotion",
        ),
        (
            "TX-PAID-ACCEPT-005",
            "readout_acceptance",
            "executive readout preserves evidence posture and next evidence steps",
            "close pilot internally",
            "hold public claims",
        ),
    ]
    return [
        {
            "acceptance_id": acceptance_id,
            "gate": gate,
            "pass_condition": condition,
            "pass_behavior": pass_behavior,
            "hold_behavior": hold_behavior,
            "held_claims": HELD,
        }
        for acceptance_id, gate, condition, pass_behavior, hold_behavior in rows
    ]


def non_fit_rows() -> list[dict[str, str]]:
    rows = [
        (
            "TX-NONFIT-001",
            "buyer asks for guaranteed SLA",
            "ROUTE can define planning promises and evidence holds, but not legal SLA guarantees in this pilot.",
        ),
        (
            "TX-NONFIT-002",
            "buyer asks for ROI number before evidence",
            "ROUTE can build the ROI evidence contract, but numeric ROI remains held until source-backed costs and benefits exist.",
        ),
        (
            "TX-NONFIT-003",
            "buyer asks for construction-ready recommendation",
            "ROUTE can stage investment questions, but construction readiness needs engineering, environmental, funding, and agency gates.",
        ),
        (
            "TX-NONFIT-004",
            "buyer will not supply source owners",
            "Keep the work at workshop/demo posture because source-backed fit cannot run without source custody.",
        ),
        (
            "TX-NONFIT-005",
            "buyer wants public validation or endorsement",
            "Do not claim validation, endorsement, approval, or public-readiness without explicit external review records.",
        ),
    ]
    return [
        {
            "non_fit_id": non_fit_id,
            "condition": condition,
            "response": response,
            "decision": "do_not_start_paid_pilot_until_resolved",
            "held_claims": HELD,
        }
        for non_fit_id, condition, response in rows
    ]


def write_docs() -> None:
    BRIEF.write_text(
        """---
name: Texas Paid Pilot Scope 001
slug: state-texas-paid-pilot-scope-001
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-texas-paid-pilot-phases-001.csv
  - data/state-texas-paid-pilot-deliverables-001.csv
  - data/state-texas-paid-pilot-acceptance-001.csv
  - data/state-texas-paid-pilot-non-fit-001.csv
  - data/state-texas-source-owner-docket-001.csv
---

# Texas Paid Pilot Scope 001

## Offer

Run a bounded Texas service-network diagnostic that turns client source payloads
into a candidate service hierarchy, failure-mode scorecard, evidence boundary,
and executive readout.

## Buyer Commitment

The buyer must name source owners, supply or explicitly hold required payloads,
review candidate roles, and accept evidence boundaries before the pilot begins.

## Commercial Boundary

This is a scope package, not a price quote, procurement response, TxDOT plan,
official route designation, legal SLA, construction package, ROI proof, state
approval, endorsement, validation, public-readiness packet, or source-backed
full inventory.
""",
        encoding="utf-8",
    )
    REVIEW.write_text(
        """---
name: Texas Paid Pilot Scope 001
slug: state-texas-paid-pilot-scope-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - docs/briefs/state-texas-paid-pilot-scope-001.md
  - data/state-texas-paid-pilot-phases-001.csv
  - data/state-texas-paid-pilot-deliverables-001.csv
  - data/state-texas-paid-pilot-acceptance-001.csv
  - data/state-texas-paid-pilot-non-fit-001.csv
---

# Texas Paid Pilot Scope 001

## Scope

This review confirms the Texas package has a buyer-facing paid pilot scope with
phases, deliverables, acceptance gates, non-fit responses, and held claims.

## Gate

Decision: **texas_paid_pilot_scope_ready_for_buyer_review**
""",
        encoding="utf-8",
    )


def main() -> None:
    write_csv(PHASES, ["phase_id", "phase", "work", "primary_input", "client_output", "hold_behavior", "pricing_posture", "held_claims"], phase_rows())
    write_csv(DELIVERABLES, ["deliverable_id", "deliverable", "contents", "client_value", "boundary", "held_claims"], deliverable_rows())
    write_csv(ACCEPTANCE, ["acceptance_id", "gate", "pass_condition", "pass_behavior", "hold_behavior", "held_claims"], acceptance_rows())
    write_csv(NON_FIT, ["non_fit_id", "condition", "response", "decision", "held_claims"], non_fit_rows())
    write_docs()
    for path in [PHASES, DELIVERABLES, ACCEPTANCE, NON_FIT, BRIEF, REVIEW]:
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
