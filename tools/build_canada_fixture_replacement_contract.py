#!/usr/bin/env python3
"""Build Canada fixture replacement contract after source-row and geometry gates."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-canada-fixture-replacement-contract-001.csv"

FIELDS = [
    "contract_id",
    "candidate_table",
    "replacement_target",
    "geometry_contract",
    "contract_decision",
    "allowed_use",
    "required_inputs",
    "blocked_uses",
    "blocked_claims",
    "next_action",
]

BLOCKED_CLAIMS = (
    "geometry_acceptance;topology_proof;map_overlay;parsed_adapter;"
    "official_network;route_designation;engineering_precision;agency_approval;"
    "construction_ready;guaranteed_sla;roi;eligibility;compliance;endorsement;"
    "validation;public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "contract_id": "CAN-REPLACE-CONTRACT-001",
            "candidate_table": "data/international-canada-parser-extraction-candidates-001.csv",
            "replacement_target": "data/canada_source_link_candidates.csv",
            "geometry_contract": "no_geometry_candidate_rows_allowed",
            "contract_decision": "replacement_contract_ready_for_internal_link_fixture_closeout",
            "allowed_use": "internal parser link-candidate fixture rows only",
            "required_inputs": "source-row validation pass; geometry policy pass; replacement role review pass_with_holds; evidence labels and blocked claims carried forward",
            "blocked_uses": "map overlay; topology proof; parsed adapter promotion; external use; official or operational claims",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "write fixture replacement closeout and update generator only if dry-run link target ownership changes",
        },
        {
            "contract_id": "CAN-REPLACE-CONTRACT-002",
            "candidate_table": "data/international-canada-parser-extraction-candidates-001.csv",
            "replacement_target": "maps/international or adapter outputs",
            "geometry_contract": "geometry_required_before_map_or_adapter_use",
            "contract_decision": "replacement_not_allowed_for_map_or_adapter_surfaces",
            "allowed_use": "none",
            "required_inputs": "separate geometry intake fixture with topology QA and role review",
            "blocked_uses": "map overlay; topology proof; parsed adapter promotion; public map; service proof",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "open geometry intake only if map or adapter promotion becomes the selected work",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
