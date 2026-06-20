#!/usr/bin/env python3
"""Gate the Canada node fixture replacement contract."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "data" / "international-canada-node-fixture-contract-001.csv"
SELECTION = ROOT / "data" / "international-canada-node-source-selection-001.csv"
PROBE = ROOT / "data" / "international-canada-node-source-probe-001.csv"

FIELDS = [
    "contract_id",
    "replacement_target",
    "replacement_source",
    "required_rows",
    "contract_decision",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

REQUIRED_NODES = {
    "CAN-PORT-VANCOUVER",
    "CAN-PORT-MONTREAL",
    "CAN-PORT-HALIFAX",
}
REQUIRED_BLOCKS = {
    "port_endorsement",
    "terminal_performance",
    "node_completeness",
    "throughput_proof",
    "road_access_proof",
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
    fields, rows = read_csv(CONTRACT)
    _, selection_rows = read_csv(SELECTION)
    _, probe_rows = read_csv(PROBE)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("node fixture contract columns do not match required contract")
    if len(rows) != 1:
        failures.append("node fixture contract must contain one closeout target row")

    selected_nodes = {row["node_id"] for row in selection_rows}
    probed_nodes = {row["node_id"] for row in probe_rows}
    if REQUIRED_NODES - selected_nodes:
        failures.append(f"source selection missing nodes: {sorted(REQUIRED_NODES - selected_nodes)}")
    if REQUIRED_NODES - probed_nodes:
        failures.append(f"source probe missing nodes: {sorted(REQUIRED_NODES - probed_nodes)}")

    for row in rows:
        if row["replacement_target"] != "data/canada_source_node_candidates.csv":
            failures.append("node fixture contract targets the wrong file")
        if row["allowed_use"] != "internal adapter node-catalog fixture rows only":
            failures.append("node fixture contract allowed use is too broad")
        if row["contract_decision"] != "node_fixture_contract_ready_for_internal_closeout":
            failures.append("node fixture contract does not hold public/external promotion")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"node fixture contract missing blocked claims: {sorted(missing_blocks)}")

    if failures:
        print("Canada node fixture contract gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada node fixture contract gate: PASS")
    print("  checked selected/probed nodes, internal allowed use, and claim blocks")
    return 0


if __name__ == "__main__":
    sys.exit(main())
