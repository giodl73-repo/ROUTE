#!/usr/bin/env python3
"""Gate EU Rhine-Alpine port-node fixture contract."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-port-node-fixture-contract-001.csv"

FIELDS = [
    "contract_id",
    "candidate_source",
    "replacement_target",
    "required_rows",
    "geometry_contract",
    "contract_decision",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_ROWS = {"NLRTM", "BEANR", "ITGOA", "CHBSL", "DEDUI"}
REQUIRED_BLOCKS = {
    "fixture_replacement",
    "internal_adapter_proof",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "terminal_performance",
    "node_completeness",
    "road_access_proof",
    "throughput_proof",
    "guaranteed_sla",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("EU port-node fixture contract columns do not match contract")
    if len(rows) != 2:
        failures.append("EU port-node fixture contract must have two rows")
    internal_rows = [row for row in rows if row["contract_id"] == "EUR-PORT-NODE-CONTRACT-001"]
    if len(internal_rows) != 1:
        failures.append("EU port-node fixture contract must have one internal contract row")
    else:
        internal = internal_rows[0]
        if set(internal["required_rows"].split(";")) != REQUIRED_ROWS:
            failures.append("internal contract required rows mismatch")
        if internal["geometry_contract"] != "no_geometry_attribute_rows_only":
            failures.append("internal contract must remain no-geometry")
        if internal["contract_decision"] != "contract_ready_for_internal_closeout_not_replacement":
            failures.append("internal contract must stop before replacement")
        if "before any internal adapter proof" not in internal["next_action"]:
            failures.append("internal contract must block internal adapter proof")
    for row in rows:
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['contract_id']} missing blocked claims: {sorted(missing)}")
        if row["allowed_use"] == "none" and "public" not in row["next_action"]:
            failures.append(f"{row['contract_id']} public-use block must name public review")
    if failures:
        print("EU Rhine-Alpine port-node fixture contract gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine port-node fixture contract gate: PASS")
    print("  checked no-geometry internal contract, public-use block, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
