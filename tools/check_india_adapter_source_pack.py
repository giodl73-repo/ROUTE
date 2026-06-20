#!/usr/bin/env python3
"""Gate India adapter source-pack preflight ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-india-adapter-source-pack-001.csv"

FIELDS = [
    "source_family",
    "source_id",
    "source_path_or_status",
    "owner_or_publisher",
    "date_accessed",
    "required_fields",
    "adapter_target",
    "promotion_decision",
    "claim_boundary",
    "next_action",
]
REQUIRED_FAMILIES = {
    "highway_ministry_context",
    "highway_authority_context",
    "port_system_context",
    "port_statistics_context",
    "hierarchy_fixture_context",
    "service_targets",
}
REQUIRED_BLOCKS = {
    "official",
    "approval",
    "guaranteed SLA",
    "ROI",
    "endorsement",
    "validation",
    "public-readiness",
    "external-readiness",
}
PROHIBITED_DECISIONS = {
    "promoted",
    "validated",
    "approved",
    "official",
    "ready",
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("India source-pack columns do not match contract")
    if len(rows) != 6:
        failures.append("India source pack must have six source-family rows")
    families = {row["source_family"] for row in rows}
    if families != REQUIRED_FAMILIES:
        failures.append(f"India source families mismatch: {sorted(families)}")
    for row in rows:
        row_id = row.get("source_id", "<missing>")
        if not row["source_path_or_status"]:
            failures.append(f"{row_id} missing source path or status")
        if not row["owner_or_publisher"]:
            failures.append(f"{row_id} missing owner or publisher")
        if row["date_accessed"] != "2026-06-20":
            failures.append(f"{row_id} access date must be 2026-06-20")
        decision = row["promotion_decision"].lower()
        for word in PROHIBITED_DECISIONS:
            if word in decision and "not promoted" not in decision:
                failures.append(f"{row_id} promotion decision overclaims: {row['promotion_decision']}")
        for required in REQUIRED_BLOCKS:
            if required not in row["claim_boundary"]:
                failures.append(f"{row_id} missing blocked claim token: {required}")
        if "before" not in row["next_action"] and row["source_family"] != "service_targets":
            failures.append(f"{row_id} next action must name a before-promotion dependency")
    if failures:
        print("India adapter source-pack gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("India adapter source-pack gate: PASS")
    print("  checked source families, promotion holds, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
