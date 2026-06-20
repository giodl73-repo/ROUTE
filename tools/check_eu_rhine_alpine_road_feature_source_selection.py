#!/usr/bin/env python3
"""Gate EU Rhine-Alpine road-feature and node source selection."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-road-feature-source-selection-001.csv"

FIELDS = [
    "selection_id",
    "source_id",
    "source_family",
    "selected_for",
    "source_url",
    "source_owner",
    "source_date",
    "observed_source_capability",
    "selection_decision",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_SELECTED_FOR = {
    "road_feature_probe",
    "port_node_probe",
    "scope_rebase_context",
    "legacy_context_only",
}
REQUIRED_BLOCKS = {
    "fixture_replacement",
    "internal_adapter_proof",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
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
        failures.append("EU road-feature source-selection columns do not match contract")
    if len(rows) != 4:
        failures.append("EU road-feature source-selection must have four rows")
    if {row["selected_for"] for row in rows} != REQUIRED_SELECTED_FOR:
        failures.append("EU source-selection rows must cover road, node, scope, and legacy decisions")
    if not any(row["selected_for"] == "road_feature_probe" and row["source_id"] == "EUR-SRC-003" for row in rows):
        failures.append("EU road-feature probe must select GISCO source family")
    if not any(row["selected_for"] == "port_node_probe" and "Ports 2013" in row["observed_source_capability"] for row in rows):
        failures.append("EU node probe must preserve GISCO Ports 2013 candidate")
    for row in rows:
        if not row["source_url"].startswith("https://"):
            failures.append(f"{row['selection_id']} missing source URL")
        if "replacement" in row["selection_decision"]:
            failures.append(f"{row['selection_id']} decision must not promote replacement")
        if "before" not in row["next_action"]:
            failures.append(f"{row['selection_id']} next action must preserve before dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['selection_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine road-feature source-selection gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine road-feature source-selection gate: PASS")
    print("  checked source choices, next probes, rebase context, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
