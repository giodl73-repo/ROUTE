#!/usr/bin/env python3
"""Gate Canada port authority packet preflight boundaries."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-canada-port-authority-packet-preflight-001.csv"

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

REQUIRED_SECTIONS = {
    "metadata",
    "source_custody",
    "materials",
    "role_review",
    "validation",
}

REQUIRED_NODE_ANCHORS = {
    "data/canada_source_node_candidates.csv#CAN-PORT-VANCOUVER",
    "data/canada_source_node_candidates.csv#CAN-PORT-MONTREAL",
    "data/canada_source_node_candidates.csv#CAN-PORT-HALIFAX",
}

ALLOWED_STATUSES = {
    "preflight_only_no_named_venue",
    "source_candidate_internal_only",
    "material_set_selected_for_preflight",
    "venue_specific_role_review_required",
    "preflight_validation_defined",
}

REQUIRED_BLOCKS = {
    "port_endorsement",
    "port_review",
    "agency_approval",
    "external_validation",
    "official_network",
    "route_designation",
    "terminal_performance",
    "node_completeness",
    "road_access_proof",
    "throughput_proof",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "construction_ready",
    "guaranteed_sla",
    "travel_time_proof",
    "delivery_commitment",
    "numeric_roi",
    "roi",
    "eligibility",
    "compliance",
    "endorsement",
    "validation",
    "public_readiness",
    "external_readiness",
}

PROHIBITED_PROMOTIONS = {
    "reviewed by",
    "approved by",
    "validated by",
    "endorsed by",
    "adopted by",
    "port authority reviewed",
    "external validation complete",
    "official canadian network",
    "terminal performance proven",
    "throughput proven",
    "guaranteed sla",
    "proves roi",
    "construction ready",
    "public ready",
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)

    failures: list[str] = []
    if fields != FIELDS:
        failures.append("Canada port packet preflight columns do not match contract")
    if len(rows) != 7:
        failures.append("Canada port packet preflight must have seven rows")

    sections = {row["section"] for row in rows}
    if not REQUIRED_SECTIONS <= sections:
        failures.append(f"Canada port packet preflight missing sections: {sorted(REQUIRED_SECTIONS - sections)}")

    anchors = {row["source_anchor"] for row in rows}
    missing_nodes = REQUIRED_NODE_ANCHORS - anchors
    if missing_nodes:
        failures.append(f"Canada port packet preflight missing node anchors: {sorted(missing_nodes)}")

    for row in rows:
        row_id = row.get("packet_id", "<missing>")
        if row["status"] not in ALLOWED_STATUSES:
            failures.append(f"{row_id} has unsupported status: {row['status']}")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"{row_id} missing blocked claims: {sorted(missing_blocks)}")
        text = " ".join(
            [
                row["packet_entry"],
                row["source_anchor"],
                row["allowed_language"],
                row["next_action"],
            ]
        ).lower()
        for phrase in PROHIBITED_PROMOTIONS:
            if phrase in text:
                failures.append(f"{row_id} promotes prohibited phrase: {phrase}")
        if "named" not in row["next_action"] and row["section"] in {"metadata", "role_review", "validation"}:
            failures.append(f"{row_id} must keep named-venue dependency explicit")

    if failures:
        print("Canada port authority packet preflight gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada port authority packet preflight gate: PASS")
    print("  checked node anchors, statuses, named-venue dependency, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
