#!/usr/bin/env python3
"""Build Texas source owner and custody docket for the source-backed pilot."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-source-owner-custody-docket-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-source-owner-custody-docket-001.md"

OWNER_DOCKET = DATA / "state-texas-source-owner-docket-001.csv"
CUSTODY_CHECKLIST = DATA / "state-texas-source-custody-checklist-001.csv"
HANDOFF = DATA / "state-texas-source-handoff-decision-001.csv"

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


def owner_rows() -> list[dict[str, str]]:
    rows = [
        (
            "TX-OWNER-001",
            "segment_inventory",
            "state road inventory owner",
            "source_segment_id route_label from_ref to_ref owner_or_jurisdiction road_class source_ref",
            "state-texas-client-like-segment-payload-001.csv",
            "stable segment ids and endpoints for the pilot scope",
            "no candidate row can become source-backed",
        ),
        (
            "TX-OWNER-002",
            "priority_nodes",
            "statewide planning or freight priority owner",
            "node_id node_label node_class jurisdiction source_ref",
            "state-texas-client-like-priority-node-payload-001.csv",
            "priority node list for metros ports border gateways energy regions rural nodes terminals and emergency nodes",
            "role assignment remains workshop-only",
        ),
        (
            "TX-OWNER-003",
            "failure_evidence",
            "operations safety resilience or freight performance owner",
            "segment_ref failure_metric_ref source_ref review_status",
            "state-texas-client-like-failure-payload-001.csv",
            "closure restriction bottleneck terminal friction evacuation or recovery evidence by segment",
            "resilience scorecard and failure claims remain held",
        ),
        (
            "TX-OWNER-004",
            "terminal_access",
            "port airport rail energy or logistics access owner",
            "terminal_id terminal_class access_route_ref nearest_tier_node source_ref",
            "state client terminal access template",
            "terminal access records tied to route and node references",
            "T4 terminal claims remain source-needed",
        ),
        (
            "TX-OWNER-005",
            "non_promotion",
            "state system coverage and maintenance owner",
            "source_segment_id coverage_status non_promotion_reason review_owner",
            "state client non-promotion template",
            "explicit M X or out-of-scope reasons for unpromoted rows",
            "full-coverage claim remains blocked",
        ),
        (
            "TX-OWNER-006",
            "claim_boundary",
            "sponsor or legal communications owner",
            "held_claims evidence_posture approval_status public_use_status",
            "state-texas-source-backed-pilot-plan-001.csv",
            "signed boundary that keeps official SLA ROI construction approval validation and public claims held",
            "packet cannot leave internal pilot posture",
        ),
    ]
    return [
        {
            "owner_id": owner_id,
            "source_surface": surface,
            "required_owner_role": owner_role,
            "required_fields": fields,
            "seed_artifact": seed,
            "pilot_unblock": unblock,
            "missing_owner_consequence": consequence,
            "initial_posture": "source-needed",
            "held_claims": HELD,
        }
        for owner_id, surface, owner_role, fields, seed, unblock, consequence in rows
    ]


def custody_rows() -> list[dict[str, str]]:
    rows = [
        (
            "TX-CUSTODY-001",
            "source_identity",
            "source title source owner capture date source url or cached artifact",
            "every source_ref resolves to reviewable source metadata",
            "mark row source-needed",
        ),
        (
            "TX-CUSTODY-002",
            "row_traceability",
            "source row id payload row id candidate row id",
            "each candidate row traces back to exactly one or more payload rows",
            "hold candidate generation",
        ),
        (
            "TX-CUSTODY-003",
            "scope_label",
            "pilot scope geography source system included rows excluded rows",
            "pilot is explicitly scoped and cannot imply full Texas inventory",
            "block source-backed full-inventory claim",
        ),
        (
            "TX-CUSTODY-004",
            "review_disposition",
            "review_owner review_date pass hold fail reason",
            "every row has a custody disposition before role review",
            "hold role review",
        ),
        (
            "TX-CUSTODY-005",
            "claim_boundary",
            "held_claims evidence_posture public_use_status approval_status",
            "all outputs preserve unsupported claim holds",
            "block sponsor closeout",
        ),
    ]
    return [
        {
            "custody_id": custody_id,
            "custody_check": check,
            "required_metadata": metadata,
            "pass_condition": pass_condition,
            "failure_behavior": failure_behavior,
            "held_claims": HELD,
        }
        for custody_id, check, metadata, pass_condition, failure_behavior in rows
    ]


def handoff_rows() -> list[dict[str, str]]:
    rows = [
        (
            "TX-HANDOFF-001",
            "owner_assignment_ready",
            "all source surfaces have named owner role and delivery date",
            "run payload receipt",
            "block pilot run",
        ),
        (
            "TX-HANDOFF-002",
            "custody_review_ready",
            "all source refs resolve to metadata or explicit source-needed disposition",
            "run candidate generation",
            "hold source-backed fit",
        ),
        (
            "TX-HANDOFF-003",
            "role_review_ready",
            "candidate rows have traceable source rows and held claims",
            "schedule Texas owner role review",
            "hold promotion",
        ),
        (
            "TX-HANDOFF-004",
            "sponsor_closeout_ready",
            "role review decisions preserve pass hold fail and next evidence",
            "write internal closeout",
            "hold public claims",
        ),
    ]
    return [
        {
            "handoff_id": handoff_id,
            "decision_gate": gate,
            "required_condition": condition,
            "pass_next_step": pass_step,
            "hold_behavior": hold_behavior,
            "initial_decision": "hold_awaiting_filled_texas_payload",
            "held_claims": HELD,
        }
        for handoff_id, gate, condition, pass_step, hold_behavior in rows
    ]


def write_docs() -> None:
    BRIEF.write_text(
        """---
name: Texas Source Owner Custody Docket 001
slug: state-texas-source-owner-custody-docket-001
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-texas-source-owner-docket-001.csv
  - data/state-texas-source-custody-checklist-001.csv
  - data/state-texas-source-handoff-decision-001.csv
  - data/state-texas-source-backed-pilot-plan-001.csv
---

# Texas Source Owner Custody Docket 001

## Use

Use this docket before running the Texas source-backed pilot. It names the owner
roles, source packets, custody checks, and handoff gates needed to turn the
Texas client-like pilot into a filled-source pilot.

## Decision

The Texas pilot remains held until owner assignment, source custody, and handoff
conditions pass.

## Boundary

This docket is not a TxDOT plan, official route designation, legal SLA,
construction package, ROI proof, state approval, endorsement, validation,
public-readiness packet, or source-backed full inventory.
""",
        encoding="utf-8",
    )
    REVIEW.write_text(
        """---
name: Texas Source Owner Custody Docket 001
slug: state-texas-source-owner-custody-docket-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - docs/briefs/state-texas-source-owner-custody-docket-001.md
  - data/state-texas-source-owner-docket-001.csv
  - data/state-texas-source-custody-checklist-001.csv
  - data/state-texas-source-handoff-decision-001.csv
---

# Texas Source Owner Custody Docket 001

## Scope

This review confirms the Texas source-backed pilot now has a pre-run owner and
source custody docket.

## Gate

Decision: **texas_source_owner_custody_ready_awaiting_named_owners**
""",
        encoding="utf-8",
    )


def main() -> None:
    write_csv(
        OWNER_DOCKET,
        [
            "owner_id",
            "source_surface",
            "required_owner_role",
            "required_fields",
            "seed_artifact",
            "pilot_unblock",
            "missing_owner_consequence",
            "initial_posture",
            "held_claims",
        ],
        owner_rows(),
    )
    write_csv(
        CUSTODY_CHECKLIST,
        [
            "custody_id",
            "custody_check",
            "required_metadata",
            "pass_condition",
            "failure_behavior",
            "held_claims",
        ],
        custody_rows(),
    )
    write_csv(
        HANDOFF,
        [
            "handoff_id",
            "decision_gate",
            "required_condition",
            "pass_next_step",
            "hold_behavior",
            "initial_decision",
            "held_claims",
        ],
        handoff_rows(),
    )
    write_docs()
    for path in [OWNER_DOCKET, CUSTODY_CHECKLIST, HANDOFF, BRIEF, REVIEW]:
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
