#!/usr/bin/env python3
"""Gate EU Rhine-Alpine port-node source-row validation."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-port-node-source-row-validation-001.csv"

FIELDS = [
    "validation_id",
    "sample_id",
    "port_id",
    "required_fields_present",
    "point_join_status",
    "role_review_status",
    "validation_result",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_PORTS = {"NLRTM", "BEANR", "ITGOA", "CHBSL", "DEDUI"}
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
        failures.append("EU port-node source-row validation columns do not match contract")
    if len(rows) != 5:
        failures.append("EU port-node source-row validation must have five rows")
    if {row["port_id"] for row in rows} != REQUIRED_PORTS:
        failures.append("EU port-node source-row validation missing required port IDs")
    for row in rows:
        if row["required_fields_present"] != "true":
            failures.append(f"{row['validation_id']} missing required fields")
        if row["point_join_status"] != "point_record_present_geometry_not_read":
            failures.append(f"{row['validation_id']} must preserve point join with geometry held")
        if row["role_review_status"] != "pass_with_holds":
            failures.append(f"{row['validation_id']} must require role review pass_with_holds")
        if row["validation_result"] != "candidate_attribute_row_validated_geometry_held":
            failures.append(f"{row['validation_id']} has unsupported validation result")
        if row["allowed_use"] != "internal node-candidate source-row validation only":
            failures.append(f"{row['validation_id']} has unsupported allowed use")
        if "before any internal node replacement" not in row["next_action"]:
            failures.append(f"{row['validation_id']} must preserve replacement dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['validation_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine port-node source-row validation gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine port-node source-row validation gate: PASS")
    print("  checked candidate attributes, role review dependency, geometry hold, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
