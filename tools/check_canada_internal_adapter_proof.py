#!/usr/bin/env python3
"""Gate Canada internal adapter proof closeout."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PROOF = ROOT / "data" / "international-canada-internal-adapter-proof-001.csv"
NODES = ROOT / "data" / "canada_source_node_candidates.csv"
LINKS = ROOT / "data" / "canada_source_link_candidates.csv"
TARGETS = ROOT / "data" / "canada_service_target_candidates.csv"
NODE_CLOSEOUT = ROOT / "data" / "international-canada-node-fixture-replacement-closeout-001.csv"
TARGET_POSTURE = ROOT / "data" / "international-canada-target-posture-001.csv"

FIELDS = [
    "proof_id",
    "proof_surface",
    "input_artifacts",
    "proof_status",
    "allowed_claim",
    "blocked_claims",
    "next_action",
]
REQUIRED_SURFACES = {
    "link_fixture",
    "node_fixture",
    "need_and_target_tables",
    "adapter_proof_decision",
}
REQUIRED_BLOCKS = {
    "official_network",
    "route_designation",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "agency_approval",
    "port_endorsement",
    "terminal_performance",
    "node_completeness",
    "road_access_proof",
    "throughput_proof",
    "construction_ready",
    "guaranteed_sla",
    "roi",
    "compliance",
    "endorsement",
    "validation",
    "public_readiness",
    "external_readiness",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    fields, proof_rows = read_csv(PROOF)
    _, link_rows = read_csv(LINKS)
    _, node_rows = read_csv(NODES)
    _, target_rows = read_csv(TARGETS)
    _, node_closeout_rows = read_csv(NODE_CLOSEOUT)
    _, target_posture_rows = read_csv(TARGET_POSTURE)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("internal proof columns do not match required contract")
    surfaces = {row["proof_surface"] for row in proof_rows}
    missing_surfaces = REQUIRED_SURFACES - surfaces
    if missing_surfaces:
        failures.append(f"internal proof missing surfaces: {sorted(missing_surfaces)}")
    if len(link_rows) != 5:
        failures.append("internal proof requires five source-derived link rows")
    if len(node_rows) != 3:
        failures.append("internal proof requires three source-custody node rows")
    if not node_closeout_rows or node_closeout_rows[0]["replacement_status"] != "internal_node_fixture_replaced":
        failures.append("internal proof requires node fixture replacement closeout")
    if not target_posture_rows or target_posture_rows[0]["target_status"] != "held_planning_assumptions_accepted_for_internal_proof":
        failures.append("internal proof requires held target posture closeout")
    for row in target_rows:
        if row["evidence_label"] != "held":
            failures.append(f"{row['target_gap_id']} target row is not held")
    for row in proof_rows:
        if row["proof_status"] in {"official_network_ready", "external_validation_ready", "guaranteed_sla_ready"}:
            failures.append(f"{row['proof_id']} uses prohibited proof status")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"{row['proof_id']} missing blocked claims: {sorted(missing_blocks)}")
    decision = [row for row in proof_rows if row["proof_surface"] == "adapter_proof_decision"]
    if len(decision) != 1:
        failures.append("internal proof requires one adapter proof decision row")
    elif decision[0]["proof_status"] != "internal_adapter_proof_ready_external_validation_held":
        failures.append("adapter proof decision must hold external validation")

    if failures:
        print("Canada internal adapter proof gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada internal adapter proof gate: PASS")
    print("  checked internal proof surfaces, fixture readiness, target holds, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
