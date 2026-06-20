#!/usr/bin/env python3
"""Gate Canada external review pathway claim boundaries."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-canada-external-review-pathway-001.csv"

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

REQUIRED_LANES = {
    "federal_transport",
    "port_authority",
    "provincial_or_regional_transport",
    "academic_or_transport_research",
    "external_validation_decision",
}

ALLOWED_STATUSES = {
    "candidate_lane_not_contacted",
    "external_validation_not_started",
}

REQUIRED_INPUTS = {
    "docs/reviews/international-canada-internal-adapter-proof-001.md",
    "docs/media/canada-internal-proof-brief.md",
    "docs/how-to/external-rehearsal-packet-selection-runbook.md",
}

REQUIRED_BLOCKS = {
    "official_network",
    "route_designation",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "agency_approval",
    "provincial_approval",
    "port_endorsement",
    "terminal_performance",
    "node_completeness",
    "road_access_proof",
    "throughput_proof",
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
    "external_validation",
    "public_readiness",
    "external_readiness",
}

PROHIBITED_PROMOTIONS = {
    "reviewed by",
    "approved by",
    "validated by",
    "endorsed by",
    "adopted by",
    "transport canada reviewed",
    "port authority reviewed",
    "external validation complete",
    "official canadian network",
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
        failures.append("Canada external review pathway columns do not match contract")
    if len(rows) != 5:
        failures.append("Canada external review pathway must have five rows")

    lanes = {row["review_lane"] for row in rows}
    if lanes != REQUIRED_LANES:
        failures.append(f"Canada external review lanes mismatch: {sorted(lanes)}")

    for row in rows:
        row_id = row.get("pathway_id", "<missing>")
        if row["status"] not in ALLOWED_STATUSES:
            failures.append(f"{row_id} has unsupported status: {row['status']}")
        inputs = set(row["input_artifacts"].split(";"))
        missing_inputs = REQUIRED_INPUTS - inputs
        if missing_inputs:
            failures.append(f"{row_id} missing input artifacts: {sorted(missing_inputs)}")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"{row_id} missing blocked claims: {sorted(missing_blocks)}")
        text = " ".join(
            [
                row["candidate_reviewer"],
                row["packet_focus"],
                row["safe_ask"],
                row["allowed_language"],
                row["next_action"],
            ]
        ).lower()
        for phrase in PROHIBITED_PROMOTIONS:
            if phrase in text:
                failures.append(f"{row_id} promotes prohibited phrase: {phrase}")
        if row["review_lane"] == "external_validation_decision":
            if row["status"] != "external_validation_not_started":
                failures.append("external validation decision row must stay not started")
            if row["allowed_language"] != "External validation for Canada has not started.":
                failures.append("external validation decision row must state validation has not started")

    if failures:
        print("Canada external review pathway gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada external review pathway gate: PASS")
    print("  checked review lanes, input artifacts, statuses, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
