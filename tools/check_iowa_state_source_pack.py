#!/usr/bin/env python3
"""Gate Iowa state-highway source-pack preflight ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "state-highway-iowa-source-pack-001.csv"

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
    "state_roadway_inventory",
    "state_freight_and_economic_context",
    "iowa_511_operating_events",
    "state_program_and_delivery_context",
    "state_asset_and_maintenance_context",
    "des_moines_scenario_fixture",
    "state_service_targets",
}
REQUIRED_BLOCKS = {
    "official state plan",
    "state DOT endorsement",
    "FHWA approval",
    "route designation",
    "source-row validation",
    "geometry acceptance",
    "construction-ready",
    "funding commitment",
    "guaranteed SLA",
    "numeric ROI",
    "ROI",
    "environmental clearance",
    "right-of-way clearance",
    "maintenance commitment",
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
        failures.append("Iowa state source-pack columns do not match contract")
    if len(rows) != 7:
        failures.append("Iowa state source pack must have seven source-family rows")
    families = {row["source_family"] for row in rows}
    if families != REQUIRED_FAMILIES:
        failures.append(f"Iowa state source families mismatch: {sorted(families)}")
    if not any(row["source_id"] == "IA-SRC-003" and "snapshot-window candidate" in row["promotion_decision"] for row in rows):
        failures.append("Iowa source pack must preserve 511 snapshot-window candidate status")
    if not any(row["source_id"] == "IA-SRC-006" and "internal fixture" in row["promotion_decision"] for row in rows):
        failures.append("Iowa source pack must preserve scenario fixture as internal only")
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
            if word in decision and "not promoted" not in decision:
                failures.append(f"{row_id} promotion decision overclaims: {row['promotion_decision']}")
        for required in REQUIRED_BLOCKS:
            if required not in row["claim_boundary"]:
                failures.append(f"{row_id} missing blocked claim token: {required}")
        if "before" not in row["next_action"] and row["source_family"] != "state_service_targets":
            failures.append(f"{row_id} next action must name a before-promotion dependency")
    if failures:
        print("Iowa state source-pack gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Iowa state source-pack gate: PASS")
    print("  checked source families, state authority holds, promotion holds, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
