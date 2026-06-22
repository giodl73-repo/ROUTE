#!/usr/bin/env python3
"""Build client-facing state intake packet from the payload promotion closeout."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "state-client-intake-packet-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-client-intake-packet-001.md"

PACKET = DATA / "state-client-intake-packet-001.csv"
AGENDA = DATA / "state-client-intake-workshop-agenda-001.csv"
CLAIMS = DATA / "state-client-intake-held-claims-001.csv"

HELD = (
    "official_designation|legal_sla|construction|cost|numeric_roi|roi|"
    "eligibility|compliance|endorsement|validation|public_readiness|"
    "state_approval|source_backed_full_inventory"
)

PACKET_FIELDS = [
    "packet_item_id",
    "item_name",
    "artifact",
    "recipient",
    "purpose",
    "client_action",
    "ready_status",
    "held_claims",
]

AGENDA_FIELDS = [
    "agenda_id",
    "workshop_step",
    "question",
    "input_artifact",
    "expected_output",
    "stop_condition",
    "held_claims",
]

CLAIM_FIELDS = [
    "claim_id",
    "claim_phrase_to_avoid",
    "safe_phrase",
    "reason",
    "unlock_condition",
    "held_claims",
]


def write_csv(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def packet_rows() -> list[dict[str, str]]:
    return [
        {
            "packet_item_id": "INTAKE-001",
            "item_name": "Segment inventory template",
            "artifact": "data/state-client-payload-segment-template-001.csv",
            "recipient": "client_state_or_operator",
            "purpose": "Collect stable road segment ids endpoints ownership class and source references.",
            "client_action": "Replace sample rows with source inventory rows.",
            "ready_status": "ready_for_client_fill",
            "held_claims": HELD,
        },
        {
            "packet_item_id": "INTAKE-002",
            "item_name": "Priority node template",
            "artifact": "data/state-client-payload-priority-node-template-001.csv",
            "recipient": "client_state_or_operator",
            "purpose": "Collect cities gateways terminals rural service nodes and jurisdiction.",
            "client_action": "Confirm node classes and add missing client priority nodes.",
            "ready_status": "ready_for_client_fill",
            "held_claims": HELD,
        },
        {
            "packet_item_id": "INTAKE-003",
            "item_name": "Restriction and failure template",
            "artifact": "data/state-client-payload-restriction-failure-template-001.csv",
            "recipient": "operations_or_planning_team",
            "purpose": "Collect closure incident bottleneck terminal-delay and failure metric references.",
            "client_action": "Attach source references or mark source-needed gaps.",
            "ready_status": "ready_for_client_fill",
            "held_claims": HELD,
        },
        {
            "packet_item_id": "INTAKE-004",
            "item_name": "Terminal access template",
            "artifact": "data/state-client-payload-terminal-access-template-001.csv",
            "recipient": "terminal_or_freight_lead",
            "purpose": "Collect port airport border freight district and local access rows.",
            "client_action": "Add terminal access routes and constraints.",
            "ready_status": "ready_for_client_fill",
            "held_claims": HELD,
        },
        {
            "packet_item_id": "INTAKE-005",
            "item_name": "Non-promotion template",
            "artifact": "data/state-client-payload-non-promotion-template-001.csv",
            "recipient": "delivery_team",
            "purpose": "Preserve full inventory coverage without promoting every row.",
            "client_action": "Record why unpromoted rows remain M or X.",
            "ready_status": "ready_for_client_fill",
            "held_claims": HELD,
        },
        {
            "packet_item_id": "INTAKE-006",
            "item_name": "Promotion closeout",
            "artifact": "data/state-payload-promotion-closeout-001.csv",
            "recipient": "sponsor_or_delivery_lead",
            "purpose": "Show current readiness and held promotion boundary.",
            "client_action": "Use as expectation-setting status sheet.",
            "ready_status": "ready_for_intake_conversation",
            "held_claims": HELD,
        },
    ]


def agenda_rows() -> list[dict[str, str]]:
    return [
        {
            "agenda_id": "AGENDA-001",
            "workshop_step": "network_promise_definition",
            "question": "Which statewide city or gateway pairs must the network serve first?",
            "input_artifact": "state-client-payload-priority-node-template-001.csv",
            "expected_output": "client priority nodes and first-pass T1/T2 discussion",
            "stop_condition": "client cannot identify priority nodes or source owner",
            "held_claims": HELD,
        },
        {
            "agenda_id": "AGENDA-002",
            "workshop_step": "inventory_mapping",
            "question": "Which source inventory should ROUTE treat as the system of record for road segments?",
            "input_artifact": "state-client-payload-segment-template-001.csv",
            "expected_output": "accepted segment source and column mapping backlog",
            "stop_condition": "no stable segment id or endpoint reference exists",
            "held_claims": HELD,
        },
        {
            "agenda_id": "AGENDA-003",
            "workshop_step": "failure_metric_mapping",
            "question": "Which restrictions failures or terminal frictions make current service promises underperform?",
            "input_artifact": "state-client-payload-restriction-failure-template-001.csv",
            "expected_output": "SSF metric mapping and source-needed evidence list",
            "stop_condition": "failure evidence cannot be tied to a segment or metric",
            "held_claims": HELD,
        },
        {
            "agenda_id": "AGENDA-004",
            "workshop_step": "coverage_audit",
            "question": "Which inventory rows should stay maintained only or outside scope?",
            "input_artifact": "state-client-payload-non-promotion-template-001.csv",
            "expected_output": "M/X coverage plan and non-promotion reasons",
            "stop_condition": "client wants only cherry-picked promoted corridors",
            "held_claims": HELD,
        },
    ]


def claim_rows() -> list[dict[str, str]]:
    return [
        {
            "claim_id": "CLAIM-001",
            "claim_phrase_to_avoid": "ROUTE has tierized the state system",
            "safe_phrase": "ROUTE has a source-ready workflow for tierizing a supplied state inventory.",
            "reason": "No filled source-backed client inventory has been reviewed.",
            "unlock_condition": "accepted source payload and role review closeout",
            "held_claims": HELD,
        },
        {
            "claim_id": "CLAIM-002",
            "claim_phrase_to_avoid": "These are state-approved T1/T2/T3/T4 routes",
            "safe_phrase": "These are candidate roles pending client and source review.",
            "reason": "Ownership and approval are not inferred from sample rows.",
            "unlock_condition": "client approval or explicit review record",
            "held_claims": HELD,
        },
        {
            "claim_id": "CLAIM-003",
            "claim_phrase_to_avoid": "ROUTE guarantees SLA or ROI improvement",
            "safe_phrase": "ROUTE identifies candidate service roles and evidence gaps.",
            "reason": "No legal SLA numeric ROI or performance validation is established.",
            "unlock_condition": "separate validated SLA ROI or performance evidence package",
            "held_claims": HELD,
        },
    ]


def write_brief() -> None:
    brief = """---
name: State Client Intake Packet 001
slug: state-client-intake-packet-001
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-client-intake-packet-001.csv
  - data/state-client-intake-workshop-agenda-001.csv
  - data/state-client-intake-held-claims-001.csv
  - data/state-payload-promotion-closeout-001.csv
---

# State Client Intake Packet 001

## Use

Use this packet to start a state, authority, consultant, or private-operator
intake conversation. It provides fillable source templates, a workshop agenda,
and safe language for explaining what ROUTE can do before source review.

## Boundary

The packet is ready for intake and internal demonstration. It is not a public
readiness packet and does not claim official tiers, legal SLAs, construction
readiness, numeric ROI, endorsement, state approval, validation, or a
source-backed full inventory.
"""
    BRIEF.write_text(brief, encoding="utf-8")


def write_review() -> None:
    review = """---
name: State Client Intake Packet 001
slug: state-client-intake-packet-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - docs/briefs/state-client-intake-packet-001.md
  - data/state-client-intake-packet-001.csv
  - data/state-client-intake-workshop-agenda-001.csv
  - data/state-client-intake-held-claims-001.csv
---

# State Client Intake Packet 001

## Scope

This review confirms that the state payload pathway has a client-facing intake
packet with templates, workshop questions, and held-claim language.

## Gate

Decision: **state_client_intake_packet_ready_promotion_held**
"""
    REVIEW.write_text(review, encoding="utf-8")


def main() -> None:
    write_csv(PACKET, PACKET_FIELDS, packet_rows())
    write_csv(AGENDA, AGENDA_FIELDS, agenda_rows())
    write_csv(CLAIMS, CLAIM_FIELDS, claim_rows())
    write_brief()
    write_review()
    for path in [PACKET, AGENDA, CLAIMS, BRIEF, REVIEW]:
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
