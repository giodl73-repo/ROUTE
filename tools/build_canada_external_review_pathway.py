#!/usr/bin/env python3
"""Build Canada external review pathway ledger with explicit holds."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-canada-external-review-pathway-001.csv"

FIELDS = [
    "pathway_id",
    "review_lane",
    "candidate_reviewer",
    "packet_focus",
    "input_artifacts",
    "required_roles",
    "safe_ask",
    "status",
    "allowed_language",
    "blocked_claims",
    "next_action",
]

INPUT_ARTIFACTS = (
    "docs/reviews/international-canada-internal-adapter-proof-001.md;"
    "docs/media/canada-internal-proof-brief.md;"
    "docs/how-to/external-rehearsal-packet-selection-runbook.md"
)

BLOCKED_CLAIMS = (
    "official_network;route_designation;geometry_acceptance;topology_proof;"
    "map_overlay;agency_approval;provincial_approval;port_endorsement;"
    "terminal_performance;node_completeness;road_access_proof;throughput_proof;"
    "construction_ready;guaranteed_sla;travel_time_proof;delivery_commitment;"
    "numeric_roi;roi;eligibility;compliance;endorsement;validation;"
    "external_validation;public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "pathway_id": "CAN-EXT-REVIEW-001",
            "review_lane": "federal_transport",
            "candidate_reviewer": "Transport Canada technical or policy review lane",
            "packet_focus": "road-system vocabulary, trade-corridor framing, source-custody boundaries, and non-official-network language",
            "input_artifacts": INPUT_ARTIFACTS,
            "required_roles": "Scope Keeper;Citation Auditor;State DOT Planner;Schematic Cartographer;V&V",
            "safe_ask": "request a source-custody and terminology review path",
            "status": "candidate_lane_not_contacted",
            "allowed_language": "Canada has a scoped pathway for future federal transport review.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "select a named venue or office before filling any external packet",
        },
        {
            "pathway_id": "CAN-EXT-REVIEW-002",
            "review_lane": "port_authority",
            "candidate_reviewer": "Vancouver, Montreal, or Halifax port authority technical contact lane",
            "packet_focus": "port-node source custody, terminal vocabulary, and node-performance non-claims",
            "input_artifacts": INPUT_ARTIFACTS,
            "required_roles": "Scope Keeper;Citation Auditor;Freight Industry;Schematic Cartographer;V&V",
            "safe_ask": "request a node-source custody review path",
            "status": "candidate_lane_not_contacted",
            "allowed_language": "Canada port nodes have candidate source-custody rows for future port review.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "choose one named port contact lane and keep performance claims held",
        },
        {
            "pathway_id": "CAN-EXT-REVIEW-003",
            "review_lane": "provincial_or_regional_transport",
            "candidate_reviewer": "Provincial or regional transport planning review lane",
            "packet_focus": "road classification, local context, maintenance authority, and map-readiness holds",
            "input_artifacts": INPUT_ARTIFACTS,
            "required_roles": "Scope Keeper;Citation Auditor;State DOT Planner;Traffic Engineer;Schematic Cartographer",
            "safe_ask": "request a local-source and authority-boundary review path",
            "status": "candidate_lane_not_contacted",
            "allowed_language": "Canada has a scoped pathway for future provincial or regional transport review.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "select one jurisdiction and replace generic rows with local source-custody rows",
        },
        {
            "pathway_id": "CAN-EXT-REVIEW-004",
            "review_lane": "academic_or_transport_research",
            "candidate_reviewer": "Canadian transport research, logistics, or planning methodology reviewer lane",
            "packet_focus": "methodology reproducibility, parser contract, held assumptions, and evidence labels",
            "input_artifacts": INPUT_ARTIFACTS,
            "required_roles": "Scope Keeper;Citation Auditor;Numeracy Checker;Optimization Methodologist;V&V",
            "safe_ask": "request methodology critique without adoption or policy claims",
            "status": "candidate_lane_not_contacted",
            "allowed_language": "Canada has a scoped pathway for future methodology review.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "prepare a narrow method packet after a reviewer and artifact subset are named",
        },
        {
            "pathway_id": "CAN-EXT-REVIEW-005",
            "review_lane": "external_validation_decision",
            "candidate_reviewer": "not selected",
            "packet_focus": "decision gate between internal proof and any future external validation claim",
            "input_artifacts": INPUT_ARTIFACTS,
            "required_roles": "Scope Keeper;Citation Auditor;Numeracy Checker;Schematic Cartographer;V&V",
            "safe_ask": "hold external validation until a named venue, packet, role review, and validation closeout exist",
            "status": "external_validation_not_started",
            "allowed_language": "External validation for Canada has not started.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "do not claim validation until a named external review record closes with explicit scope",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
