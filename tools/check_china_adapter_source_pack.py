#!/usr/bin/env python3
"""Gate China adapter source-pack preflight ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-china-adapter-source-pack-001.csv"

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
    "transport_ministry_context",
    "transport_plan_context",
    "transport_statistics_context",
    "highway_standards_context",
    "port_waterway_context",
    "hierarchy_fixture_context",
    "service_targets",
}
REQUIRED_BLOCKS = {
    "official",
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
        failures.append("China source-pack columns do not match contract")
    if len(rows) != 7:
        failures.append("China source pack must have seven source-family rows")
    families = {row["source_family"] for row in rows}
    if families != REQUIRED_FAMILIES:
        failures.append(f"China source families mismatch: {sorted(families)}")
    for row in rows:
        row_id = row.get("source_id", "<missing>")
        if not row["source_path_or_status"]:
            failures.append(f"{row_id} missing source path or status")
        if not row["owner_or_publisher"]:
            failures.append(f"{row_id} missing owner or publisher")
        if row["date_accessed"] != "2026-06-21":
            failures.append(f"{row_id} access date must be 2026-06-21")
        decision = row["promotion_decision"].lower()
        for word in PROHIBITED_DECISIONS:
            if word in decision and "not promoted" not in decision and "not accepted" not in decision:
                failures.append(f"{row_id} promotion decision overclaims: {row['promotion_decision']}")
        for required in REQUIRED_BLOCKS:
            if required not in row["claim_boundary"]:
                failures.append(f"{row_id} missing blocked claim token: {required}")
        if row["source_family"] != "service_targets" and "before" not in row["next_action"]:
            failures.append(f"{row_id} next action must name a before-promotion dependency")
    if failures:
        print("China adapter source-pack gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("China adapter source-pack gate: PASS")
    print("  checked source families, promotion holds, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
