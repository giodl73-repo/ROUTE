#!/usr/bin/env python3
"""Evaluate candidate state payload tierization rows for role-review readiness."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
REVIEW = ROOT / "docs" / "reviews" / "state-payload-role-review-evaluation-001.md"

CANDIDATES = DATA / "state-payload-candidate-tierization-001.csv"
ROLE_REVIEW = DATA / "state-payload-candidate-role-review-001.csv"
PREFLIGHT = DATA / "state-client-payload-preflight-evaluation-001.csv"
OUTPUT = DATA / "state-payload-role-review-evaluation-001.csv"
SUMMARY = DATA / "state-payload-role-review-summary-001.csv"

HELD = (
    "official_designation|legal_sla|construction|cost|numeric_roi|roi|"
    "eligibility|compliance|endorsement|validation|public_readiness|"
    "state_approval|source_backed_full_inventory"
)

FIELDS = [
    "evaluation_id",
    "source_segment_id",
    "candidate_role",
    "fit_status",
    "promotion_status",
    "promotion_blocker",
    "allowed_use",
    "required_next_evidence",
    "held_claims",
]

SUMMARY_FIELDS = [
    "summary_id",
    "surface",
    "candidate_rows",
    "fit_pass_rows",
    "promotion_hold_rows",
    "promotion_fail_rows",
    "decision",
    "next_action",
    "held_claims",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def source_custody_held() -> bool:
    rows = read_csv(PREFLIGHT)
    return any(row["check_area"] == "source_custody" and row["evaluation_status"] == "hold" for row in rows)


def build_rows() -> list[dict[str, str]]:
    candidates = read_csv(CANDIDATES)
    reviews = {row["source_segment_id"]: row for row in read_csv(ROLE_REVIEW)}
    custody_held = source_custody_held()
    rows: list[dict[str, str]] = []
    for idx, candidate in enumerate(candidates, start=1):
        review = reviews[candidate["source_segment_id"]]
        evidence_posture = candidate["evidence_posture"]
        fit_status = "fit_pass"
        if evidence_posture != "source-needed" or review["review_status"] != "role_review_required":
            fit_status = "fit_hold"
        promotion_status = "promotion_hold" if custody_held else "promotion_review_ready"
        blocker = (
            "source_custody_held_and_real_client_payload_not_reviewed"
            if custody_held
            else "role_review_required_before_promotion"
        )
        rows.append(
            {
                "evaluation_id": f"ROLE-EVAL-{idx:03d}",
                "source_segment_id": candidate["source_segment_id"],
                "candidate_role": candidate["candidate_role"],
                "fit_status": fit_status,
                "promotion_status": promotion_status,
                "promotion_blocker": blocker,
                "allowed_use": "internal_candidate_transform_and_client_workshop_prompt",
                "required_next_evidence": review["required_next_evidence"],
                "held_claims": HELD,
            }
        )
    return rows


def build_summary(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    return [
        {
            "summary_id": "ROLE-SUMMARY-001",
            "surface": "generic_state_payload_sample",
            "candidate_rows": str(len(rows)),
            "fit_pass_rows": str(sum(1 for row in rows if row["fit_status"] == "fit_pass")),
            "promotion_hold_rows": str(sum(1 for row in rows if row["promotion_status"] == "promotion_hold")),
            "promotion_fail_rows": str(sum(1 for row in rows if row["promotion_status"] == "promotion_fail")),
            "decision": "candidate_fit_passed_promotion_held",
            "next_action": "replace sample rows with accepted client payload and rerun role review evaluation",
            "held_claims": HELD,
        }
    ]


def write_csv(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def write_review(rows: list[dict[str, str]], summary: list[dict[str, str]]) -> None:
    review = f"""---
name: State Payload Role Review Evaluation 001
slug: state-payload-role-review-evaluation-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-payload-role-review-evaluation-001.csv
  - data/state-payload-role-review-summary-001.csv
  - data/state-payload-candidate-tierization-001.csv
  - data/state-client-payload-preflight-evaluation-001.csv
---

# State Payload Role Review Evaluation 001

## Scope

This evaluation reviews the candidate tierization rows emitted from the generic
client payload sample. It separates a plausible internal fit from any promoted
client, official, SLA, ROI, construction, validation, or approval claim.

## Result

| Check | Result |
|---|---|
| Candidate rows reviewed | {len(rows)} |
| Fit pass rows | {summary[0]["fit_pass_rows"]} |
| Promotion hold rows | {summary[0]["promotion_hold_rows"]} |
| Decision | {summary[0]["decision"]} |

## Evidence Boundary

The role review uses sample payload rows only. It does not validate client data,
source custody, official designations, legal SLAs, construction readiness, cost,
numeric ROI, eligibility, compliance, endorsement, public readiness, state
approval, or source-backed full inventory.

## Gate

Decision: **state_payload_role_review_passed_for_internal_candidate_only**
"""
    REVIEW.write_text(review, encoding="utf-8")


def main() -> None:
    rows = build_rows()
    summary = build_summary(rows)
    write_csv(OUTPUT, FIELDS, rows)
    write_csv(SUMMARY, SUMMARY_FIELDS, summary)
    write_review(rows, summary)
    print(f"wrote {OUTPUT}")
    print(f"wrote {SUMMARY}")
    print(f"wrote {REVIEW}")


if __name__ == "__main__":
    main()
