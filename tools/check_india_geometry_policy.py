#!/usr/bin/env python3
"""Gate India geometry policy before map or fixture use."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
POLICY = ROOT / "data" / "international-india-geometry-policy-001.csv"

FIELDS = [
    "policy_id",
    "surface",
    "current_geometry_status",
    "decision",
    "required_before_acceptance",
    "blocked_uses",
    "blocked_claims",
    "next_action",
]
REQUIRED_BLOCKS = {
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "fixture_replacement",
    "parsed_adapter",
    "official_network",
    "official_corridor_designation",
    "national_approval",
    "state_approval",
    "route_designation",
    "engineering_precision",
    "construction_ready",
    "guaranteed_sla",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
    "internal_adapter_proof",
}


def main() -> int:
    with POLICY.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("India geometry policy columns do not match contract")
    if len(rows) != 3:
        failures.append("India geometry policy must cover dry-run, map/media, and replacement surfaces")
    for row in rows:
        if row["current_geometry_status"] != "not_requested":
            failures.append(f"{row['policy_id']} accepts geometry")
        if "accept" in row["decision"] and "reject" not in row["decision"]:
            failures.append(f"{row['policy_id']} promotes geometry acceptance")
        if not row["required_before_acceptance"]:
            failures.append(f"{row['policy_id']} missing acceptance prerequisites")
        if "fixture replacement" not in row["blocked_uses"] and "fixture_replacement" not in row["blocked_uses"]:
            failures.append(f"{row['policy_id']} must block fixture replacement use")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['policy_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("India geometry policy gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("India geometry policy gate: PASS")
    print("  checked no-geometry posture, acceptance prerequisites, blocked uses, and claim holds")
    return 0


if __name__ == "__main__":
    sys.exit(main())
