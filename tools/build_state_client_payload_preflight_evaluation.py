#!/usr/bin/env python3
"""Evaluate generic state client payload templates against the adapter contract."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
REVIEW = ROOT / "docs" / "reviews" / "state-client-payload-preflight-evaluation-001.md"

SEGMENTS = DATA / "state-client-payload-segment-template-001.csv"
NODES = DATA / "state-client-payload-priority-node-template-001.csv"
TERMINALS = DATA / "state-client-payload-terminal-access-template-001.csv"
FAILURES = DATA / "state-client-payload-restriction-failure-template-001.csv"
NON_PROMOTION = DATA / "state-client-payload-non-promotion-template-001.csv"
MANIFEST = DATA / "state-client-payload-manifest-001.csv"
OUTPUT = DATA / "state-client-payload-preflight-evaluation-001.csv"

BLOCKED = (
    "official_designation;legal_sla;construction;cost;numeric_roi;roi;"
    "eligibility;compliance;endorsement;validation;public_readiness;"
    "state_approval;source_backed_full_inventory"
)

FIELDS = [
    "check_id",
    "check_area",
    "input_artifact",
    "observed_signal",
    "evaluation_status",
    "next_action",
    "blocked_claims",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def fieldnames(path: Path) -> set[str]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.reader(f)
        return set(next(reader))


def status(ok: bool) -> str:
    return "pass" if ok else "hold"


def build_rows() -> list[dict[str, str]]:
    segment_rows = read_csv(SEGMENTS)
    node_rows = read_csv(NODES)
    terminal_rows = read_csv(TERMINALS)
    failure_rows = read_csv(FAILURES)
    non_promotion_rows = read_csv(NON_PROMOTION)
    manifest_rows = read_csv(MANIFEST)

    segment_ids = {row["source_segment_id"] for row in segment_rows}
    node_ids = {row["node_id"] for row in node_rows}
    terminal_route_refs = {row["access_route_ref"] for row in terminal_rows if row["access_route_ref"]}
    failure_segment_refs = {row["segment_ref"] for row in failure_rows if row["segment_ref"]}
    non_promotion_refs = {row["source_segment_id"] for row in non_promotion_rows if row["source_segment_id"]}
    segment_node_refs = {
        ref
        for row in segment_rows
        for ref in row.get("priority_node_refs", "").split("|")
        if ref
    }

    rows: list[dict[str, str]] = []
    rows.append(
        {
            "check_id": "EVAL-001",
            "check_area": "manifest_completeness",
            "input_artifact": "data/state-client-payload-manifest-001.csv",
            "observed_signal": f"{len(manifest_rows)} payload templates listed",
            "evaluation_status": status(len(manifest_rows) == 5),
            "next_action": "keep all five payload surfaces in client intake packet",
            "blocked_claims": BLOCKED,
        }
    )
    required_segment_fields = {
        "source_segment_id",
        "route_label",
        "from_ref",
        "to_ref",
        "owner_or_jurisdiction",
        "road_class",
        "source_ref",
    }
    missing_segment_fields = required_segment_fields - fieldnames(SEGMENTS)
    rows.append(
        {
            "check_id": "EVAL-002",
            "check_area": "segment_shape",
            "input_artifact": "data/state-client-payload-segment-template-001.csv",
            "observed_signal": "missing fields " + ";".join(sorted(missing_segment_fields)) if missing_segment_fields else f"{len(segment_rows)} sample segment rows have minimum fields",
            "evaluation_status": status(not missing_segment_fields and len(segment_rows) >= 1),
            "next_action": "replace sample rows with client road inventory rows",
            "blocked_claims": BLOCKED,
        }
    )
    missing_node_refs = segment_node_refs - node_ids
    rows.append(
        {
            "check_id": "EVAL-003",
            "check_area": "priority_node_references",
            "input_artifact": "data/state-client-payload-priority-node-template-001.csv",
            "observed_signal": "missing node refs " + ";".join(sorted(missing_node_refs)) if missing_node_refs else "segment priority_node_refs resolve to node template",
            "evaluation_status": status(not missing_node_refs),
            "next_action": "client confirms node classes and adds missing node rows",
            "blocked_claims": BLOCKED,
        }
    )
    missing_terminal_refs = terminal_route_refs - segment_ids
    rows.append(
        {
            "check_id": "EVAL-004",
            "check_area": "terminal_access_references",
            "input_artifact": "data/state-client-payload-terminal-access-template-001.csv",
            "observed_signal": "missing segment refs " + ";".join(sorted(missing_terminal_refs)) if missing_terminal_refs else "terminal access_route_ref resolves to segment template",
            "evaluation_status": status(not missing_terminal_refs),
            "next_action": "client supplies access route rows for each terminal",
            "blocked_claims": BLOCKED,
        }
    )
    missing_failure_refs = failure_segment_refs - segment_ids
    rows.append(
        {
            "check_id": "EVAL-005",
            "check_area": "restriction_failure_references",
            "input_artifact": "data/state-client-payload-restriction-failure-template-001.csv",
            "observed_signal": "missing segment refs " + ";".join(sorted(missing_failure_refs)) if missing_failure_refs else "restriction and failure segment_ref values resolve to segment template",
            "evaluation_status": status(not missing_failure_refs),
            "next_action": "client attaches source references for restrictions and failures",
            "blocked_claims": BLOCKED,
        }
    )
    missing_non_promotion_refs = non_promotion_refs - segment_ids
    rows.append(
        {
            "check_id": "EVAL-006",
            "check_area": "non_promotion_references",
            "input_artifact": "data/state-client-payload-non-promotion-template-001.csv",
            "observed_signal": "missing segment refs " + ";".join(sorted(missing_non_promotion_refs)) if missing_non_promotion_refs else "non-promotion rows resolve to segment template",
            "evaluation_status": status(not missing_non_promotion_refs),
            "next_action": "client records non-promotion reasons for every unpromoted inventory segment",
            "blocked_claims": BLOCKED,
        }
    )
    rows.append(
        {
            "check_id": "EVAL-007",
            "check_area": "source_custody",
            "input_artifact": "data/state-client-payload-*-template-001.csv",
            "observed_signal": "templates contain sample source_ref placeholders only",
            "evaluation_status": "hold",
            "next_action": "client replaces placeholders with accepted source references before source-backed fit",
            "blocked_claims": BLOCKED,
        }
    )
    rows.append(
        {
            "check_id": "EVAL-008",
            "check_area": "promotion_readiness",
            "input_artifact": "data/state-client-payload-preflight-evaluation-001.csv",
            "observed_signal": "template integrity can be checked but no real client payload has been reviewed",
            "evaluation_status": "hold",
            "next_action": "run evaluator against a filled client payload and then route rows to role review",
            "blocked_claims": BLOCKED,
        }
    )
    return rows


def write_csv(path: Path, rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def write_review(rows: list[dict[str, str]]) -> None:
    pass_count = sum(1 for row in rows if row["evaluation_status"] == "pass")
    hold_count = sum(1 for row in rows if row["evaluation_status"] == "hold")
    review = f"""---
name: State Client Payload Preflight Evaluation 001
slug: state-client-payload-preflight-evaluation-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-client-payload-preflight-evaluation-001.csv
  - data/state-client-payload-manifest-001.csv
  - data/state-client-payload-segment-template-001.csv
  - data/state-client-payload-priority-node-template-001.csv
  - data/state-client-payload-terminal-access-template-001.csv
  - data/state-client-payload-restriction-failure-template-001.csv
  - data/state-client-payload-non-promotion-template-001.csv
---

# State Client Payload Preflight Evaluation 001

## Scope

This evaluation reads the generic state client payload templates and checks
whether the package is internally coherent enough to accept a filled client
payload.

## Result

| Check | Result |
|---|---|
| Pass rows | {pass_count} |
| Hold rows | {hold_count} |
| Real client data reviewed | no |
| Source custody accepted | no |

## Evidence Boundary

This evaluates template integrity and cross-references only. It does not validate
client data, source custody, official designations, legal SLAs, construction
readiness, cost, numeric ROI, eligibility, compliance, endorsement, public
readiness, state approval, or source-backed full inventory.

## Gate

Decision: **state_client_payload_preflight_ready_for_filled_payload**
"""
    REVIEW.write_text(review, encoding="utf-8")


def main() -> None:
    rows = build_rows()
    write_csv(OUTPUT, rows)
    write_review(rows)
    print(f"wrote {OUTPUT}")
    print(f"wrote {REVIEW}")


if __name__ == "__main__":
    main()
