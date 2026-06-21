#!/usr/bin/env python3
"""Build Iowa state-highway source-pack preflight ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "state-highway-iowa-source-pack-001.csv"

FIELDS = [
    "source_family",
    "source_id",
    "source_path_or_status",
    "owner_or_publisher",
    "date_accessed",
    "required_fields",
    "adapter_target",
    "promotion_decision",
    "claim_boundary",
    "next_action",
]

BLOCKED = (
    "no official state plan state DOT endorsement FHWA approval route "
    "designation source-row validation geometry acceptance topology proof map "
    "overlay construction-ready funding commitment guaranteed SLA travel-time "
    "proof delivery commitment numeric ROI ROI benefit-cost ratio eligibility "
    "compliance environmental clearance right-of-way clearance maintenance "
    "commitment throughput proof validation public-readiness external-readiness "
    "or external validation claim"
)


def main() -> None:
    rows = [
        {
            "source_family": "state_roadway_inventory",
            "source_id": "IA-SRC-001",
            "source_path_or_status": "source-needed:Iowa DOT roadway inventory or public roadway GIS/service",
            "owner_or_publisher": "Iowa Department of Transportation",
            "date_accessed": "2026-06-21",
            "required_fields": "route identifier; jurisdiction; functional class; geometry availability status; access note",
            "adapter_target": "state_source_link_candidates;state_hierarchy_candidates",
            "promotion_decision": "source-needed not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "select exact Iowa DOT roadway inventory source before source-row validation or map use",
        },
        {
            "source_family": "state_freight_and_economic_context",
            "source_id": "IA-SRC-002",
            "source_path_or_status": "source-needed:Iowa freight plan, state freight network, or commerce/logistics source",
            "owner_or_publisher": "Iowa DOT or state freight/economic-development publisher",
            "date_accessed": "2026-06-21",
            "required_fields": "freight corridor or node context; publication date; source owner; access note",
            "adapter_target": "state_need_candidates;freight_node_context",
            "promotion_decision": "source-needed not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "select exact freight corridor and node sources before freight priority or service-role claims",
        },
        {
            "source_family": "iowa_511_operating_events",
            "source_id": "IA-SRC-003",
            "source_path_or_status": "docs/evidence-campaigns/milepost-9-iowa-repeat-window.md;data/t1-failure-events.csv",
            "owner_or_publisher": "Iowa DOT 511 source path via ROUTE snapshot workflow",
            "date_accessed": "2026-06-21",
            "required_fields": "event id; route context; capture timestamp; event timing status; observation-window status",
            "adapter_target": "state_reliability_evidence_window;resilience_stressor_candidates",
            "promotion_decision": "snapshot-window candidate not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "extend repeated observation window before reliability, recovery, or SLA inference",
        },
        {
            "source_family": "state_program_and_delivery_context",
            "source_id": "IA-SRC-004",
            "source_path_or_status": "source-needed:Iowa STIP/TIP/LRTP/TAMP or project-development source",
            "owner_or_publisher": "Iowa DOT, MPO, or state planning publisher",
            "date_accessed": "2026-06-21",
            "required_fields": "program name; project or asset context; funding window; delivery constraint; access note",
            "adapter_target": "delivery_constraint_ledger;state_review_packet",
            "promotion_decision": "source-needed not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "select exact state program rows before delivery, funding, maintenance, or construction claims",
        },
        {
            "source_family": "state_asset_and_maintenance_context",
            "source_id": "IA-SRC-005",
            "source_path_or_status": "source-needed:state pavement, bridge, winter operations, or maintenance source",
            "owner_or_publisher": "Iowa DOT or asset-management publisher",
            "date_accessed": "2026-06-21",
            "required_fields": "asset class; condition/status field; route or facility reference; maintenance constraint; access note",
            "adapter_target": "asset_condition_context;maintenance_constraint_ledger",
            "promotion_decision": "source-needed not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "select exact asset and maintenance rows before readiness or lifecycle claims",
        },
        {
            "source_family": "des_moines_scenario_fixture",
            "source_id": "IA-SRC-006",
            "source_path_or_status": "data/game/des-moines-diamond-state-fixture.json;data/game/des-moines-diamond-session-fixture.csv",
            "owner_or_publisher": "ROUTE internal scenario fixture",
            "date_accessed": "2026-06-21",
            "required_fields": "scenario option; budget; public patience; operating capacity; publication gate",
            "adapter_target": "scenario_review_fixture;state_meeting_rehearsal",
            "promotion_decision": "internal fixture not promoted",
            "claim_boundary": BLOCKED,
            "next_action": "replace scenario assumptions with Iowa source rows before decision-support or publication use",
        },
        {
            "source_family": "state_service_targets",
            "source_id": "IA-SRC-SLA-001",
            "source_path_or_status": "none",
            "owner_or_publisher": "none",
            "date_accessed": "2026-06-21",
            "required_fields": "target id; target class; assumption label; local basis; numeracy review",
            "adapter_target": "state_service_target_set",
            "promotion_decision": "held",
            "claim_boundary": BLOCKED,
            "next_action": "keep Iowa service targets assumption-labeled until state evidence and numeracy review close",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
