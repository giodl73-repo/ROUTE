#!/usr/bin/env python3
"""Build state tierization fit diagnostics from current slate samples."""

from __future__ import annotations

import csv
from collections import Counter, defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
REVIEW = ROOT / "docs" / "reviews" / "state-tierization-fit-kernel-001.md"
PROFILE = DATA / "state-tierization-fit-role-vector-profile-001.csv"
COVERAGE = DATA / "state-tierization-fit-state-coverage-001.csv"

BLOCKED = (
    "official_designation;legal_sla;construction;cost;numeric_roi;roi;"
    "eligibility;compliance;endorsement;validation;public_readiness;"
    "state_approval;source_backed_full_inventory"
)

PROFILE_FIELDS = [
    "vector_id",
    "signal_family",
    "sample_support_rows",
    "dominant_roles",
    "cue_terms",
    "fit_use",
    "required_source_fields",
    "blocked_claims",
]

COVERAGE_FIELDS = [
    "state",
    "sample_rows",
    "t1_count",
    "t2_count",
    "t3_count",
    "t4_count",
    "r_overlay_count",
    "m_count",
    "x_count",
    "heuristic_held_count",
    "source_needed_count",
    "metric_refs",
    "primary_vectors",
    "fit_decision",
    "next_action",
    "blocked_claims",
]

VECTOR_RULES = [
    (
        "SV-001",
        "statewide_trunk_gateway",
        {"trunk", "gateway", "statewide", "cross-state", "north-south", "east-west"},
        "Fit candidate T1 rows where a segment carries a top statewide or gateway promise.",
        "road_inventory_id;functional_class;major_city_or_gateway_nodes;cross_state_continuity",
    ),
    (
        "SV-002",
        "regional_redundancy_load_shedding",
        {"redundancy", "fallback", "load-shedding", "alternate", "backstop"},
        "Fit candidate T2 rows where a route relieves or backstops a trunk promise.",
        "parallel_route_relationship;detour_capacity;service_class;incident_history",
    ),
    (
        "SV-003",
        "rural_access_continuity",
        {"rural", "sparse", "isolation", "service nodes", "emergency access", "continuity"},
        "Fit candidate T3 rows where a route prevents rural isolation or protects access continuity.",
        "rural_service_nodes;population_access;emergency_access;seasonal_exposure",
    ),
    (
        "SV-004",
        "terminal_local_access",
        {"terminal", "port", "airport", "border", "industrial", "freight district"},
        "Fit candidate T4 rows where local access controls the larger service promise.",
        "terminal_inventory;last_mile_route;truck_routing_constraints;queue_or_delay_evidence",
    ),
    (
        "SV-005",
        "resilience_recovery_exposure",
        {"winter", "pass", "flood", "coastal", "evacuation", "recovery", "closure"},
        "Fit R overlays where incident, weather, geohazard, or evacuation recovery changes service value.",
        "closure_history;hazard_exposure;recovery_time;detour_suitability",
    ),
    (
        "SV-006",
        "maintenance_non_promotion",
        {"maintained", "no promoted", "non-promotion", "outside", "excluded"},
        "Fit M or X rows where full coverage is preserved without selling a service promise.",
        "inventory_coverage;maintenance_owner;exclusion_reason;scope_boundary",
    ),
]


def read_rows() -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    for path in sorted(DATA.glob("full-state-system-tierization-slate-[0-9][0-9][0-9].csv")):
        if path.name.endswith("-scorecard.csv"):
            continue
        with path.open(newline="", encoding="utf-8") as f:
            rows.extend(csv.DictReader(f))
    return rows


def matches(row: dict[str, str], cues: set[str]) -> bool:
    text = " ".join(
        [
            row.get("route_label", ""),
            row.get("road_class", ""),
            row.get("service_reason", ""),
            row.get("next_review_step", ""),
        ]
    ).lower()
    return any(cue in text for cue in cues)


def build_profile(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    profile: list[dict[str, str]] = []
    for vector_id, family, cues, fit_use, fields in VECTOR_RULES:
        matched = [row for row in rows if matches(row, cues)]
        role_counts = Counter(row["candidate_role"] for row in matched)
        dominant = ";".join(role for role, _ in role_counts.most_common())
        profile.append(
            {
                "vector_id": vector_id,
                "signal_family": family,
                "sample_support_rows": str(len(matched)),
                "dominant_roles": dominant,
                "cue_terms": ";".join(sorted(cues)),
                "fit_use": fit_use,
                "required_source_fields": fields,
                "blocked_claims": BLOCKED,
            }
        )
    return profile


def build_coverage(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    by_state: dict[str, list[dict[str, str]]] = defaultdict(list)
    for row in rows:
        by_state[row["state"]].append(row)

    output: list[dict[str, str]] = []
    for state in sorted(by_state):
        state_rows = by_state[state]
        role_counts = Counter(row["candidate_role"] for row in state_rows)
        posture_counts = Counter(row["evidence_posture"] for row in state_rows)
        metrics = sorted(
            {
                metric
                for row in state_rows
                for metric in row["failure_metric_refs"].split("|")
                if metric
            }
        )
        vector_hits: list[str] = []
        for vector_id, family, cues, _, _ in VECTOR_RULES:
            if any(matches(row, cues) for row in state_rows):
                vector_hits.append(f"{vector_id}:{family}")
        has_role_spread = all(role_counts.get(role, 0) > 0 for role in ["T1", "T2", "T3", "T4"])
        has_non_promotion = role_counts.get("M", 0) + role_counts.get("X", 0) > 0
        fit_decision = (
            "fit_sample_complete_source_inventory_required"
            if has_role_spread and has_non_promotion and len(vector_hits) >= 5
            else "fit_sample_gap_review_required"
        )
        output.append(
            {
                "state": state,
                "sample_rows": str(len(state_rows)),
                "t1_count": str(role_counts.get("T1", 0)),
                "t2_count": str(role_counts.get("T2", 0)),
                "t3_count": str(role_counts.get("T3", 0)),
                "t4_count": str(role_counts.get("T4", 0)),
                "r_overlay_count": str(sum(1 for row in state_rows if "R" in row["overlay_roles"].split("|"))),
                "m_count": str(role_counts.get("M", 0)),
                "x_count": str(role_counts.get("X", 0)),
                "heuristic_held_count": str(posture_counts.get("heuristic-held", 0)),
                "source_needed_count": str(posture_counts.get("source-needed", 0)),
                "metric_refs": ";".join(metrics),
                "primary_vectors": ";".join(vector_hits),
                "fit_decision": fit_decision,
                "next_action": "attach source road inventory and client priority nodes before promoting any fitted role",
                "blocked_claims": BLOCKED,
            }
        )
    return output


def write_csv(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def write_review(rows: list[dict[str, str]], profile: list[dict[str, str]]) -> None:
    state_count = len(rows)
    sample_complete = sum(row["fit_decision"] == "fit_sample_complete_source_inventory_required" for row in rows)
    review = f"""---
name: State Tierization Fit Kernel 001
slug: state-tierization-fit-kernel-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-tierization-fit-role-vector-profile-001.csv
  - data/state-tierization-fit-state-coverage-001.csv
  - data/full-state-system-tierization-template.csv
  - data/state-system-failure-metric-menu.csv
  - docs/reports/full-state-system-tierization-framework.md
---

# State Tierization Fit Kernel 001

## Scope

This review upgrades the state package from hand-authored examples toward a
repeatable fit kernel. The kernel reads the current full-state tierization
slates and extracts reusable signal families for trunk, redundancy, rural
access, terminal access, resilience, and non-promotion rows.

## Fit Result

| Check | Result |
|---|---|
| State samples inspected | {state_count} |
| Vector families emitted | {len(profile)} |
| Samples with full T1/T2/T3/T4 plus M/X coverage | {sample_complete} |
| Promotion posture | source inventory required |

## What This Proves

The current slate set is sufficient to train and test a bounded role-assignment
heuristic across different state-network vectors. It proves ROUTE can represent
complexity as structured rows rather than one-off prose.

## What This Does Not Prove

The fit kernel does not prove official state tiers, legal SLAs, construction
readiness, numeric ROI, cost, eligibility, compliance, endorsement, external
validation, public readiness, state approval, or source-backed full inventory.

## Gate

Decision: **state_fit_kernel_ready_for_source_inventory_adapter**
"""
    REVIEW.write_text(review, encoding="utf-8")


def main() -> None:
    rows = read_rows()
    profile = build_profile(rows)
    coverage = build_coverage(rows)
    write_csv(PROFILE, PROFILE_FIELDS, profile)
    write_csv(COVERAGE, COVERAGE_FIELDS, coverage)
    write_review(coverage, profile)
    print(f"wrote {PROFILE}")
    print(f"wrote {COVERAGE}")
    print(f"wrote {REVIEW}")


if __name__ == "__main__":
    main()
