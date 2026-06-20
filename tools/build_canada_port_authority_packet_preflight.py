#!/usr/bin/env python3
"""Build Canada port authority packet preflight ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-canada-port-authority-packet-preflight-001.csv"

FIELDS = [
    "packet_id",
    "section",
    "packet_entry",
    "source_anchor",
    "required_role",
    "status",
    "allowed_language",
    "blocked_claims",
    "next_action",
]

BLOCKED_CLAIMS = (
    "port_endorsement;port_review;agency_approval;external_validation;"
    "official_network;route_designation;terminal_performance;node_completeness;"
    "road_access_proof;throughput_proof;geometry_acceptance;topology_proof;"
    "map_overlay;construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;endorsement;"
    "validation;public_readiness;external_readiness"
)


def main() -> None:
    rows = [
        {
            "packet_id": "CAN-PORT-PREFLIGHT-001",
            "section": "metadata",
            "packet_entry": "Canada port authority source-custody packet preflight",
            "source_anchor": "data/international-canada-external-review-pathway-001.csv#CAN-EXT-REVIEW-002",
            "required_role": "Scope Keeper",
            "status": "preflight_only_no_named_venue",
            "allowed_language": "A Canada port-authority packet preflight exists for future source-custody review.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "select a named port authority venue or contact before copying the external packet template",
        },
        {
            "packet_id": "CAN-PORT-PREFLIGHT-002",
            "section": "source_custody",
            "packet_entry": "Port of Vancouver node custody row",
            "source_anchor": "data/canada_source_node_candidates.csv#CAN-PORT-VANCOUVER",
            "required_role": "Citation Auditor",
            "status": "source_candidate_internal_only",
            "allowed_language": "The Vancouver port node has a selected public source-custody candidate.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "verify source owner, access note, and review scope with a named venue before external use",
        },
        {
            "packet_id": "CAN-PORT-PREFLIGHT-003",
            "section": "source_custody",
            "packet_entry": "Port of Montreal node custody row",
            "source_anchor": "data/canada_source_node_candidates.csv#CAN-PORT-MONTREAL",
            "required_role": "Citation Auditor",
            "status": "source_candidate_internal_only",
            "allowed_language": "The Montreal port node has a selected public source-custody candidate.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "verify source owner, access note, and review scope with a named venue before external use",
        },
        {
            "packet_id": "CAN-PORT-PREFLIGHT-004",
            "section": "source_custody",
            "packet_entry": "Port of Halifax node custody row",
            "source_anchor": "data/canada_source_node_candidates.csv#CAN-PORT-HALIFAX",
            "required_role": "Citation Auditor",
            "status": "source_candidate_internal_only",
            "allowed_language": "The Halifax port node has a selected public source-custody candidate.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "verify source owner, access note, and review scope with a named venue before external use",
        },
        {
            "packet_id": "CAN-PORT-PREFLIGHT-005",
            "section": "materials",
            "packet_entry": "Canada internal proof brief, node fixture closeout, media proof brief, and external review pathway",
            "source_anchor": "docs/reviews/international-canada-internal-adapter-proof-001.md;docs/reviews/international-canada-node-fixture-replacement-closeout-001.md;docs/media/canada-internal-proof-brief.md;docs/reviews/international-canada-external-review-pathway-001.md",
            "required_role": "Scope Keeper",
            "status": "material_set_selected_for_preflight",
            "allowed_language": "The preflight material set is narrow and node-custody focused.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "remove any material not needed by the named port venue",
        },
        {
            "packet_id": "CAN-PORT-PREFLIGHT-006",
            "section": "role_review",
            "packet_entry": "Scope Keeper, Citation Auditor, Freight Industry, Schematic Cartographer, and V&V rerun required",
            "source_anchor": "docs/how-to/external-rehearsal-packet-selection-runbook.md",
            "required_role": "V&V",
            "status": "venue_specific_role_review_required",
            "allowed_language": "A venue-specific role rerun is required before external use.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "create a named packet role review after a venue is selected",
        },
        {
            "packet_id": "CAN-PORT-PREFLIGHT-007",
            "section": "validation",
            "packet_entry": "prohibited-claim scan, Canada port preflight gate, Canada media proof gate, and L0",
            "source_anchor": "package.json#check:canada:port-authority-packet",
            "required_role": "V&V",
            "status": "preflight_validation_defined",
            "allowed_language": "Validation requirements are defined for a future named port packet.",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "run packet-specific validation again after filling a named external packet",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
