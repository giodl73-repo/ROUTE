#!/usr/bin/env python3
"""Build Texas source-backed pilot plan from diagnostic readout and source asks."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-source-backed-pilot-plan-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-source-backed-pilot-plan-001.md"

PLAN = DATA / "state-texas-source-backed-pilot-plan-001.csv"
ACCEPTANCE = DATA / "state-texas-source-backed-pilot-acceptance-001.csv"
BLOCKERS = DATA / "state-texas-source-backed-pilot-blockers-001.csv"

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


def plan_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-PILOT-001", "payload_owner_assignment", "Name owners for Texas segment inventory priority nodes failure evidence terminal access and non-promotion rows.", "state-texas-client-intake-source-asks-001.csv", "all four source surfaces have named owner and delivery date", "block_run", "source-needed"),
        ("TX-PILOT-002", "payload_receipt", "Receive filled Texas payloads in ROUTE template shape.", "state-client-payload-*-template-001.csv", "required columns present and sample rows replaced or clearly marked", "hold_source_backed_fit", "source-needed"),
        ("TX-PILOT-003", "source_custody_review", "Review source references for segment identity priority nodes failures terminal access and non-promotion reasons.", "filled Texas payload source_ref columns", "accepted source_ref or source-needed disposition for every row", "hold_promotion", "source-needed"),
        ("TX-PILOT-004", "candidate_generation", "Run fit kernel against accepted Texas payload rows.", "state payload candidate tierization tools", "candidate rows emitted with evidence posture and held claims", "hold_role_review", "source-needed"),
        ("TX-PILOT-005", "role_review", "Review T1 T2 T3 T4 R M X assignments with Texas owner.", "candidate tierization and role review outputs", "each row pass hold or fail with next evidence", "hold_promotion", "source-needed"),
        ("TX-PILOT-006", "sponsor_closeout", "Summarize what can be used internally and what remains blocked.", "promotion closeout outputs", "promotion decision and next-action ledger", "hold_public_claims", "source-needed"),
    ]
    return [
        {
            "step_id": step_id,
            "pilot_step": step,
            "work": work,
            "input_artifact": artifact,
            "pass_condition": pass_condition,
            "fail_or_hold_behavior": behavior,
            "initial_posture": posture,
            "held_claims": HELD,
        }
        for step_id, step, work, artifact, pass_condition, behavior, posture in rows
    ]


def acceptance_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-ACCEPT-001", "segment_inventory", "source_segment_id route_label from_ref to_ref owner_or_jurisdiction road_class source_ref", "every candidate row has stable id endpoints owner class and source reference", "without this no source-backed candidate fit"),
        ("TX-ACCEPT-002", "priority_nodes", "node_id node_label node_class jurisdiction source_ref", "priority nodes resolve all segment priority references", "without this roles remain workshop prompts"),
        ("TX-ACCEPT-003", "failure_evidence", "segment_ref failure_metric_ref source_ref review_status", "failure evidence maps to SSF metric or explicit source-needed gap", "without this resilience and failure scorecards remain held"),
        ("TX-ACCEPT-004", "terminal_access", "terminal_id terminal_class access_route_ref nearest_tier_node source_ref", "terminal access rows resolve to segment and node references", "without this T4 rows remain source-needed"),
        ("TX-ACCEPT-005", "non_promotion", "source_segment_id coverage_status non_promotion_reason review_owner", "unpromoted rows have explicit M/X reason", "without this full-coverage claim remains blocked"),
        ("TX-ACCEPT-006", "claim_boundary", "held_claims and evidence_posture columns", "all outputs preserve official SLA ROI construction approval validation and full-inventory holds unless separately unlocked", "without this packet is not promotable"),
    ]
    return [
        {
            "acceptance_id": item_id,
            "surface": surface,
            "required_fields": fields,
            "pass_condition": pass_condition,
            "hold_consequence": consequence,
            "held_claims": HELD,
        }
        for item_id, surface, fields, pass_condition, consequence in rows
    ]


def blocker_rows() -> list[dict[str, str]]:
    rows = [
        ("TX-BLOCK-001", "no_filled_payload", "No real Texas payload has been supplied.", "Use client-like pilot only.", "request source owners and delivery date"),
        ("TX-BLOCK-002", "source_ref_placeholders", "Rows contain placeholder or internal sample source refs.", "Keep source custody held.", "replace placeholders with accepted source references"),
        ("TX-BLOCK-003", "role_owner_missing", "No Texas owner has reviewed candidate tier assignment.", "Keep promotion held.", "schedule role review workshop"),
        ("TX-BLOCK-004", "failure_evidence_missing", "Failure metrics are not tied to accepted incident restriction closure or terminal evidence.", "Keep resilience and scorecard claims held.", "attach evidence or mark source-needed gap"),
        ("TX-BLOCK-005", "full_inventory_gap", "Sample rows do not represent complete Texas inventory.", "Block full-inventory claim.", "account for all source segments or scope the pilot explicitly"),
    ]
    return [
        {
            "blocker_id": blocker_id,
            "blocker": blocker,
            "diagnostic": diagnostic,
            "required_behavior": behavior,
            "clearance_action": action,
            "held_claims": HELD,
        }
        for blocker_id, blocker, diagnostic, behavior, action in rows
    ]


def write_docs() -> None:
    brief = """---
name: Texas Source-Backed Pilot Plan 001
slug: state-texas-source-backed-pilot-plan-001
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-texas-source-backed-pilot-plan-001.csv
  - data/state-texas-source-backed-pilot-acceptance-001.csv
  - data/state-texas-source-backed-pilot-blockers-001.csv
  - data/state-texas-diagnostic-readout-001.csv
---

# Texas Source-Backed Pilot Plan 001

## Purpose

This plan defines the next Texas run after the client-like pilot. It tells the
delivery team what must arrive, what must be checked, and when to stop rather
than promote unsupported claims.

## Pilot Decision

Run only after Texas source owners are named and a filled payload is supplied.

## Boundary

This is a pilot plan, not a TxDOT plan, legal SLA, official route tier,
construction package, ROI proof, validation, public-readiness packet, or
source-backed full inventory.
"""
    BRIEF.write_text(brief, encoding="utf-8")
    review = """---
name: Texas Source-Backed Pilot Plan 001
slug: state-texas-source-backed-pilot-plan-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - docs/briefs/state-texas-source-backed-pilot-plan-001.md
  - data/state-texas-source-backed-pilot-plan-001.csv
  - data/state-texas-source-backed-pilot-acceptance-001.csv
  - data/state-texas-source-backed-pilot-blockers-001.csv
---

# Texas Source-Backed Pilot Plan 001

## Scope

This review confirms the Texas diagnostic readout now has a source-backed pilot
run plan with acceptance gates and blocker behavior.

## Gate

Decision: **texas_source_backed_pilot_plan_ready_awaiting_payload**
"""
    REVIEW.write_text(review, encoding="utf-8")


def main() -> None:
    write_csv(PLAN, ["step_id", "pilot_step", "work", "input_artifact", "pass_condition", "fail_or_hold_behavior", "initial_posture", "held_claims"], plan_rows())
    write_csv(ACCEPTANCE, ["acceptance_id", "surface", "required_fields", "pass_condition", "hold_consequence", "held_claims"], acceptance_rows())
    write_csv(BLOCKERS, ["blocker_id", "blocker", "diagnostic", "required_behavior", "clearance_action", "held_claims"], blocker_rows())
    write_docs()
    for path in [PLAN, ACCEPTANCE, BLOCKERS, BRIEF, REVIEW]:
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
