#!/usr/bin/env python3
"""Build candidate tierization rows from the generic state client payload sample."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
REVIEW = ROOT / "docs" / "reviews" / "state-payload-candidate-tierization-001.md"

SEGMENTS = DATA / "state-client-payload-segment-template-001.csv"
TERMINALS = DATA / "state-client-payload-terminal-access-template-001.csv"
FAILURES = DATA / "state-client-payload-restriction-failure-template-001.csv"
NON_PROMOTION = DATA / "state-client-payload-non-promotion-template-001.csv"
OUTPUT = DATA / "state-payload-candidate-tierization-001.csv"
ROLE_REVIEW = DATA / "state-payload-candidate-role-review-001.csv"

BLOCKED = (
    "official_designation|legal_sla|construction|cost|numeric_roi|roi|"
    "eligibility|compliance|endorsement|validation|public_readiness|"
    "state_approval|source_backed_full_inventory"
)

TIER_FIELDS = [
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
    "evidence_posture",
    "next_review_step",
    "held_claims",
]

ROLE_REVIEW_FIELDS = [
    "review_id",
    "source_segment_id",
    "candidate_role",
    "fit_vectors",
    "fit_reason",
    "review_status",
    "required_next_evidence",
    "held_claims",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def role_for(row: dict[str, str], non_promotion_ids: set[str], terminal_route_ids: set[str]) -> tuple[str, str, str, str]:
    segment_id = row["source_segment_id"]
    road_class = row["road_class"].lower()
    route = row["route_label"].lower()
    priority_refs = row.get("priority_node_refs", "")
    alt_refs = row.get("parallel_or_alternate_refs", "")
    restriction_refs = row.get("restriction_refs", "")

    if segment_id in non_promotion_ids:
        return (
            "M",
            "",
            "SV-006",
            "Maintained inventory row with explicit non-promotion reason from payload.",
        )
    if segment_id in terminal_route_ids or "terminal" in road_class or "local-access" in route:
        return (
            "T4",
            "R" if restriction_refs else "",
            "SV-004;SV-005",
            "Terminal or local access row controls connection from terminal to nearest tier node.",
        )
    if "interstate" in road_class or route.startswith("i-"):
        return (
            "T1",
            "R" if alt_refs or restriction_refs else "",
            "SV-001;SV-002;SV-005",
            "Statewide trunk candidate with priority nodes and possible alternate or failure evidence.",
        )
    if "state-highway" in road_class and priority_refs and alt_refs:
        return (
            "T2",
            "R",
            "SV-002;SV-003;SV-005",
            "Regional connector candidate with alternate relationship and priority-node coverage.",
        )
    if "state-highway" in road_class and priority_refs:
        return (
            "T3",
            "R" if restriction_refs else "",
            "SV-003;SV-005",
            "Access continuity candidate requiring client review of service role.",
        )
    return (
        "M",
        "",
        "SV-006",
        "Inventory row held as maintenance or monitor until role evidence is provided.",
    )


def metric_refs_for(row: dict[str, str], failure_by_segment: dict[str, list[str]], role: str) -> str:
    metrics = set(failure_by_segment.get(row["source_segment_id"], []))
    if role == "T1":
        metrics.update(["SSF-001", "SSF-002"])
    elif role == "T2":
        metrics.update(["SSF-002", "SSF-004"])
    elif role == "T3":
        metrics.update(["SSF-004", "SSF-006"])
    elif role == "T4":
        metrics.update(["SSF-005", "SSF-007"])
    elif role in {"M", "X"}:
        metrics.add("SSF-008")
    return "|".join(sorted(metrics))


def build() -> tuple[list[dict[str, str]], list[dict[str, str]]]:
    segments = read_csv(SEGMENTS)
    terminals = read_csv(TERMINALS)
    failures = read_csv(FAILURES)
    non_promotion = read_csv(NON_PROMOTION)
    terminal_route_ids = {row["access_route_ref"] for row in terminals}
    non_promotion_ids = {row["source_segment_id"] for row in non_promotion}
    failure_by_segment: dict[str, list[str]] = {}
    for row in failures:
        failure_by_segment.setdefault(row["segment_ref"], []).append(row["failure_metric_ref"])

    tier_rows: list[dict[str, str]] = []
    review_rows: list[dict[str, str]] = []
    for idx, row in enumerate(segments, start=1):
        role, overlay, vectors, reason = role_for(row, non_promotion_ids, terminal_route_ids)
        evidence_posture = "source-needed"
        tier_rows.append(
            {
                "state": "example-state",
                "source_segment_id": row["source_segment_id"],
                "route_label": row["route_label"],
                "from_ref": row["from_ref"],
                "to_ref": row["to_ref"],
                "owner_or_jurisdiction": row["owner_or_jurisdiction"],
                "road_class": row["road_class"],
                "candidate_role": role,
                "overlay_roles": overlay,
                "service_reason": reason,
                "failure_metric_refs": metric_refs_for(row, failure_by_segment, role),
                "evidence_posture": evidence_posture,
                "next_review_step": "Replace sample payload with accepted client source rows and run role review.",
                "held_claims": BLOCKED,
            }
        )
        review_rows.append(
            {
                "review_id": f"ROLE-FIT-{idx:03d}",
                "source_segment_id": row["source_segment_id"],
                "candidate_role": role,
                "fit_vectors": vectors,
                "fit_reason": reason,
                "review_status": "role_review_required",
                "required_next_evidence": "accepted source references plus client priority-node confirmation plus failure metric review",
                "held_claims": BLOCKED,
            }
        )
    return tier_rows, review_rows


def write_csv(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def write_review(tier_rows: list[dict[str, str]], review_rows: list[dict[str, str]]) -> None:
    roles = sorted({row["candidate_role"] for row in tier_rows})
    review = f"""---
name: State Payload Candidate Tierization 001
slug: state-payload-candidate-tierization-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-payload-candidate-tierization-001.csv
  - data/state-payload-candidate-role-review-001.csv
  - data/state-client-payload-preflight-evaluation-001.csv
  - data/state-tierization-fit-role-vector-profile-001.csv
---

# State Payload Candidate Tierization 001

## Scope

This review applies the state fit kernel to the generic client payload sample and
emits candidate T1/T2/T4/M rows plus role-review requirements.

## Result

| Check | Result |
|---|---|
| Candidate tier rows | {len(tier_rows)} |
| Role review rows | {len(review_rows)} |
| Candidate roles emitted | {";".join(roles)} |
| Evidence posture | source-needed |

## Evidence Boundary

This is a sample candidate fit from template payload rows. It does not validate
client data, source custody, official designations, legal SLAs, construction
readiness, cost, numeric ROI, eligibility, compliance, endorsement, public
readiness, state approval, or source-backed full inventory.

## Gate

Decision: **state_payload_candidate_tierization_ready_for_filled_payload_role_review**
"""
    REVIEW.write_text(review, encoding="utf-8")


def main() -> None:
    tier_rows, review_rows = build()
    write_csv(OUTPUT, TIER_FIELDS, tier_rows)
    write_csv(ROLE_REVIEW, ROLE_REVIEW_FIELDS, review_rows)
    write_review(tier_rows, review_rows)
    print(f"wrote {OUTPUT}")
    print(f"wrote {ROLE_REVIEW}")
    print(f"wrote {REVIEW}")


if __name__ == "__main__":
    main()
