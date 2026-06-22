#!/usr/bin/env python3
"""Build generic state source-inventory adapter contract artifacts."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
REVIEW = ROOT / "docs" / "reviews" / "state-source-inventory-adapter-contract-001.md"
PROFILE = DATA / "state-tierization-fit-role-vector-profile-001.csv"
FIELD_CONTRACT = DATA / "state-source-inventory-adapter-field-contract-001.csv"
ROW_CONTRACT = DATA / "state-source-inventory-adapter-row-contract-001.csv"
PRECHECK = DATA / "state-source-inventory-adapter-precheck-001.csv"

BLOCKED = (
    "official_designation;legal_sla;construction;cost;numeric_roi;roi;"
    "eligibility;compliance;endorsement;validation;public_readiness;"
    "state_approval;source_backed_full_inventory"
)

FIELD_CONTRACT_FIELDS = [
    "field_name",
    "field_group",
    "required_for_vectors",
    "required_for_roles",
    "accepted_source_examples",
    "normalization_rule",
    "missing_field_behavior",
    "blocked_claims",
]

ROW_CONTRACT_FIELDS = [
    "row_id",
    "input_surface",
    "minimum_columns",
    "role_fit_dependency",
    "evidence_posture_on_ingest",
    "promotion_gate",
    "blocked_claims",
]

PRECHECK_FIELDS = [
    "precheck_id",
    "adapter_step",
    "question",
    "pass_condition",
    "hold_condition",
    "output_artifact",
    "blocked_claims",
]


def read_profile() -> list[dict[str, str]]:
    with PROFILE.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def vector_index(profile: list[dict[str, str]]) -> dict[str, dict[str, str]]:
    return {row["vector_id"]: row for row in profile}


def build_field_contract(profile: list[dict[str, str]]) -> list[dict[str, str]]:
    vectors = vector_index(profile)
    return [
        {
            "field_name": "source_segment_id",
            "field_group": "identity",
            "required_for_vectors": "SV-001;SV-002;SV-003;SV-004;SV-005;SV-006",
            "required_for_roles": "T1;T2;T3;T4;R;M;X",
            "accepted_source_examples": "state roadway inventory segment id;linear referencing id;route-milepost key",
            "normalization_rule": "Preserve source key and add ROUTE row id only as a derived identifier.",
            "missing_field_behavior": "block_source_backed_fit",
            "blocked_claims": BLOCKED,
        },
        {
            "field_name": "route_label",
            "field_group": "identity",
            "required_for_vectors": "SV-001;SV-002;SV-003;SV-006",
            "required_for_roles": "T1;T2;T3;M;X",
            "accepted_source_examples": "route number;route name;shield label",
            "normalization_rule": "Normalize display label but retain original source label.",
            "missing_field_behavior": "allow_internal_only_row_with_label_needed",
            "blocked_claims": BLOCKED,
        },
        {
            "field_name": "from_ref",
            "field_group": "topology",
            "required_for_vectors": "SV-001;SV-002;SV-003;SV-005",
            "required_for_roles": "T1;T2;T3;R",
            "accepted_source_examples": "begin node;begin milepost;jurisdiction boundary;named place",
            "normalization_rule": "Map to a stable endpoint reference before fitting corridor continuity.",
            "missing_field_behavior": "block_continuity_fit",
            "blocked_claims": BLOCKED,
        },
        {
            "field_name": "to_ref",
            "field_group": "topology",
            "required_for_vectors": "SV-001;SV-002;SV-003;SV-005",
            "required_for_roles": "T1;T2;T3;R",
            "accepted_source_examples": "end node;end milepost;jurisdiction boundary;named place",
            "normalization_rule": "Map to a stable endpoint reference before fitting corridor continuity.",
            "missing_field_behavior": "block_continuity_fit",
            "blocked_claims": BLOCKED,
        },
        {
            "field_name": "owner_or_jurisdiction",
            "field_group": "authority",
            "required_for_vectors": "SV-001;SV-002;SV-003;SV-004;SV-006",
            "required_for_roles": "T1;T2;T3;T4;M;X",
            "accepted_source_examples": "state DOT;turnpike authority;county;municipality;port or airport authority",
            "normalization_rule": "Separate owner from service role and do not infer approval from ownership.",
            "missing_field_behavior": "block_client_review_packet",
            "blocked_claims": BLOCKED,
        },
        {
            "field_name": "road_class",
            "field_group": "classification",
            "required_for_vectors": "SV-001;SV-002;SV-003;SV-006",
            "required_for_roles": "T1;T2;T3;M;X",
            "accepted_source_examples": "interstate;US highway;state highway;local access;functional class",
            "normalization_rule": "Map source classes to ROUTE display classes without promoting a role by class alone.",
            "missing_field_behavior": "route_to_manual_role_review",
            "blocked_claims": BLOCKED,
        },
        {
            "field_name": "priority_node_refs",
            "field_group": "market",
            "required_for_vectors": "SV-001;SV-003;SV-004",
            "required_for_roles": "T1;T3;T4",
            "accepted_source_examples": "cities;gateways;ports;airports;border crossings;hospitals;campuses;freight districts",
            "normalization_rule": "Link segment endpoints or access links to client-approved priority nodes.",
            "missing_field_behavior": "fit_as_source_needed_only",
            "blocked_claims": BLOCKED,
        },
        {
            "field_name": "parallel_or_alternate_refs",
            "field_group": "resilience",
            "required_for_vectors": "SV-002;SV-005",
            "required_for_roles": "T2;R",
            "accepted_source_examples": "detour route;parallel corridor;alternate state route;incident management route",
            "normalization_rule": "Record candidate alternate relationship and require review before scoring resilience.",
            "missing_field_behavior": "block_redundancy_claim",
            "blocked_claims": BLOCKED,
        },
        {
            "field_name": "restriction_refs",
            "field_group": "operations",
            "required_for_vectors": "SV-002;SV-003;SV-004;SV-005",
            "required_for_roles": "T2;T3;T4;R",
            "accepted_source_examples": "truck restriction;bridge limit;seasonal closure;work-zone constraint;local access constraint",
            "normalization_rule": "Treat restrictions as fit penalties or review gates until source evidence is accepted.",
            "missing_field_behavior": "allow_fit_with_restriction_gap",
            "blocked_claims": BLOCKED,
        },
        {
            "field_name": "observed_failure_refs",
            "field_group": "evidence",
            "required_for_vectors": "SV-002;SV-003;SV-004;SV-005",
            "required_for_roles": "T2;T3;T4;R",
            "accepted_source_examples": "closure log;incident record;bottleneck list;terminal delay record;emergency access record",
            "normalization_rule": "Map evidence to SSF metric ids and keep current_signal not-scored until reviewed.",
            "missing_field_behavior": "keep_evidence_posture_source_needed",
            "blocked_claims": BLOCKED,
        },
        {
            "field_name": "non_promotion_reason",
            "field_group": "coverage",
            "required_for_vectors": "SV-006",
            "required_for_roles": "M;X",
            "accepted_source_examples": "low service relevance;outside scope;maintained only;local-only function;duplicate segment",
            "normalization_rule": "Require an explicit reason when a segment is not promoted to T1/T2/T3/T4.",
            "missing_field_behavior": "block_full_coverage_claim",
            "blocked_claims": BLOCKED,
        },
    ]


def build_row_contract() -> list[dict[str, str]]:
    return [
        {
            "row_id": "SRC-ROW-001",
            "input_surface": "state_road_inventory_segment",
            "minimum_columns": "source_segment_id;route_label;from_ref;to_ref;owner_or_jurisdiction;road_class",
            "role_fit_dependency": "SV-001;SV-002;SV-003;SV-005;SV-006",
            "evidence_posture_on_ingest": "source-needed",
            "promotion_gate": "source row accepted plus client priority-node review plus failure metric mapping",
            "blocked_claims": BLOCKED,
        },
        {
            "row_id": "SRC-ROW-002",
            "input_surface": "priority_node_inventory",
            "minimum_columns": "node_id;node_label;node_class;jurisdiction;source_ref",
            "role_fit_dependency": "SV-001;SV-003;SV-004",
            "evidence_posture_on_ingest": "source-needed",
            "promotion_gate": "client confirms node classes and service promise relevance",
            "blocked_claims": BLOCKED,
        },
        {
            "row_id": "SRC-ROW-003",
            "input_surface": "terminal_access_inventory",
            "minimum_columns": "terminal_id;terminal_class;access_route_ref;nearest_tier_node;source_ref",
            "role_fit_dependency": "SV-004;SV-005",
            "evidence_posture_on_ingest": "source-needed",
            "promotion_gate": "terminal owner/access constraint evidence reviewed",
            "blocked_claims": BLOCKED,
        },
        {
            "row_id": "SRC-ROW-004",
            "input_surface": "restriction_and_failure_inventory",
            "minimum_columns": "restriction_id;segment_ref;restriction_type;failure_metric_ref;source_ref",
            "role_fit_dependency": "SV-002;SV-003;SV-004;SV-005",
            "evidence_posture_on_ingest": "source-needed",
            "promotion_gate": "restriction or failure evidence mapped to SSF metric and reviewed",
            "blocked_claims": BLOCKED,
        },
        {
            "row_id": "SRC-ROW-005",
            "input_surface": "non_promotion_inventory",
            "minimum_columns": "source_segment_id;coverage_status;non_promotion_reason;review_owner",
            "role_fit_dependency": "SV-006",
            "evidence_posture_on_ingest": "source-needed",
            "promotion_gate": "coverage audit confirms no service role is being sold for the segment",
            "blocked_claims": BLOCKED,
        },
    ]


def build_precheck() -> list[dict[str, str]]:
    return [
        {
            "precheck_id": "PRE-001",
            "adapter_step": "inventory_shape",
            "question": "Does the source inventory contain stable segment identity and endpoints?",
            "pass_condition": "source_segment_id from_ref and to_ref are present for every candidate road row",
            "hold_condition": "missing identity or endpoint fields blocks source-backed fit",
            "output_artifact": "state-source-inventory-adapter-field-contract-001.csv",
            "blocked_claims": BLOCKED,
        },
        {
            "precheck_id": "PRE-002",
            "adapter_step": "priority_node_shape",
            "question": "Does the client identify cities gateways terminals and access nodes?",
            "pass_condition": "priority node rows include node class jurisdiction and source reference",
            "hold_condition": "missing nodes keeps fitted roles heuristic-held or source-needed",
            "output_artifact": "state-source-inventory-adapter-row-contract-001.csv",
            "blocked_claims": BLOCKED,
        },
        {
            "precheck_id": "PRE-003",
            "adapter_step": "restriction_failure_shape",
            "question": "Can constraints and failures be mapped to SSF metrics?",
            "pass_condition": "restriction and failure rows include segment reference metric reference and source reference",
            "hold_condition": "missing evidence blocks failure or resilience promotion",
            "output_artifact": "state-source-inventory-adapter-precheck-001.csv",
            "blocked_claims": BLOCKED,
        },
        {
            "precheck_id": "PRE-004",
            "adapter_step": "coverage_non_promotion",
            "question": "Can every source segment receive either a role or a non-promotion reason?",
            "pass_condition": "each segment has candidate role or non_promotion_reason",
            "hold_condition": "missing non-promotion reasons block full-coverage claims",
            "output_artifact": "state-source-inventory-adapter-precheck-001.csv",
            "blocked_claims": BLOCKED,
        },
    ]


def write_csv(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def write_review(field_rows: list[dict[str, str]], row_rows: list[dict[str, str]], precheck_rows: list[dict[str, str]]) -> None:
    review = f"""---
name: State Source Inventory Adapter Contract 001
slug: state-source-inventory-adapter-contract-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-source-inventory-adapter-field-contract-001.csv
  - data/state-source-inventory-adapter-row-contract-001.csv
  - data/state-source-inventory-adapter-precheck-001.csv
  - data/state-tierization-fit-role-vector-profile-001.csv
  - docs/reviews/state-tierization-fit-kernel-001.md
---

# State Source Inventory Adapter Contract 001

## Scope

This contract defines the first generic bridge from a state road inventory into
the ROUTE full-state tierization fit kernel. It is designed for state DOT,
turnpike, port, airport, MPO, or consultant source packages.

## Contract Result

| Check | Result |
|---|---|
| Required source fields | {len(field_rows)} |
| Input row surfaces | {len(row_rows)} |
| Precheck gates | {len(precheck_rows)} |
| Initial ingest posture | source-needed |

## Product Use

The adapter tells a client what data ROUTE needs before it can apply the fitted
T1/T2/T3/T4/R/M/X roles to a real inventory. It also keeps every unsupported
role, SLA, ROI, construction, approval, validation, and full-inventory claim
held until the source package passes review.

## Gate

Decision: **state_source_inventory_adapter_contract_ready_for_client_payload**
"""
    REVIEW.write_text(review, encoding="utf-8")


def main() -> None:
    profile = read_profile()
    field_rows = build_field_contract(profile)
    row_rows = build_row_contract()
    precheck_rows = build_precheck()
    write_csv(FIELD_CONTRACT, FIELD_CONTRACT_FIELDS, field_rows)
    write_csv(ROW_CONTRACT, ROW_CONTRACT_FIELDS, row_rows)
    write_csv(PRECHECK, PRECHECK_FIELDS, precheck_rows)
    write_review(field_rows, row_rows, precheck_rows)
    print(f"wrote {FIELD_CONTRACT}")
    print(f"wrote {ROW_CONTRACT}")
    print(f"wrote {PRECHECK}")
    print(f"wrote {REVIEW}")


if __name__ == "__main__":
    main()
