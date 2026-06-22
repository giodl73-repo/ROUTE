#!/usr/bin/env python3
"""Build Texas buyer review packet for paid pilot decision."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-buyer-review-packet-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-buyer-review-packet-001.md"

DECISION = DATA / "state-texas-buyer-review-decision-001.csv"
AGENDA = DATA / "state-texas-buyer-review-agenda-001.csv"
SOURCE_REQUEST = DATA / "state-texas-buyer-source-request-001.csv"
OBJECTIONS = DATA / "state-texas-buyer-objection-response-001.csv"

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


def decision_rows() -> list[dict[str, str]]:
    rows = [
        (
            "TX-BUYER-DECISION-001",
            "why_now",
            "Texas can test whether a full-state service-network hierarchy adds decision value beyond ordinary maps.",
            "Proceed only if a sponsor has a bounded pilot geography and source owners.",
        ),
        (
            "TX-BUYER-DECISION-002",
            "what_buyer_gets",
            "A scoped service hierarchy, source custody ledger, failure-mode scorecard, investment question backlog, and executive readout.",
            "Proceed only as an internal diagnostic package.",
        ),
        (
            "TX-BUYER-DECISION-003",
            "what_buyer_must_supply",
            "Segment inventory, priority nodes, failure evidence, terminal access, non-promotion reasons, and claim-boundary owner.",
            "Hold kickoff if source owners are not named.",
        ),
        (
            "TX-BUYER-DECISION-004",
            "what_is_not_included",
            "No price quote, procurement response, official plan, legal SLA, construction package, numeric ROI, endorsement, validation, public-readiness, or full inventory claim.",
            "Stop if buyer needs any held claim as the first deliverable.",
        ),
        (
            "TX-BUYER-DECISION-005",
            "go_no_go",
            "Start only when scope, source owner roster, source delivery path, role-review owner, and evidence boundary are accepted.",
            "Otherwise keep the relationship at workshop posture.",
        ),
    ]
    return [
        {
            "decision_id": decision_id,
            "decision_topic": topic,
            "buyer_language": language,
            "go_no_go_rule": rule,
            "recommended_posture": "buyer_review_only",
            "held_claims": HELD,
        }
        for decision_id, topic, language, rule in rows
    ]


def agenda_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-BUYER-AGENDA-001", "opening_boundary", "Confirm this is a diagnostic scope review, not a procurement or public plan.", "buyer accepts held claims", "hold meeting if buyer demands unsupported claims"),
        ("TX-BUYER-AGENDA-002", "service_priorities", "Ask for top Texas places, gateways, terminals, rural regions, and unacceptable failures.", "priority-node backlog", "hold source intake"),
        ("TX-BUYER-AGENDA-003", "source_owner_review", "Walk through segment, node, failure, terminal, non-promotion, and claim-boundary owners.", "source owner roster", "block paid pilot start"),
        ("TX-BUYER-AGENDA-004", "deliverable_review", "Review scope sheet, custody ledger, hierarchy, scorecard, backlog, and executive readout.", "deliverable acceptance", "hold scope acceptance"),
        ("TX-BUYER-AGENDA-005", "decision_close", "Ask whether buyer wants workshop only, paid pilot scoping, or hold.", "go no-go disposition", "record non-fit reason"),
    ]
    return [
        {
            "agenda_id": agenda_id,
            "agenda_item": item,
            "prompt": prompt,
            "expected_output": output,
            "hold_behavior": hold,
            "held_claims": HELD,
        }
        for agenda_id, item, prompt, output, hold in rows
    ]


def source_request_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-SOURCE-REQ-001", "segment_inventory", "Please identify the owner for the pilot-scope roadway segment inventory and the source format available.", "source_segment_id route_label from_ref to_ref owner_or_jurisdiction road_class source_ref"),
        ("TX-SOURCE-REQ-002", "priority_nodes", "Please provide the priority cities, gateways, ports, border crossings, energy regions, rural service nodes, emergency nodes, and terminals for the pilot.", "node_id node_label node_class jurisdiction source_ref"),
        ("TX-SOURCE-REQ-003", "failure_evidence", "Please identify available incident, closure, restriction, bottleneck, terminal-friction, evacuation, or recovery evidence.", "segment_ref failure_metric_ref source_ref review_status"),
        ("TX-SOURCE-REQ-004", "terminal_access", "Please identify port, airport, rail, energy, industrial, or logistics terminals included in the pilot and their access-route references.", "terminal_id terminal_class access_route_ref nearest_tier_node source_ref"),
        ("TX-SOURCE-REQ-005", "non_promotion", "Please identify pilot-scope segments that should remain maintenance-only or outside scope, with reasons.", "source_segment_id coverage_status non_promotion_reason review_owner"),
        ("TX-SOURCE-REQ-006", "claim_boundary", "Please name the owner who can approve diagnostic language and held-claim boundaries for internal use.", "held_claims evidence_posture approval_status public_use_status"),
    ]
    return [
        {
            "request_id": request_id,
            "source_surface": surface,
            "buyer_request": request,
            "required_fields": fields,
            "blocks_if_missing": "paid_pilot_start",
            "held_claims": HELD,
        }
        for request_id, surface, request, fields in rows
    ]


def objection_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-OBJ-001", "Is this a TxDOT plan?", "No. It is a diagnostic review packet until an authorized external review says otherwise."),
        ("TX-OBJ-002", "Can we use the resulting hierarchy publicly?", "Not from this packet. Public-readiness stays held until release review and claim approval occur."),
        ("TX-OBJ-003", "Does this guarantee a service level?", "No. The pilot can define candidate promises and evidence gaps; legal SLA claims remain held."),
        ("TX-OBJ-004", "Will this produce ROI?", "It can define the ROI evidence contract and investment questions; numeric ROI remains held."),
        ("TX-OBJ-005", "Is this a construction recommendation?", "No. It can structure next studies, pilots, and decision packages; construction readiness remains outside this scope."),
        ("TX-OBJ-006", "Can we start without source owners?", "No. Without source owners, keep the meeting at workshop posture and do not start the paid pilot."),
    ]
    return [
        {
            "objection_id": objection_id,
            "buyer_question": question,
            "safe_response": response,
            "decision_rule": "answer_without_promoting_held_claims",
            "held_claims": HELD,
        }
        for objection_id, question, response in rows
    ]


def write_docs() -> None:
    BRIEF.write_text(
        """---
name: Texas Buyer Review Packet 001
slug: state-texas-buyer-review-packet-001
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-texas-buyer-review-decision-001.csv
  - data/state-texas-buyer-review-agenda-001.csv
  - data/state-texas-buyer-source-request-001.csv
  - data/state-texas-buyer-objection-response-001.csv
  - data/state-texas-paid-pilot-phases-001.csv
---

# Texas Buyer Review Packet 001

## Purpose

Use this packet with a Texas buyer or sponsor to decide whether the paid pilot
scope is ready to start, should remain a workshop, or should be held.

## Close

The desired close is not adoption. The desired close is a go/no-go decision on
bounded pilot scoping, source owner assignment, and evidence-boundary acceptance.

## Boundary

This packet is not a price quote, procurement response, TxDOT plan, official
route designation, legal SLA, construction package, numeric ROI proof, state
approval, endorsement, validation, public-readiness packet, or source-backed full
inventory.
""",
        encoding="utf-8",
    )
    REVIEW.write_text(
        """---
name: Texas Buyer Review Packet 001
slug: state-texas-buyer-review-packet-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - docs/briefs/state-texas-buyer-review-packet-001.md
  - data/state-texas-buyer-review-decision-001.csv
  - data/state-texas-buyer-review-agenda-001.csv
  - data/state-texas-buyer-source-request-001.csv
  - data/state-texas-buyer-objection-response-001.csv
---

# Texas Buyer Review Packet 001

## Scope

This review confirms the Texas paid pilot has a buyer review packet with a
decision memo, agenda, source request, objection responses, and held claims.

## Gate

Decision: **texas_buyer_review_packet_ready_for_sponsor_conversation**
""",
        encoding="utf-8",
    )


def main() -> None:
    write_csv(DECISION, ["decision_id", "decision_topic", "buyer_language", "go_no_go_rule", "recommended_posture", "held_claims"], decision_rows())
    write_csv(AGENDA, ["agenda_id", "agenda_item", "prompt", "expected_output", "hold_behavior", "held_claims"], agenda_rows())
    write_csv(SOURCE_REQUEST, ["request_id", "source_surface", "buyer_request", "required_fields", "blocks_if_missing", "held_claims"], source_request_rows())
    write_csv(OBJECTIONS, ["objection_id", "buyer_question", "safe_response", "decision_rule", "held_claims"], objection_rows())
    write_docs()
    for path in [DECISION, AGENDA, SOURCE_REQUEST, OBJECTIONS, BRIEF, REVIEW]:
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
