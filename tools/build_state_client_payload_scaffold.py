#!/usr/bin/env python3
"""Build generic client payload scaffold for state source inventory intake."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
REVIEW = ROOT / "docs" / "reviews" / "state-client-payload-scaffold-001.md"

SEGMENTS = DATA / "state-client-payload-segment-template-001.csv"
NODES = DATA / "state-client-payload-priority-node-template-001.csv"
TERMINALS = DATA / "state-client-payload-terminal-access-template-001.csv"
FAILURES = DATA / "state-client-payload-restriction-failure-template-001.csv"
NON_PROMOTION = DATA / "state-client-payload-non-promotion-template-001.csv"
MANIFEST = DATA / "state-client-payload-manifest-001.csv"
PREFLIGHT = DATA / "state-client-payload-preflight-001.csv"

BLOCKED = (
    "official_designation;legal_sla;construction;cost;numeric_roi;roi;"
    "eligibility;compliance;endorsement;validation;public_readiness;"
    "state_approval;source_backed_full_inventory"
)


def write_csv(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def build_segments() -> None:
    fields = [
        "source_segment_id",
        "route_label",
        "from_ref",
        "to_ref",
        "owner_or_jurisdiction",
        "road_class",
        "priority_node_refs",
        "parallel_or_alternate_refs",
        "restriction_refs",
        "observed_failure_refs",
        "source_ref",
        "client_notes",
    ]
    rows = [
        {
            "source_segment_id": "state-segment-001",
            "route_label": "I-00",
            "from_ref": "city-a",
            "to_ref": "city-b",
            "owner_or_jurisdiction": "state-dot",
            "road_class": "interstate",
            "priority_node_refs": "node-city-a|node-city-b",
            "parallel_or_alternate_refs": "state-segment-002",
            "restriction_refs": "restriction-001",
            "observed_failure_refs": "failure-001",
            "source_ref": "client-source-road-inventory",
            "client_notes": "example statewide trunk row",
        },
        {
            "source_segment_id": "state-segment-002",
            "route_label": "SR-00",
            "from_ref": "city-b",
            "to_ref": "regional-market",
            "owner_or_jurisdiction": "state-dot",
            "road_class": "state-highway",
            "priority_node_refs": "node-city-b|node-regional-market",
            "parallel_or_alternate_refs": "state-segment-001",
            "restriction_refs": "",
            "observed_failure_refs": "",
            "source_ref": "client-source-road-inventory",
            "client_notes": "example regional connector row",
        },
        {
            "source_segment_id": "state-segment-003",
            "route_label": "LOCAL-ACCESS",
            "from_ref": "terminal-a",
            "to_ref": "city-a",
            "owner_or_jurisdiction": "local-or-terminal",
            "road_class": "terminal-access",
            "priority_node_refs": "node-terminal-a|node-city-a",
            "parallel_or_alternate_refs": "",
            "restriction_refs": "restriction-002",
            "observed_failure_refs": "restriction-002",
            "source_ref": "client-source-road-inventory",
            "client_notes": "example terminal access row",
        },
        {
            "source_segment_id": "state-segment-099",
            "route_label": "SR-99",
            "from_ref": "local-a",
            "to_ref": "local-b",
            "owner_or_jurisdiction": "state-dot",
            "road_class": "state-highway",
            "priority_node_refs": "",
            "parallel_or_alternate_refs": "",
            "restriction_refs": "",
            "observed_failure_refs": "",
            "source_ref": "client-source-road-inventory",
            "client_notes": "example maintained non-promotion row",
        },
    ]
    write_csv(SEGMENTS, fields, rows)


def build_nodes() -> None:
    fields = ["node_id", "node_label", "node_class", "jurisdiction", "source_ref", "client_priority"]
    rows = [
        {
            "node_id": "node-city-a",
            "node_label": "City A",
            "node_class": "city",
            "jurisdiction": "example-state",
            "source_ref": "client-priority-node-list",
            "client_priority": "statewide",
        },
        {
            "node_id": "node-terminal-a",
            "node_label": "Terminal A",
            "node_class": "port_or_airport_or_freight_terminal",
            "jurisdiction": "example-state",
            "source_ref": "client-priority-node-list",
            "client_priority": "terminal-access",
        },
        {
            "node_id": "node-city-b",
            "node_label": "City B",
            "node_class": "city",
            "jurisdiction": "example-state",
            "source_ref": "client-priority-node-list",
            "client_priority": "statewide",
        },
        {
            "node_id": "node-regional-market",
            "node_label": "Regional Market",
            "node_class": "regional-market",
            "jurisdiction": "example-state",
            "source_ref": "client-priority-node-list",
            "client_priority": "regional",
        },
    ]
    write_csv(NODES, fields, rows)


def build_terminals() -> None:
    fields = [
        "terminal_id",
        "terminal_class",
        "access_route_ref",
        "nearest_tier_node",
        "source_ref",
        "known_access_constraint_refs",
    ]
    rows = [
        {
            "terminal_id": "terminal-a",
            "terminal_class": "port_or_airport_or_border_or_industrial",
            "access_route_ref": "state-segment-003",
            "nearest_tier_node": "node-city-a",
            "source_ref": "client-terminal-inventory",
            "known_access_constraint_refs": "restriction-002",
        }
    ]
    write_csv(TERMINALS, fields, rows)


def build_failures() -> None:
    fields = [
        "restriction_id",
        "segment_ref",
        "restriction_type",
        "failure_metric_ref",
        "source_ref",
        "description",
        "review_status",
    ]
    rows = [
        {
            "restriction_id": "restriction-001",
            "segment_ref": "state-segment-001",
            "restriction_type": "incident_or_closure_or_bottleneck",
            "failure_metric_ref": "SSF-002",
            "source_ref": "client-operations-log",
            "description": "example alternate-route penalty evidence row",
            "review_status": "source-needed",
        },
        {
            "restriction_id": "restriction-002",
            "segment_ref": "state-segment-003",
            "restriction_type": "terminal_access_constraint",
            "failure_metric_ref": "SSF-005",
            "source_ref": "client-terminal-access-log",
            "description": "example terminal access friction evidence row",
            "review_status": "source-needed",
        },
    ]
    write_csv(FAILURES, fields, rows)


def build_non_promotion() -> None:
    fields = ["source_segment_id", "coverage_status", "non_promotion_reason", "review_owner", "source_ref"]
    rows = [
        {
            "source_segment_id": "state-segment-099",
            "coverage_status": "maintained_not_promoted",
            "non_promotion_reason": "maintained route with no promoted service role in current package",
            "review_owner": "client-review-owner",
            "source_ref": "client-road-inventory",
        }
    ]
    write_csv(NON_PROMOTION, fields, rows)


def build_manifest() -> None:
    fields = [
        "payload_id",
        "template_path",
        "input_surface",
        "minimum_contract_source",
        "required_for_preflight",
        "initial_evidence_posture",
        "blocked_claims",
    ]
    rows = [
        {
            "payload_id": "PAYLOAD-SEGMENTS",
            "template_path": "data/state-client-payload-segment-template-001.csv",
            "input_surface": "state_road_inventory_segment",
            "minimum_contract_source": "SRC-ROW-001",
            "required_for_preflight": "yes",
            "initial_evidence_posture": "source-needed",
            "blocked_claims": BLOCKED,
        },
        {
            "payload_id": "PAYLOAD-NODES",
            "template_path": "data/state-client-payload-priority-node-template-001.csv",
            "input_surface": "priority_node_inventory",
            "minimum_contract_source": "SRC-ROW-002",
            "required_for_preflight": "yes",
            "initial_evidence_posture": "source-needed",
            "blocked_claims": BLOCKED,
        },
        {
            "payload_id": "PAYLOAD-TERMINALS",
            "template_path": "data/state-client-payload-terminal-access-template-001.csv",
            "input_surface": "terminal_access_inventory",
            "minimum_contract_source": "SRC-ROW-003",
            "required_for_preflight": "yes",
            "initial_evidence_posture": "source-needed",
            "blocked_claims": BLOCKED,
        },
        {
            "payload_id": "PAYLOAD-FAILURES",
            "template_path": "data/state-client-payload-restriction-failure-template-001.csv",
            "input_surface": "restriction_and_failure_inventory",
            "minimum_contract_source": "SRC-ROW-004",
            "required_for_preflight": "yes",
            "initial_evidence_posture": "source-needed",
            "blocked_claims": BLOCKED,
        },
        {
            "payload_id": "PAYLOAD-NON-PROMOTION",
            "template_path": "data/state-client-payload-non-promotion-template-001.csv",
            "input_surface": "non_promotion_inventory",
            "minimum_contract_source": "SRC-ROW-005",
            "required_for_preflight": "yes",
            "initial_evidence_posture": "source-needed",
            "blocked_claims": BLOCKED,
        },
    ]
    write_csv(MANIFEST, fields, rows)


def build_preflight() -> None:
    fields = [
        "preflight_id",
        "payload_id",
        "template_path",
        "contract_check",
        "template_status",
        "client_data_status",
        "next_action",
        "blocked_claims",
    ]
    rows = [
        {
            "preflight_id": "CLIENT-PRE-001",
            "payload_id": "PAYLOAD-SEGMENTS",
            "template_path": "data/state-client-payload-segment-template-001.csv",
            "contract_check": "minimum columns present for SRC-ROW-001",
            "template_status": "pass",
            "client_data_status": "not-provided",
            "next_action": "client fills source segment inventory and source references",
            "blocked_claims": BLOCKED,
        },
        {
            "preflight_id": "CLIENT-PRE-002",
            "payload_id": "PAYLOAD-NODES",
            "template_path": "data/state-client-payload-priority-node-template-001.csv",
            "contract_check": "minimum columns present for SRC-ROW-002",
            "template_status": "pass",
            "client_data_status": "not-provided",
            "next_action": "client confirms priority node classes and jurisdictions",
            "blocked_claims": BLOCKED,
        },
        {
            "preflight_id": "CLIENT-PRE-003",
            "payload_id": "PAYLOAD-TERMINALS",
            "template_path": "data/state-client-payload-terminal-access-template-001.csv",
            "contract_check": "minimum columns present for SRC-ROW-003",
            "template_status": "pass",
            "client_data_status": "not-provided",
            "next_action": "client supplies terminal inventory or marks surface not applicable",
            "blocked_claims": BLOCKED,
        },
        {
            "preflight_id": "CLIENT-PRE-004",
            "payload_id": "PAYLOAD-FAILURES",
            "template_path": "data/state-client-payload-restriction-failure-template-001.csv",
            "contract_check": "minimum columns present for SRC-ROW-004",
            "template_status": "pass",
            "client_data_status": "not-provided",
            "next_action": "client maps restrictions and failures to source references",
            "blocked_claims": BLOCKED,
        },
        {
            "preflight_id": "CLIENT-PRE-005",
            "payload_id": "PAYLOAD-NON-PROMOTION",
            "template_path": "data/state-client-payload-non-promotion-template-001.csv",
            "contract_check": "minimum columns present for SRC-ROW-005",
            "template_status": "pass",
            "client_data_status": "not-provided",
            "next_action": "client records non-promotion reasons for unpromoted inventory rows",
            "blocked_claims": BLOCKED,
        },
    ]
    write_csv(PREFLIGHT, fields, rows)


def write_review() -> None:
    review = f"""---
name: State Client Payload Scaffold 001
slug: state-client-payload-scaffold-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-client-payload-manifest-001.csv
  - data/state-client-payload-preflight-001.csv
  - data/state-client-payload-segment-template-001.csv
  - data/state-client-payload-priority-node-template-001.csv
  - data/state-client-payload-terminal-access-template-001.csv
  - data/state-client-payload-restriction-failure-template-001.csv
  - data/state-client-payload-non-promotion-template-001.csv
  - data/state-source-inventory-adapter-row-contract-001.csv
---

# State Client Payload Scaffold 001

## Scope

This scaffold turns the generic state source-inventory adapter contract into
client-fillable payload templates. It gives a state or infrastructure operator a
specific package for road segments, priority nodes, terminal access, restrictions
and failures, and non-promotion coverage.

## Scaffold Result

| Check | Result |
|---|---|
| Payload templates | 5 |
| Manifest rows | 5 |
| Template preflight rows | 5 |
| Segment sample rows | 4 |
| Priority node sample rows | 4 |
| Client data status | not-provided |

## Evidence Boundary

The scaffold validates template shape only. It does not validate client data,
source custody, official designations, legal SLAs, construction readiness, cost,
numeric ROI, funding eligibility, compliance, endorsement, public readiness,
state approval, or source-backed full inventory.

## Gate

Decision: **state_client_payload_scaffold_ready_for_first_client_fill**
"""
    REVIEW.write_text(review, encoding="utf-8")


def main() -> None:
    build_segments()
    build_nodes()
    build_terminals()
    build_failures()
    build_non_promotion()
    build_manifest()
    build_preflight()
    write_review()
    for path in [SEGMENTS, NODES, TERMINALS, FAILURES, NON_PROMOTION, MANIFEST, PREFLIGHT, REVIEW]:
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
