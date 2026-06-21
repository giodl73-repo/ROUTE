#!/usr/bin/env python3
"""Build a bounded Iowa state-highway-system pilot ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "state-highway-system-pilot-iowa-001.csv"

FIELDS = [
    "pilot_id",
    "state",
    "surface",
    "input_artifacts",
    "candidate_network_role",
    "state_use_case",
    "evidence_status",
    "review_roles",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "official_state_plan;state_dot_endorsement;fhwa_approval;route_designation;"
    "construction_ready;funding_commitment;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;benefit_cost_ratio;eligibility;"
    "compliance;environmental_clearance;right_of_way_clearance;"
    "maintenance_commitment;source_row_validation;geometry_acceptance;"
    "topology_proof;map_overlay;throughput_proof;validation;"
    "external_validation;public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "pilot_id": "IA-STATE-PILOT-001",
            "state": "Iowa",
            "surface": "state_value_intake",
            "input_artifacts": "docs/briefs/state-value-brief.md;docs/how-to/round2-state-intake-payload.md",
            "candidate_network_role": "state_priorities_to_service_roles",
            "state_use_case": "translate state priorities into T1/T2/T3/T4 planning questions before corridor commitments",
            "evidence_status": "intake_ready_claims_held",
            "review_roles": "State DOT Planner;Scope Keeper;Citation Auditor;Numeracy Checker",
            "blocked_claims": BLOCKED,
            "next_action": "fill state-specific source pack before any official corridor or funding claim",
        },
        {
            "pilot_id": "IA-STATE-PILOT-002",
            "state": "Iowa",
            "surface": "des_moines_i35_i80_operating_context",
            "input_artifacts": "docs/evidence-campaigns/milepost-9-iowa-repeat-window.md;data/t1-failure-events.csv",
            "candidate_network_role": "urban_interstate_diamond_resilience_stressor",
            "state_use_case": "treat Des Moines I-35/I-80 evidence as a repeat-window candidate for reliability and resilience review",
            "evidence_status": "snapshot_window_candidate_promotion_blocked",
            "review_roles": "Traffic Engineer;State DOT Planner;V&V;Citation Auditor",
            "blocked_claims": BLOCKED,
            "next_action": "extend repeated observation window before reliability or SLA inference",
        },
        {
            "pilot_id": "IA-STATE-PILOT-003",
            "state": "Iowa",
            "surface": "statewide_trunk_and_connector_hypothesis",
            "input_artifacts": "data/beck-stop-sla.csv;docs/reports/interstate-2-0-doctrine-report.md",
            "candidate_network_role": "interstate_spine_and_regional_connector_candidate",
            "state_use_case": "infer where Iowa routes act as national spine, regional connector, rural access, or terminal-access candidates",
            "evidence_status": "heuristic_planning_only",
            "review_roles": "Schematic Cartographer;State DOT Planner;Freight Economist;Scope Keeper",
            "blocked_claims": BLOCKED,
            "next_action": "join Iowa DOT roadway inventory and freight-node sources before state hierarchy acceptance",
        },
        {
            "pilot_id": "IA-STATE-PILOT-004",
            "state": "Iowa",
            "surface": "scenario_game_fixture",
            "input_artifacts": "data/game/des-moines-diamond-state-fixture.json;data/game/des-moines-diamond-session-fixture.csv",
            "candidate_network_role": "state_meeting_scenario_rehearsal",
            "state_use_case": "simulate tradeoffs among work-zone sequencing, relay staffing, budget, patience, capacity, and evidence confidence",
            "evidence_status": "scenario_fixture_publication_locked",
            "review_roles": "State DOT Planner;Traffic Engineer;Scope Keeper;V&V",
            "blocked_claims": BLOCKED,
            "next_action": "replace scenario fixture assumptions with state source rows before decision support use",
        },
        {
            "pilot_id": "IA-STATE-PILOT-005",
            "state": "Iowa",
            "surface": "state_review_packet",
            "input_artifacts": "docs/briefs/state-to-aashto-regional-packet.md;docs/reports/route-evidence-posture.md",
            "candidate_network_role": "state_to_regional_review_packet",
            "state_use_case": "prepare a state-to-regional review packet that separates analytical merit from agency authority and delivery constraints",
            "evidence_status": "packet_scaffold_ready_external_review_held",
            "review_roles": "State DOT Planner;Scope Keeper;Citation Auditor;Schematic Cartographer;V&V",
            "blocked_claims": BLOCKED,
            "next_action": "name exact Iowa source rows and dissent questions before any state or AASHTO-style rehearsal",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
