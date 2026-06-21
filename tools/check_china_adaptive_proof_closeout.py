#!/usr/bin/env python3
"""Gate China adaptive proof closeout ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CLOSEOUT = ROOT / "data" / "international-china-adaptive-proof-closeout-001.csv"
FIXTURE_BLOCKER = ROOT / "data" / "international-china-fixture-blocker-001.csv"
ROLE_REVIEW = ROOT / "data" / "international-china-dry-run-role-review-001.csv"
GEOMETRY_POLICY = ROOT / "data" / "international-china-geometry-policy-001.csv"

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
    "dry_run_and_role_review",
    "geometry_and_fixture_blocker",
    "fixture_replacement_decision",
    "adaptive_proof_decision",
}
REQUIRED_BLOCKS = {
    "canada_depth_equivalence",
    "india_japan_content_depth_equivalence",
    "internal_adapter_proof",
    "official_network",
    "policy_alignment",
    "source_row_validation",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
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
    "content_depth_equivalent",
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
    _, role_rows = read_csv(ROLE_REVIEW)
    _, geometry_rows = read_csv(GEOMETRY_POLICY)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("China adaptive closeout columns do not match contract")
    surfaces = {row["proof_surface"] for row in rows}
    missing_surfaces = REQUIRED_SURFACES - surfaces
    if missing_surfaces:
        failures.append(f"China adaptive closeout missing surfaces: {sorted(missing_surfaces)}")
    if len(rows) != 6:
        failures.append("China adaptive closeout must contain six proof surfaces")
    if not blocker_rows or blocker_rows[0]["replacement_decision"] != "blocked_dry_run_rows_not_source_validated_geometry_not_accepted":
        failures.append("China adaptive closeout requires fixture blocker")
    if len(role_rows) != 5 or not all(row["result"] == "pass_with_holds" for row in role_rows):
        failures.append("China adaptive closeout requires dry-run role review pass with holds")
    if not geometry_rows or not all(row["current_geometry_status"] == "not_requested" for row in geometry_rows):
        failures.append("China adaptive closeout requires no-geometry policy")

    for row in rows:
        if row["closeout_status"] in PROHIBITED_STATUSES:
            failures.append(f"{row['closeout_id']} uses prohibited closeout status")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['closeout_id']} missing blocked claims: {sorted(missing)}")
        if not row["input_artifacts"]:
            failures.append(f"{row['closeout_id']} missing input artifacts")
        text = " ".join([row["closeout_status"], row["allowed_claim"], row["next_action"]]).lower()
        if "content-depth proof" in text and row["proof_surface"] != "adaptive_proof_decision":
            failures.append(f"{row['closeout_id']} discusses content-depth outside decision row")
    decision = [row for row in rows if row["proof_surface"] == "adaptive_proof_decision"]
    if len(decision) != 1:
        failures.append("China adaptive closeout requires one decision row")
    elif decision[0]["closeout_status"] != "adaptive_proof_complete_at_dry_run_depth_canada_india_japan_depth_not_claimed":
        failures.append("China adaptive closeout decision must avoid Canada/India/Japan depth claim")
    if not any("dry-run-depth" in row["allowed_claim"] or "dry-run-depth" in row["next_action"] for row in rows):
        failures.append("China adaptive closeout must name dry-run-depth proof")

    if failures:
        print("China adaptive proof closeout gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("China adaptive proof closeout gate: PASS")
    print("  checked dry-run-depth adaptive completion, fixture blocker, role holds, geometry hold, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
