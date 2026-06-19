#!/usr/bin/env python3
"""Gate Canada fixture replacement contract before closeout."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "data" / "international-canada-fixture-replacement-contract-001.csv"

FIELDS = [
    "contract_id",
    "candidate_table",
    "replacement_target",
    "geometry_contract",
    "contract_decision",
    "allowed_use",
    "required_inputs",
    "blocked_uses",
    "blocked_claims",
    "next_action",
]

REQUIRED_BLOCKS = {
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "parsed_adapter",
    "official_network",
    "route_designation",
    "engineering_precision",
    "agency_approval",
    "construction_ready",
    "guaranteed_sla",
    "roi",
    "eligibility",
    "compliance",
    "endorsement",
    "validation",
    "public_readiness",
    "external_readiness",
}


def main() -> int:
    with CONTRACT.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("replacement contract columns do not match required contract")
    if len(rows) != 2:
        failures.append("replacement contract must separate link-fixture and map/adapter surfaces")
    link_rows = [row for row in rows if row["replacement_target"] == "data/canada_source_link_candidates.csv"]
    if len(link_rows) != 1:
        failures.append("replacement contract missing single link-fixture target")
    for row in rows:
        if "public" in row["allowed_use"] or "external" in row["allowed_use"]:
            failures.append(f"{row['contract_id']} allows public/external use")
        if row["contract_id"] == "CAN-REPLACE-CONTRACT-001":
            if row["geometry_contract"] != "no_geometry_candidate_rows_allowed":
                failures.append("link-fixture contract does not explicitly allow no-geometry candidates")
            if "internal parser link-candidate fixture rows only" != row["allowed_use"]:
                failures.append("link-fixture contract allowed use is too broad")
        if row["contract_id"] == "CAN-REPLACE-CONTRACT-002":
            if row["allowed_use"] != "none":
                failures.append("map/adapter surface should allow no use")
        blocked = set(row["blocked_claims"].split(";"))
        missing = REQUIRED_BLOCKS - blocked
        if missing:
            failures.append(f"{row['contract_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("Canada fixture replacement contract gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Canada fixture replacement contract gate: PASS")
    print("  checked no-geometry link contract, map/adapter exclusion, required inputs, and claim holds")
    return 0


if __name__ == "__main__":
    sys.exit(main())
