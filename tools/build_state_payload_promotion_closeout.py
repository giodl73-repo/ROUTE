#!/usr/bin/env python3
"""Build closeout summary for the generic state payload promotion pathway."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
REVIEW = ROOT / "docs" / "reviews" / "state-payload-promotion-closeout-001.md"

PREFLIGHT = DATA / "state-client-payload-preflight-evaluation-001.csv"
ROLE_SUMMARY = DATA / "state-payload-role-review-summary-001.csv"
OUTPUT = DATA / "state-payload-promotion-closeout-001.csv"
ACTION = DATA / "state-payload-promotion-next-actions-001.csv"

HELD = (
    "official_designation|legal_sla|construction|cost|numeric_roi|roi|"
    "eligibility|compliance|endorsement|validation|public_readiness|"
    "state_approval|source_backed_full_inventory"
)

CLOSEOUT_FIELDS = [
    "closeout_id",
    "chain_step",
    "artifact",
    "status",
    "promotion_effect",
    "remaining_blocker",
    "allowed_use",
    "held_claims",
]

ACTION_FIELDS = [
    "action_id",
    "recipient",
    "requested_input",
    "required_artifact",
    "unblocks",
    "claim_boundary",
    "held_claims",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def build_closeout() -> list[dict[str, str]]:
    preflight = read_csv(PREFLIGHT)
    role_summary = read_csv(ROLE_SUMMARY)[0]
    source_held = any(row["check_area"] == "source_custody" and row["evaluation_status"] == "hold" for row in preflight)
    promotion_held = role_summary["decision"] == "candidate_fit_passed_promotion_held"
    return [
        {
            "closeout_id": "CLOSE-001",
            "chain_step": "fit_kernel",
            "artifact": "data/state-tierization-fit-role-vector-profile-001.csv",
            "status": "operational",
            "promotion_effect": "supports internal candidate fitting",
            "remaining_blocker": "requires source inventory and client priority-node review",
            "allowed_use": "product demonstration and delivery planning",
            "held_claims": HELD,
        },
        {
            "closeout_id": "CLOSE-002",
            "chain_step": "source_adapter_contract",
            "artifact": "data/state-source-inventory-adapter-field-contract-001.csv",
            "status": "operational",
            "promotion_effect": "defines client source package requirements",
            "remaining_blocker": "client payload not supplied",
            "allowed_use": "client intake checklist",
            "held_claims": HELD,
        },
        {
            "closeout_id": "CLOSE-003",
            "chain_step": "payload_scaffold",
            "artifact": "data/state-client-payload-manifest-001.csv",
            "status": "operational",
            "promotion_effect": "provides fillable payload templates",
            "remaining_blocker": "templates contain sample rows only",
            "allowed_use": "client handoff packet",
            "held_claims": HELD,
        },
        {
            "closeout_id": "CLOSE-004",
            "chain_step": "payload_preflight",
            "artifact": "data/state-client-payload-preflight-evaluation-001.csv",
            "status": "held" if source_held else "operational",
            "promotion_effect": "checks template cross-reference integrity",
            "remaining_blocker": "source custody held" if source_held else "filled payload review required",
            "allowed_use": "internal QA and client readiness discussion",
            "held_claims": HELD,
        },
        {
            "closeout_id": "CLOSE-005",
            "chain_step": "candidate_tierization",
            "artifact": "data/state-payload-candidate-tierization-001.csv",
            "status": "operational",
            "promotion_effect": "emits source-needed candidate roles",
            "remaining_blocker": "candidate rows are sample-derived and require role review",
            "allowed_use": "internal candidate transform and workshop prompt",
            "held_claims": HELD,
        },
        {
            "closeout_id": "CLOSE-006",
            "chain_step": "role_review_evaluation",
            "artifact": "data/state-payload-role-review-summary-001.csv",
            "status": "held" if promotion_held else "operational",
            "promotion_effect": "fit passes internally but promotion remains held",
            "remaining_blocker": "real client payload and accepted source references required",
            "allowed_use": "delivery closeout and sales expectation setting",
            "held_claims": HELD,
        },
    ]


def build_actions() -> list[dict[str, str]]:
    return [
        {
            "action_id": "NEXT-001",
            "recipient": "client_state_or_operator",
            "requested_input": "filled segment inventory with stable source ids endpoints owner class and source references",
            "required_artifact": "state-client-payload-segment-template-001.csv",
            "unblocks": "source-backed candidate row generation",
            "claim_boundary": "does not create official tier or SLA claim",
            "held_claims": HELD,
        },
        {
            "action_id": "NEXT-002",
            "recipient": "client_state_or_operator",
            "requested_input": "priority nodes including cities gateways terminals rural service nodes and jurisdiction",
            "required_artifact": "state-client-payload-priority-node-template-001.csv",
            "unblocks": "client-specific T1 T3 and T4 fit review",
            "claim_boundary": "does not imply state approval or public readiness",
            "held_claims": HELD,
        },
        {
            "action_id": "NEXT-003",
            "recipient": "operations_or_planning_team",
            "requested_input": "restriction closure incident terminal-delay and failure metric source references",
            "required_artifact": "state-client-payload-restriction-failure-template-001.csv",
            "unblocks": "failure scorecard and resilience overlay review",
            "claim_boundary": "does not prove performance benefit or ROI",
            "held_claims": HELD,
        },
        {
            "action_id": "NEXT-004",
            "recipient": "delivery_team",
            "requested_input": "non-promotion reasons for inventory rows not assigned T1 T2 T3 or T4",
            "required_artifact": "state-client-payload-non-promotion-template-001.csv",
            "unblocks": "full-coverage audit without cherry-picking",
            "claim_boundary": "does not prove full source-backed inventory until every source segment is accounted for",
            "held_claims": HELD,
        },
    ]


def write_csv(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def write_review(closeout: list[dict[str, str]], actions: list[dict[str, str]]) -> None:
    held_count = sum(1 for row in closeout if row["status"] == "held")
    review = f"""---
name: State Payload Promotion Closeout 001
slug: state-payload-promotion-closeout-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-payload-promotion-closeout-001.csv
  - data/state-payload-promotion-next-actions-001.csv
  - data/state-payload-role-review-summary-001.csv
  - data/state-client-payload-preflight-evaluation-001.csv
---

# State Payload Promotion Closeout 001

## Scope

This closeout summarizes the generic state payload pathway from fit kernel to
role review. It is the delivery-facing status sheet for whether ROUTE can move
from sample payloads to a real client payload.

## Result

| Check | Result |
|---|---|
| Chain steps reviewed | {len(closeout)} |
| Held steps | {held_count} |
| Next action rows | {len(actions)} |
| Promotion decision | held until filled source-backed client payload |

## Use

The package is ready for a client intake conversation and internal product demo.
It is not ready for public claim promotion, official tier assignment, SLA
commitment, ROI claim, construction claim, or source-backed full-inventory claim.

## Gate

Decision: **state_payload_pathway_ready_for_client_intake_promotion_held**
"""
    REVIEW.write_text(review, encoding="utf-8")


def main() -> None:
    closeout = build_closeout()
    actions = build_actions()
    write_csv(OUTPUT, CLOSEOUT_FIELDS, closeout)
    write_csv(ACTION, ACTION_FIELDS, actions)
    write_review(closeout, actions)
    print(f"wrote {OUTPUT}")
    print(f"wrote {ACTION}")
    print(f"wrote {REVIEW}")


if __name__ == "__main__":
    main()
