#!/usr/bin/env python3
"""Gate EU Rhine-Alpine adaptive proof closeout ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CLOSEOUT = ROOT / "data" / "international-eu-rhine-alpine-adaptive-proof-closeout-001.csv"
NODE_CLOSEOUT = ROOT / "data" / "international-eu-rhine-alpine-port-node-fixture-closeout-001.csv"
TARGET_POSTURE = ROOT / "data" / "international-eu-rhine-alpine-target-posture-001.csv"
ROAD_DISPOSITION = ROOT / "data" / "international-eu-rhine-alpine-road-link-source-disposition-001.csv"
ROAD_REQUEST = ROOT / "data" / "international-eu-rhine-alpine-road-link-endpoint-request-001.csv"
PARITY_GAP = ROOT / "data" / "international-eu-rhine-alpine-parity-gap-001.csv"

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
    "node_fixture_branch",
    "target_posture",
    "road_link_blocker",
    "adaptive_proof_decision",
}
REQUIRED_BLOCKS = {
    "canada_depth_equivalence",
    "internal_adapter_proof",
    "official_network",
    "source_row_validation_for_road_links",
    "link_fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "agency_review",
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
    "link_fixture_ready",
    "validated",
    "public_ready",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    fields, rows = read_csv(CLOSEOUT)
    _, node_rows = read_csv(NODE_CLOSEOUT)
    _, target_rows = read_csv(TARGET_POSTURE)
    _, disposition_rows = read_csv(ROAD_DISPOSITION)
    _, request_rows = read_csv(ROAD_REQUEST)
    _, parity_rows = read_csv(PARITY_GAP)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("EU adaptive closeout columns do not match contract")
    surfaces = {row["proof_surface"] for row in rows}
    missing_surfaces = REQUIRED_SURFACES - surfaces
    if missing_surfaces:
        failures.append(f"EU adaptive closeout missing surfaces: {sorted(missing_surfaces)}")
    if len(rows) != 6:
        failures.append("EU adaptive closeout must contain six proof surfaces")
    if not node_rows or node_rows[0]["replacement_status"] != "internal_node_fixture_replaced_no_geometry":
        failures.append("EU adaptive closeout requires node fixture closeout")
    if not target_rows or target_rows[0]["target_status"] != "held_planning_assumptions_accepted_for_internal_proof":
        failures.append("EU adaptive closeout requires held target posture")
    if not disposition_rows or disposition_rows[0]["disposition"] != "official_endpoint_not_acquired_contact_or_alternative_source_required":
        failures.append("EU adaptive closeout requires missing road-link endpoint disposition")
    if len(request_rows) != 4:
        failures.append("EU adaptive closeout requires four-lane endpoint request packet")
    if not any(row.get("parity_decision") == "blocked" for row in parity_rows):
        failures.append("EU adaptive closeout requires blocked parity gap row")

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
        failures.append("EU adaptive closeout requires one decision row")
    elif decision[0]["closeout_status"] != "adaptive_proof_complete_canada_depth_not_claimed":
        failures.append("EU adaptive closeout decision must avoid Canada-depth claim")

    if failures:
        print("EU Rhine-Alpine adaptive proof closeout gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("EU Rhine-Alpine adaptive proof closeout gate: PASS")
    print("  checked adaptive completion, node closeout, target holds, road blocker, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
