#!/usr/bin/env python3
"""Gate India source field inventory."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
INVENTORY = ROOT / "data" / "international-india-source-field-inventory-001.csv"

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
    "IND-SRC-001",
    "IND-SRC-002",
    "IND-SRC-003",
    "IND-SRC-004",
    "IND-SRC-005",
    "IND-SRC-SLA-001",
}
REQUIRED_BLOCKS = {
    "official_corridor_designation",
    "national_approval",
    "state_approval",
    "source_row_validation",
    "fixture_replacement",
    "geometry_acceptance",
    "topology_proof",
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
        failures.append("India field inventory columns do not match contract")
    if {row["source_id"] for row in rows} != REQUIRED_SOURCES:
        failures.append("India field inventory source coverage mismatch")
    for row in rows:
        if row["evidence_label"] not in {"source-candidate", "source-needed", "held", "heuristic-held"}:
            failures.append(f"{row['source_id']} has unsupported evidence label")
        if "evidence not accepted" not in row["inventory_basis"] and row["source_id"] != "IND-SRC-SLA-001":
            failures.append(f"{row['source_id']} does not preserve evidence-not-accepted posture")
        if row["source_id"] != "IND-SRC-SLA-001" and not row["candidate_fields"]:
            failures.append(f"{row['source_id']} missing candidate fields")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['source_id']} missing blocked claims: {sorted(missing)}")
        if "before" not in row["next_action"]:
            failures.append(f"{row['source_id']} next action must preserve before-promotion dependency")
    if failures:
        print("India source field inventory gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("India source field inventory gate: PASS")
    print("  checked source coverage, field posture, evidence labels, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
