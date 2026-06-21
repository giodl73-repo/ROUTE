#!/usr/bin/env python3
"""Gate Japan source field inventory."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
INVENTORY = ROOT / "data" / "international-japan-source-field-inventory-001.csv"

FIELDS = [
    "inventory_id",
    "source_id",
    "source_family",
    "inventory_basis",
    "candidate_fields",
    "inventory_status",
    "evidence_label",
    "blocked_claims",
    "next_action",
]
REQUIRED_SOURCES = {
    "JPN-SRC-001",
    "JPN-SRC-002",
    "JPN-SRC-003",
    "JPN-SRC-004",
    "JPN-SRC-005",
    "JPN-SRC-006",
    "JPN-SRC-007",
    "JPN-SRC-SLA-001",
}
REQUIRED_BLOCKS = {
    "official_corridor_designation",
    "ministry_approval",
    "route_designation",
    "source_row_validation",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "disaster_readiness",
    "guaranteed_sla",
    "numeric_roi",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
}


def main() -> int:
    with INVENTORY.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("Japan field inventory columns do not match contract")
    if {row["source_id"] for row in rows} != REQUIRED_SOURCES:
        failures.append("Japan field inventory source coverage mismatch")
    for row in rows:
        if row["evidence_label"] not in {"source-candidate", "source-needed", "held", "heuristic-held"}:
            failures.append(f"{row['source_id']} has unsupported evidence label")
        if "evidence not accepted" not in row["inventory_basis"] and row["source_id"] != "JPN-SRC-SLA-001":
            failures.append(f"{row['source_id']} does not preserve evidence-not-accepted posture")
        if row["source_id"] != "JPN-SRC-SLA-001" and not row["candidate_fields"]:
            failures.append(f"{row['source_id']} missing candidate fields")
        if row["source_id"] == "JPN-SRC-004" and row["evidence_label"] != "source-needed":
            failures.append("JPN-SRC-004 must remain source-needed until GSI probe is usable")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['source_id']} missing blocked claims: {sorted(missing)}")
        if "before" not in row["next_action"]:
            failures.append(f"{row['source_id']} next action must preserve before-promotion dependency")
    if failures:
        print("Japan source field inventory gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Japan source field inventory gate: PASS")
    print("  checked source coverage, field posture, evidence labels, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
