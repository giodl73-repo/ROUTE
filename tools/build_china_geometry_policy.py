#!/usr/bin/env python3
"""Build China geometry policy for dry-run candidates."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-china-geometry-policy-001.csv"

FIELDS = [
    "policy_id",
    "surface",
    "current_geometry_status",
    "decision",
    "required_before_acceptance",
    "blocked_uses",
    "blocked_claims",
    "next_action",
]

BLOCKED_CLAIMS = (
    "geometry_acceptance;topology_proof;map_overlay;fixture_replacement;"
    "parsed_adapter;official_network;official_corridor_designation;"
    "policy_alignment;route_designation;engineering_precision;"
    "terminal_performance;road_access_proof;throughput_proof;"
    "construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;"
    "endorsement;validation;external_validation;public_readiness;"
    "external_readiness;internal_adapter_proof"
)


def main() -> None:
    rows = [
        {
            "policy_id": "CHN-GEOM-POLICY-001",
            "surface": "parser dry-run link and node candidates",
            "current_geometry_status": "not_requested",
            "decision": "reject_geometry_for_current_candidate_set",
            "required_before_acceptance": "bounded geometry source selected; source license/access note; coordinate reference system recorded; row-level geometry join; topology QA; Schematic Cartographer review; Traffic Engineer no-operational-claim review",
            "blocked_uses": "map overlay; topology proof; fixture replacement; parsed adapter promotion; operating claim; terminal-performance claim",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep geometry_ref as not_accepted until a separate China geometry intake fixture closes",
        },
        {
            "policy_id": "CHN-GEOM-POLICY-002",
            "surface": "media and schematic map references",
            "current_geometry_status": "not_requested",
            "decision": "caption_as_no_geometry_candidate_only",
            "required_before_acceptance": "map caption pattern; explicit no-proof label; accepted geometry ledger; policy-alignment claim review; role review",
            "blocked_uses": "public map; route proof; service proof; official network depiction; policy-alignment proof; fixture replacement",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "refer media users to dry-run role review and fixture blocker, not maps",
        },
        {
            "policy_id": "CHN-GEOM-POLICY-003",
            "surface": "fixture replacement closeout",
            "current_geometry_status": "not_requested",
            "decision": "hold_replacement_until_geometry_or_no_geometry_contract_selected",
            "required_before_acceptance": "replacement contract states whether China link and node candidates may remain non-geometric; if geometry is used, close geometry intake first",
            "blocked_uses": "dry-run fixture replacement; parsed adapter promotion; internal adapter proof",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep fixture blocker active before any replacement contract",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
