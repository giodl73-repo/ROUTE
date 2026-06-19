#!/usr/bin/env python3
"""Build Canada geometry policy for source-derived candidates."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-canada-geometry-policy-001.csv"

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
    "parsed_adapter;official_network;route_designation;engineering_precision;"
    "agency_approval;construction_ready;guaranteed_sla;roi;eligibility;"
    "compliance;endorsement;validation;public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "policy_id": "CAN-GEOM-POLICY-001",
            "surface": "parser extraction candidates",
            "current_geometry_status": "not_requested",
            "decision": "reject_geometry_for_current_candidate_set",
            "required_before_acceptance": "bounded geometry fetch; coordinate reference system recorded; source license/access note; row-level geometry join; topology QA; Schematic Cartographer review; Traffic Engineer no-operational-claim review",
            "blocked_uses": "map overlay; topology proof; fixture replacement; adapter promotion; operating claim",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "keep geometry_ref as not_requested until a separate geometry intake fixture closes",
        },
        {
            "policy_id": "CAN-GEOM-POLICY-002",
            "surface": "media and map references",
            "current_geometry_status": "not_requested",
            "decision": "caption_as_no_geometry_candidate_only",
            "required_before_acceptance": "map caption pattern; explicit no-proof label; accepted geometry ledger; role review",
            "blocked_uses": "public map; route proof; service proof; official network depiction",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "refer media users to source-row validation and replacement-role review, not maps",
        },
        {
            "policy_id": "CAN-GEOM-POLICY-003",
            "surface": "fixture replacement closeout",
            "current_geometry_status": "not_requested",
            "decision": "hold_replacement_until_geometry_or_no_geometry_contract_selected",
            "required_before_acceptance": "replacement contract states whether link candidates may remain non-geometric; if geometry is used, close geometry intake first",
            "blocked_uses": "dry-run fixture replacement; parsed adapter promotion",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "write replacement closeout only after geometry/no-geometry contract is explicit",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
