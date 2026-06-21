#!/usr/bin/env python3
"""Gate international system flexibility proof ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-system-flexibility-proof-001.csv"

FIELDS = [
    "proof_id",
    "region_or_surface",
    "kernel_step",
    "observed_variation",
    "system_response",
    "flexibility_decision",
    "evidence_artifacts",
    "blocked_claims",
    "next_action",
]
REQUIRED_DECISIONS = {
    "depth_instance_complete_external_validation_held",
    "adaptive_branch_complete_fixture_replacement_held",
    "adaptive_branch_complete_source_row_validation_held",
    "gap_detected_without_false_promotion",
    "breadth_instance_complete_validation_held",
}
REQUIRED_BLOCKS = {
    "official_network",
    "route_designation",
    "agency_approval",
    "external_validation",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "construction_ready",
    "guaranteed_sla",
    "numeric_roi",
    "roi",
    "validation",
    "public_readiness",
    "external_readiness",
}
PROHIBITED = {
    "official network",
    "guaranteed sla",
    "proves roi",
    "validated by",
    "approved by",
    "construction ready",
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("international flexibility proof columns do not match contract")
    if len(rows) != 6:
        failures.append("international flexibility proof must have six rows")
    decisions = {row["flexibility_decision"] for row in rows}
    if decisions != REQUIRED_DECISIONS:
        failures.append(f"flexibility decisions mismatch: {sorted(decisions)}")
    if not any(row["region_or_surface"] == "Canada" for row in rows):
        failures.append("flexibility proof must include Canada depth instance")
    if not any(row["region_or_surface"] == "EU Rhine-Alpine" for row in rows):
        failures.append("flexibility proof must include EU adaptive branch")
    if not any(row["region_or_surface"] == "India" for row in rows):
        failures.append("flexibility proof must include India adaptive branch")
    if not any(row["region_or_surface"] == "Japan" for row in rows):
        failures.append("flexibility proof must include Japan adaptive branch")
    if not any("current corridor scope" in row["next_action"] for row in rows):
        failures.append("flexibility proof must name EU rebase next action")
    if not any("source-row validation" in row["next_action"] and row["region_or_surface"] == "India" for row in rows):
        failures.append("flexibility proof must name India source-row validation hold")
    if not any("GSI road-link source custody" in row["next_action"] and row["region_or_surface"] == "Japan" for row in rows):
        failures.append("flexibility proof must name Japan GSI source-custody hold")
    for row in rows:
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['proof_id']} missing blocked claims: {sorted(missing)}")
        if not row["evidence_artifacts"]:
            failures.append(f"{row['proof_id']} missing evidence artifacts")
        text = " ".join(
            [
                row["observed_variation"],
                row["system_response"],
                row["flexibility_decision"],
                row["next_action"],
            ]
        ).lower()
        for phrase in PROHIBITED:
            if phrase in text:
                failures.append(f"{row['proof_id']} promotes prohibited phrase: {phrase}")
    if failures:
        print("International system flexibility proof gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("International system flexibility proof gate: PASS")
    print("  checked depth, breadth, adaptive branch, gap detection, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
