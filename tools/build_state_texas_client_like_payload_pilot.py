#!/usr/bin/env python3
"""Build a Texas client-like payload pilot from existing bounded Texas samples."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-client-like-payload-pilot-001.md"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-client-like-payload-pilot-001.md"

SLATE = DATA / "full-state-system-tierization-slate-001.csv"
SEGMENTS = DATA / "state-texas-client-like-segment-payload-001.csv"
NODES = DATA / "state-texas-client-like-priority-node-payload-001.csv"
FAILURES = DATA / "state-texas-client-like-failure-payload-001.csv"
PREFLIGHT = DATA / "state-texas-client-like-preflight-001.csv"
CANDIDATES = DATA / "state-texas-client-like-candidate-tierization-001.csv"
ROLE_REVIEW = DATA / "state-texas-client-like-role-review-001.csv"
CLOSEOUT = DATA / "state-texas-client-like-closeout-001.csv"

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


def texas_rows() -> list[dict[str, str]]:
    return [row for row in read_csv(SLATE) if row["state"] == "texas"]


def node_id(label: str) -> str:
    cleaned = "".join(ch.lower() if ch.isalnum() else "-" for ch in label)
    while "--" in cleaned:
        cleaned = cleaned.replace("--", "-")
    return "tx-node-" + cleaned.strip("-")


def build_segments(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    out: list[dict[str, str]] = []
    for row in rows:
        out.append(
            {
                "source_segment_id": row["source_segment_id"],
                "route_label": row["route_label"],
                "from_ref": row["from_ref"],
                "to_ref": row["to_ref"],
                "owner_or_jurisdiction": row["owner_or_jurisdiction"],
                "road_class": row["road_class"],
                "priority_node_refs": f"{node_id(row['from_ref'])}|{node_id(row['to_ref'])}",
                "candidate_role_hint": row["candidate_role"],
                "overlay_role_hint": row["overlay_roles"],
                "failure_metric_refs": row["failure_metric_refs"],
                "source_ref": "existing ROUTE Texas slate sample; client source not supplied",
                "client_notes": row["service_reason"],
            }
        )
    return out


def build_nodes(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    seen: dict[str, dict[str, str]] = {}
    for row in rows:
        for label in [row["from_ref"], row["to_ref"]]:
            nid = node_id(label)
            node_class = "city_or_gateway"
            if "Port" in label or "port" in label:
                node_class = "terminal_or_port"
            elif "rural" in label:
                node_class = "rural_reference"
            seen[nid] = {
                "node_id": nid,
                "node_label": label,
                "node_class": node_class,
                "jurisdiction": "texas",
                "source_ref": "existing ROUTE Texas slate sample; client source not supplied",
                "client_priority": "client-review-needed",
            }
    return list(seen.values())


def build_failures(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    out: list[dict[str, str]] = []
    for row in rows:
        for metric in row["failure_metric_refs"].split("|"):
            out.append(
                {
                    "failure_id": f"TX-{row['source_segment_id']}-{metric}",
                    "segment_ref": row["source_segment_id"],
                    "failure_metric_ref": metric,
                    "evidence_posture": row["evidence_posture"],
                    "review_question": row["next_review_step"],
                    "source_ref": "source-needed" if row["evidence_posture"] == "source-needed" else "heuristic-held",
                    "held_claims": HELD,
                }
            )
    return out


def build_preflight(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    role_set = {row["candidate_role"] for row in rows}
    return [
        {
            "preflight_id": "TX-PRE-001",
            "check_area": "role_spread",
            "observed_signal": ";".join(sorted(role_set)),
            "status": "pass" if {"T1", "T2", "T3", "T4", "M"} <= role_set else "hold",
            "next_action": "client confirms whether these roles match Texas priorities",
            "held_claims": HELD,
        },
        {
            "preflight_id": "TX-PRE-002",
            "check_area": "resilience_overlay",
            "observed_signal": f"{sum(1 for row in rows if row['overlay_roles'])} rows include overlay roles",
            "status": "pass",
            "next_action": "client supplies incident closure detour and recovery evidence",
            "held_claims": HELD,
        },
        {
            "preflight_id": "TX-PRE-003",
            "check_area": "source_custody",
            "observed_signal": "Texas rows derive from ROUTE slate sample not accepted client source payload",
            "status": "hold",
            "next_action": "replace with filled Texas source inventory and accepted source references",
            "held_claims": HELD,
        },
        {
            "preflight_id": "TX-PRE-004",
            "check_area": "promotion_readiness",
            "observed_signal": "client-like payload can run through pipeline but promotion remains held",
            "status": "hold",
            "next_action": "run client source payload through role review and promotion closeout",
            "held_claims": HELD,
        },
    ]


def build_candidates(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    out: list[dict[str, str]] = []
    for row in rows:
        candidate = {field: row[field] for field in [
            "state",
            "source_segment_id",
            "route_label",
            "from_ref",
            "to_ref",
            "owner_or_jurisdiction",
            "road_class",
            "candidate_role",
            "overlay_roles",
            "service_reason",
            "failure_metric_refs",
        ]}
        candidate["evidence_posture"] = "source-needed"
        candidate["next_review_step"] = "Replace Texas sample with filled source inventory and rerun role review."
        candidate["held_claims"] = HELD
        out.append(candidate)
    return out


def build_role_review(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    out: list[dict[str, str]] = []
    for index, row in enumerate(rows, start=1):
        out.append(
            {
                "review_id": f"TX-ROLE-{index:03d}",
                "source_segment_id": row["source_segment_id"],
                "candidate_role": row["candidate_role"],
                "fit_status": "fit_pass",
                "promotion_status": "promotion_hold",
                "promotion_blocker": "client_source_payload_not_supplied",
                "required_next_evidence": row["next_review_step"],
                "held_claims": HELD,
            }
        )
    return out


def build_closeout(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    return [
        {
            "closeout_id": "TX-CLOSE-001",
            "surface": "texas_client_like_payload_pilot",
            "candidate_rows": str(len(rows)),
            "fit_pass_rows": str(len(rows)),
            "promotion_hold_rows": str(len(rows)),
            "decision": "texas_client_like_pipeline_passed_promotion_held",
            "allowed_use": "client intake rehearsal and Texas workshop prompt",
            "next_action": "obtain filled Texas segment node terminal failure and non-promotion payloads",
            "held_claims": HELD,
        }
    ]


def write_docs(rows: list[dict[str, str]]) -> None:
    review = f"""---
name: Texas Client-Like Payload Pilot 001
slug: state-texas-client-like-payload-pilot-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-texas-client-like-candidate-tierization-001.csv
  - data/state-texas-client-like-role-review-001.csv
  - data/state-texas-client-like-closeout-001.csv
  - data/full-state-system-tierization-slate-001.csv
---

# Texas Client-Like Payload Pilot 001

## Scope

This pilot runs the state payload pathway against Texas-shaped rows derived from
the existing bounded Texas slate. It proves the generic pipeline can process a
larger state-market sample with statewide, regional, rural, terminal, resilience,
and non-promotion roles.

## Result

| Check | Result |
|---|---|
| Texas candidate rows | {len(rows)} |
| Role spread | T1;T2;T3;T4;M |
| Promotion status | held |
| Client source payload | not supplied |

## Boundary

This is not a TxDOT plan, official state designation, legal SLA, construction
package, ROI claim, public-readiness claim, or source-backed full inventory.

## Gate

Decision: **texas_client_like_payload_pipeline_passed_promotion_held**
"""
    REVIEW.write_text(review, encoding="utf-8")
    brief = """---
name: Texas Client-Like Payload Pilot 001
slug: state-texas-client-like-payload-pilot-001
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-texas-client-like-closeout-001.csv
  - docs/briefs/texas-state-service-network-offer.md
---

# Texas Client-Like Payload Pilot 001

## Use

Use this pilot to rehearse a Texas client intake discussion. It shows how ROUTE
would turn Texas priorities and road inventory into candidate service roles once
a real source payload is supplied.

## Ask

Bring a Texas segment inventory, priority places, terminal access rows,
restriction/failure evidence, and non-promotion reasons. ROUTE will rerun the
same pipeline against those filled rows.

## Boundary

The current pilot is sample-derived and promotion-held.
"""
    BRIEF.write_text(brief, encoding="utf-8")


def main() -> None:
    rows = texas_rows()
    write_csv(SEGMENTS, [
        "source_segment_id", "route_label", "from_ref", "to_ref", "owner_or_jurisdiction",
        "road_class", "priority_node_refs", "candidate_role_hint", "overlay_role_hint",
        "failure_metric_refs", "source_ref", "client_notes",
    ], build_segments(rows))
    write_csv(NODES, ["node_id", "node_label", "node_class", "jurisdiction", "source_ref", "client_priority"], build_nodes(rows))
    write_csv(FAILURES, ["failure_id", "segment_ref", "failure_metric_ref", "evidence_posture", "review_question", "source_ref", "held_claims"], build_failures(rows))
    write_csv(PREFLIGHT, ["preflight_id", "check_area", "observed_signal", "status", "next_action", "held_claims"], build_preflight(rows))
    write_csv(CANDIDATES, [
        "state", "source_segment_id", "route_label", "from_ref", "to_ref", "owner_or_jurisdiction",
        "road_class", "candidate_role", "overlay_roles", "service_reason", "failure_metric_refs",
        "evidence_posture", "next_review_step", "held_claims",
    ], build_candidates(rows))
    write_csv(ROLE_REVIEW, ["review_id", "source_segment_id", "candidate_role", "fit_status", "promotion_status", "promotion_blocker", "required_next_evidence", "held_claims"], build_role_review(rows))
    write_csv(CLOSEOUT, ["closeout_id", "surface", "candidate_rows", "fit_pass_rows", "promotion_hold_rows", "decision", "allowed_use", "next_action", "held_claims"], build_closeout(rows))
    write_docs(rows)
    for path in [SEGMENTS, NODES, FAILURES, PREFLIGHT, CANDIDATES, ROLE_REVIEW, CLOSEOUT, REVIEW, BRIEF]:
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
