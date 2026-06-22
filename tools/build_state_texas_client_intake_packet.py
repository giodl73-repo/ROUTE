#!/usr/bin/env python3
"""Build Texas-specific client intake packet from the client-like payload pilot."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-client-intake-packet-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-client-intake-packet-001.md"

PACKET = DATA / "state-texas-client-intake-packet-001.csv"
AGENDA = DATA / "state-texas-client-intake-workshop-agenda-001.csv"
CLAIMS = DATA / "state-texas-client-intake-held-claims-001.csv"
ASKS = DATA / "state-texas-client-intake-source-asks-001.csv"

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


def packet_rows() -> list[dict[str, str]]:
    fields = [
        ("TX-INTAKE-001", "Texas client-like pilot closeout", "data/state-texas-client-like-closeout-001.csv", "sponsor_or_delivery_lead", "Show that the Texas-shaped pipeline runs but promotion is held.", "Use as readiness status for the first Texas conversation."),
        ("TX-INTAKE-002", "Texas candidate tierization", "data/state-texas-client-like-candidate-tierization-001.csv", "planning_or_operations_lead", "Review candidate T1 T2 T3 T4 M rows before replacing samples.", "Mark which rows match Texas priorities and which are missing."),
        ("TX-INTAKE-003", "Texas priority-node payload", "data/state-texas-client-like-priority-node-payload-001.csv", "client_state_or_operator", "Seed the priority-place discussion around metros gateways ports border and rural references.", "Replace sample nodes with client-approved priority nodes."),
        ("TX-INTAKE-004", "Texas failure payload", "data/state-texas-client-like-failure-payload-001.csv", "operations_or_resilience_team", "Seed the failure-mode discussion around reliability redundancy access terminal friction and non-promotion.", "Attach accepted incident closure restriction or terminal evidence."),
        ("TX-INTAKE-005", "Texas service-network offer", "docs/briefs/texas-state-service-network-offer.md", "executive_sponsor", "Frame the 90-day diagnostic offer and buyer value.", "Confirm whether the first paid wedge is diagnostic intake role design or source-backed pilot."),
    ]
    return [
        {
            "packet_item_id": item_id,
            "item_name": name,
            "artifact": artifact,
            "recipient": recipient,
            "purpose": purpose,
            "client_action": action,
            "ready_status": "ready_for_texas_intake",
            "held_claims": HELD,
        }
        for item_id, name, artifact, recipient, purpose, action in fields
    ]


def agenda_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-AGENDA-001", "service_promise", "Which Texas megaregions gateways and production regions must stay connected first?", "Texas offer and priority-node payload", "ranked priority-node backlog", "client cannot name priority places"),
        ("TX-AGENDA-002", "role_fit", "Do the sample T1 T2 T3 T4 M roles match how Texas would describe statewide service?", "Texas candidate tierization", "role edits and missing-role list", "sample rows are treated as official"),
        ("TX-AGENDA-003", "failure_modes", "Which hurricane flood heat winter wildfire border bridge work-zone incident or evacuation failures are unacceptable?", "Texas failure payload", "failure evidence request list", "failure cannot be tied to source evidence"),
        ("TX-AGENDA-004", "source_payload", "Which Texas source inventories can be supplied for segments nodes terminals restrictions and non-promotion reasons?", "generic client payload templates", "filled-payload owner and delivery date", "no source owner is available"),
    ]
    return [
        {
            "agenda_id": agenda_id,
            "workshop_step": step,
            "question": question,
            "input_artifact": artifact,
            "expected_output": output,
            "stop_condition": stop,
            "held_claims": HELD,
        }
        for agenda_id, step, question, artifact, output, stop in rows
    ]


def claim_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-CLAIM-001", "ROUTE has a Texas state network plan", "ROUTE has a Texas-shaped intake pilot and source-ready workflow.", "No Texas client source payload or approval is present.", "accepted client payload plus explicit approval record"),
        ("TX-CLAIM-002", "These are TxDOT-approved tiers", "These are candidate roles for workshop review.", "The pilot derives from internal bounded sample rows.", "role review with client-designated owner"),
        ("TX-CLAIM-003", "Texas receives SLA or ROI guarantees", "ROUTE identifies candidate service roles and evidence gaps.", "No legal SLA numeric ROI or performance validation has been supplied.", "separate validated SLA ROI evidence package"),
        ("TX-CLAIM-004", "Texas full inventory is complete", "The pathway can ingest a full inventory once supplied.", "The pilot has six sample rows not a full source inventory.", "all source segments accounted for with role or non-promotion reason"),
    ]
    return [
        {
            "claim_id": claim_id,
            "claim_phrase_to_avoid": avoid,
            "safe_phrase": safe,
            "reason": reason,
            "unlock_condition": unlock,
            "held_claims": HELD,
        }
        for claim_id, avoid, safe, reason, unlock in rows
    ]


def ask_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-ASK-001", "segment_inventory", "Texas roadway segment source with stable ids endpoints owner jurisdiction and class", "state-texas-client-like-segment-payload-001.csv", "source-backed candidate tierization", "source-needed"),
        ("TX-ASK-002", "priority_nodes", "Texas metros ports border gateways energy regions rural service nodes terminals and emergency nodes", "state-texas-client-like-priority-node-payload-001.csv", "Texas-specific service promise design", "source-needed"),
        ("TX-ASK-003", "failure_evidence", "Incident closure bottleneck restriction terminal friction evacuation or recovery evidence by segment", "state-texas-client-like-failure-payload-001.csv", "failure scorecard and resilience overlay review", "source-needed"),
        ("TX-ASK-004", "non_promotion", "Rows that should remain maintained-only or outside scope with reasons", "generic non-promotion payload template", "full-coverage audit without cherry-picking", "source-needed"),
    ]
    return [
        {
            "ask_id": ask_id,
            "source_surface": surface,
            "requested_input": requested,
            "seed_artifact": artifact,
            "unblocks": unblocks,
            "initial_posture": posture,
            "held_claims": HELD,
        }
        for ask_id, surface, requested, artifact, unblocks, posture in rows
    ]


def write_brief() -> None:
    brief = """---
name: Texas Client Intake Packet 001
slug: state-texas-client-intake-packet-001
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-texas-client-intake-packet-001.csv
  - data/state-texas-client-intake-workshop-agenda-001.csv
  - data/state-texas-client-intake-source-asks-001.csv
  - data/state-texas-client-like-closeout-001.csv
---

# Texas Client Intake Packet 001

## Use

Use this packet to run the first Texas service-network intake conversation. It
connects the Texas offer to a concrete payload request, workshop agenda, and
held-claims language.

## Meeting Outcome

The meeting should produce a Texas source payload owner, a priority-node backlog,
a failure-evidence request list, and a decision on whether to run the first
source-backed Texas pilot.

## Boundary

This packet is not a TxDOT plan, official tier assignment, legal SLA, ROI claim,
construction package, endorsement, validation, public-readiness packet, or
source-backed full inventory.
"""
    BRIEF.write_text(brief, encoding="utf-8")


def write_review() -> None:
    review = """---
name: Texas Client Intake Packet 001
slug: state-texas-client-intake-packet-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - docs/briefs/state-texas-client-intake-packet-001.md
  - data/state-texas-client-intake-packet-001.csv
  - data/state-texas-client-intake-workshop-agenda-001.csv
  - data/state-texas-client-intake-held-claims-001.csv
  - data/state-texas-client-intake-source-asks-001.csv
---

# Texas Client Intake Packet 001

## Scope

This review confirms the Texas client-like pilot has a Texas-specific intake
packet, workshop agenda, source asks, and held-claim language.

## Gate

Decision: **texas_client_intake_packet_ready_promotion_held**
"""
    REVIEW.write_text(review, encoding="utf-8")


def main() -> None:
    write_csv(PACKET, ["packet_item_id", "item_name", "artifact", "recipient", "purpose", "client_action", "ready_status", "held_claims"], packet_rows())
    write_csv(AGENDA, ["agenda_id", "workshop_step", "question", "input_artifact", "expected_output", "stop_condition", "held_claims"], agenda_rows())
    write_csv(CLAIMS, ["claim_id", "claim_phrase_to_avoid", "safe_phrase", "reason", "unlock_condition", "held_claims"], claim_rows())
    write_csv(ASKS, ["ask_id", "source_surface", "requested_input", "seed_artifact", "unblocks", "initial_posture", "held_claims"], ask_rows())
    write_brief()
    write_review()
    for path in [PACKET, AGENDA, CLAIMS, ASKS, BRIEF, REVIEW]:
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
