#!/usr/bin/env python3
"""Gate Japan adaptive proof closeout ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CLOSEOUT = ROOT / "data" / "international-japan-adaptive-proof-closeout-001.csv"
FIXTURE_BLOCKER = ROOT / "data" / "international-japan-fixture-blocker-001.csv"
CONTENT_ROWVAL = ROOT / "data" / "international-japan-source-content-row-validation-001.csv"
CONTENT_ROLE = ROOT / "data" / "international-japan-content-row-role-review-001.csv"
GEOMETRY_POLICY = ROOT / "data" / "international-japan-geometry-policy-001.csv"

FIELDS = [
    "closeout_id",
    "proof_surface",
    "input_artifacts",
    "closeout_status",
    "allowed_claim",
    "blocked_claims",
    "next_action",
]
REQUIRED_SURFACES = {
    "hierarchy_and_map_fixture",
    "source_kernel_and_parser_contract",
    "source_content_branch",
    "geometry_and_fixture_blocker",
    "fixture_replacement_decision",
    "adaptive_proof_decision",
}
REQUIRED_BLOCKS = {
    "canada_depth_equivalence",
    "internal_adapter_proof",
    "official_network",
    "source_row_validation",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "disaster_readiness",
    "terminal_performance",
    "throughput_proof",
    "guaranteed_sla",
    "numeric_roi",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
}
PROHIBITED_STATUSES = {
    "internal_adapter_proof_ready",
    "canada_depth_equivalent",
    "fixture_replacement_ready",
    "source_row_validated",
    "validated",
    "public_ready",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    fields, rows = read_csv(CLOSEOUT)
    _, blocker_rows = read_csv(FIXTURE_BLOCKER)
    _, content_rowval = read_csv(CONTENT_ROWVAL)
    _, content_role = read_csv(CONTENT_ROLE)
    _, geometry_rows = read_csv(GEOMETRY_POLICY)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("Japan adaptive closeout columns do not match contract")
    surfaces = {row["proof_surface"] for row in rows}
    missing_surfaces = REQUIRED_SURFACES - surfaces
    if missing_surfaces:
        failures.append(f"Japan adaptive closeout missing surfaces: {sorted(missing_surfaces)}")
    if len(rows) != 6:
        failures.append("Japan adaptive closeout must contain six proof surfaces")
    if not blocker_rows or blocker_rows[0]["replacement_decision"] != "blocked_source_rows_not_validated_gsi_link_source_needed_geometry_not_accepted":
        failures.append("Japan adaptive closeout requires fixture blocker")
    if len(content_rowval) != 6 or not any(row["validation_result"] == "source_content_blocker_matched_not_source_row_validated" for row in content_rowval):
        failures.append("Japan adaptive closeout requires content rows plus GSI blocker matched but not source-row validated")
    if len(content_role) != 5 or not all(row["result"] == "pass_with_holds" for row in content_role):
        failures.append("Japan adaptive closeout requires content-row role review pass with holds")
    if not geometry_rows or not all(row["current_geometry_status"] == "not_requested" for row in geometry_rows):
        failures.append("Japan adaptive closeout requires no-geometry policy")

    for row in rows:
        if row["closeout_status"] in PROHIBITED_STATUSES:
            failures.append(f"{row['closeout_id']} uses prohibited closeout status")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['closeout_id']} missing blocked claims: {sorted(missing)}")
        if not row["input_artifacts"]:
            failures.append(f"{row['closeout_id']} missing input artifacts")
    decision = [row for row in rows if row["proof_surface"] == "adaptive_proof_decision"]
    if len(decision) != 1:
        failures.append("Japan adaptive closeout requires one decision row")
    elif decision[0]["closeout_status"] != "adaptive_proof_complete_canada_depth_not_claimed":
        failures.append("Japan adaptive closeout decision must avoid Canada-depth claim")
    if not any("GSI" in row["allowed_claim"] or "GSI" in row["next_action"] for row in rows):
        failures.append("Japan adaptive closeout must preserve GSI source-needed blocker")

    if failures:
        print("Japan adaptive proof closeout gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Japan adaptive proof closeout gate: PASS")
    print("  checked adaptive completion, GSI blocker, fixture blocker, content holds, geometry hold, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
