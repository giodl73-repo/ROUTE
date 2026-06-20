#!/usr/bin/env python3
"""Gate reusable international adapter proof-kernel ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-adapter-proof-kernel-001.csv"

FIELDS = [
    "kernel_step",
    "generic_function",
    "canada_instance",
    "evidence_artifact",
    "status",
    "reusable_for",
    "blocked_claims",
]

REQUIRED_STEPS = {
    "source_custody",
    "parser_contract",
    "fixture_replacement",
    "target_posture",
    "review_packet",
}

REQUIRED_BLOCKS = {
    "official_network",
    "route_designation",
    "agency_approval",
    "external_validation",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
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
    "public_readiness",
    "external_readiness",
}

PROHIBITED_PROMOTIONS = {
    "official network",
    "approved by",
    "validated by",
    "endorsed by",
    "guaranteed sla",
    "proves roi",
    "construction ready",
    "public ready",
    "external ready",
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)

    failures: list[str] = []
    if fields != FIELDS:
        failures.append("international adapter proof kernel columns do not match contract")
    if len(rows) != 5:
        failures.append("international adapter proof kernel must have five rows")

    steps = {row["kernel_step"] for row in rows}
    if steps != REQUIRED_STEPS:
        failures.append(f"kernel steps mismatch: {sorted(steps)}")

    for row in rows:
        step = row.get("kernel_step", "<missing>")
        if row["status"] != "generic_kernel_instantiated_by_canada":
            failures.append(f"{step} has unsupported status: {row['status']}")
        if row["reusable_for"] != "country_or_region_adapter":
            failures.append(f"{step} must remain reusable for country_or_region_adapter")
        if not row["evidence_artifact"]:
            failures.append(f"{step} missing evidence artifacts")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"{step} missing blocked claims: {sorted(missing_blocks)}")
        text = " ".join(
            [row["generic_function"], row["canada_instance"], row["evidence_artifact"]]
        ).lower()
        for phrase in PROHIBITED_PROMOTIONS:
            if phrase in text:
                failures.append(f"{step} promotes prohibited phrase: {phrase}")

    if failures:
        print("International adapter proof kernel gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("International adapter proof kernel gate: PASS")
    print("  checked reusable steps, Canada instantiation, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
