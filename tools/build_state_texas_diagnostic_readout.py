#!/usr/bin/env python3
"""Build Texas diagnostic readout from client-like pilot and intake packet."""

from __future__ import annotations

import csv
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-diagnostic-readout-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-diagnostic-readout-001.md"

CANDIDATES = DATA / "state-texas-client-like-candidate-tierization-001.csv"
ASKS = DATA / "state-texas-client-intake-source-asks-001.csv"
CLOSEOUT = DATA / "state-texas-client-like-closeout-001.csv"
READOUT = DATA / "state-texas-diagnostic-readout-001.csv"
SCORECARD = DATA / "state-texas-diagnostic-scorecard-001.csv"

HELD = (
    "official_designation|legal_sla|construction|cost|numeric_roi|roi|"
    "eligibility|compliance|endorsement|validation|public_readiness|"
    "state_approval|source_backed_full_inventory"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def write_csv(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def build_readout() -> list[dict[str, str]]:
    candidates = read_csv(CANDIDATES)
    asks = read_csv(ASKS)
    closeout = read_csv(CLOSEOUT)[0]
    roles = Counter(row["candidate_role"] for row in candidates)
    overlay_count = sum(1 for row in candidates if row["overlay_roles"])
    return [
        {
            "readout_id": "TX-READOUT-001",
            "topic": "pipeline_result",
            "status": closeout["decision"],
            "finding": f"{closeout['candidate_rows']} Texas-shaped candidate rows ran through the pipeline with promotion held.",
            "client_question": "Should Texas proceed to a filled source-payload pilot?",
            "next_action": closeout["next_action"],
            "held_claims": HELD,
        },
        {
            "readout_id": "TX-READOUT-002",
            "topic": "role_spread",
            "status": "pass",
            "finding": "Candidate roles include " + ";".join(f"{role}:{roles[role]}" for role in sorted(roles)),
            "client_question": "Which Texas roles are missing or misclassified from the first sample?",
            "next_action": "Use workshop to edit T1 T2 T3 T4 M assumptions before source-backed run.",
            "held_claims": HELD,
        },
        {
            "readout_id": "TX-READOUT-003",
            "topic": "resilience_overlay",
            "status": "pass",
            "finding": f"{overlay_count} rows include resilience overlays or review hooks.",
            "client_question": "Which failures would Texas treat as unacceptable service failures?",
            "next_action": "Map hurricane flood heat winter wildfire border bridge work-zone and incident evidence to SSF metrics.",
            "held_claims": HELD,
        },
        {
            "readout_id": "TX-READOUT-004",
            "topic": "source_asks",
            "status": "ready_for_intake",
            "finding": f"{len(asks)} source ask rows define the payload needed for the first real Texas run.",
            "client_question": "Who owns segment inventory priority nodes failure evidence and non-promotion reasons?",
            "next_action": "Assign Texas source owners and delivery dates.",
            "held_claims": HELD,
        },
        {
            "readout_id": "TX-READOUT-005",
            "topic": "promotion_boundary",
            "status": "held",
            "finding": "All official tier SLA ROI construction approval validation and full-inventory claims remain held.",
            "client_question": "Which claims does the sponsor want to unlock first and with what evidence?",
            "next_action": "Create source-backed pilot run after payload receipt.",
            "held_claims": HELD,
        },
    ]


def build_scorecard() -> list[dict[str, str]]:
    candidates = read_csv(CANDIDATES)
    rows: list[dict[str, str]] = []
    for idx, row in enumerate(candidates, start=1):
        rows.append(
            {
                "score_id": f"TX-SCORE-{idx:03d}",
                "candidate_id": row["source_segment_id"],
                "role": row["candidate_role"],
                "pipeline_fit": "pass",
                "source_status": "held",
                "promotion_status": "held",
                "primary_gap": "filled source payload and client role review required",
                "held_claims": HELD,
            }
        )
    return rows


def write_docs(readout: list[dict[str, str]], scorecard: list[dict[str, str]]) -> None:
    brief = f"""---
name: Texas Diagnostic Readout 001
slug: state-texas-diagnostic-readout-001
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-texas-diagnostic-readout-001.csv
  - data/state-texas-diagnostic-scorecard-001.csv
  - data/state-texas-client-intake-source-asks-001.csv
---

# Texas Diagnostic Readout 001

## Result

The Texas client-like pilot is ready for sponsor review and intake. It produced
{len(scorecard)} candidate role rows and a concrete source-ask list. Promotion is
held until Texas supplies a filled source payload and role review is rerun.

## Sponsor Decision

Decide whether to run a first source-backed Texas pilot using the intake packet.

## Boundary

This readout is not a TxDOT plan, official tier assignment, legal SLA, ROI claim,
construction package, endorsement, validation, public-readiness packet, or
source-backed full inventory.
"""
    BRIEF.write_text(brief, encoding="utf-8")
    review = """---
name: Texas Diagnostic Readout 001
slug: state-texas-diagnostic-readout-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - docs/briefs/state-texas-diagnostic-readout-001.md
  - data/state-texas-diagnostic-readout-001.csv
  - data/state-texas-diagnostic-scorecard-001.csv
---

# Texas Diagnostic Readout 001

## Scope

This review confirms the Texas client-like pilot has a sponsor-facing diagnostic
readout and candidate scorecard.

## Gate

Decision: **texas_diagnostic_readout_ready_source_backed_pilot_next**
"""
    REVIEW.write_text(review, encoding="utf-8")


def main() -> None:
    readout = build_readout()
    scorecard = build_scorecard()
    write_csv(READOUT, ["readout_id", "topic", "status", "finding", "client_question", "next_action", "held_claims"], readout)
    write_csv(SCORECARD, ["score_id", "candidate_id", "role", "pipeline_fit", "source_status", "promotion_status", "primary_gap", "held_claims"], scorecard)
    write_docs(readout, scorecard)
    for path in [READOUT, SCORECARD, BRIEF, REVIEW]:
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
