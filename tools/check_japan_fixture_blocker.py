#!/usr/bin/env python3
"""Gate Japan fixture replacement blocker."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BLOCKER = ROOT / "data" / "international-japan-fixture-blocker-001.csv"

FIELDS = [
    "blocker_id",
    "replacement_target",
    "current_rows",
    "role_review_status",
    "geometry_status",
    "replacement_decision",
    "allowed_use",
    "blocked_claims",
    "required_next_step",
]
REQUIRED_BLOCKS = {
    "source_row_validation",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "disaster_readiness",
    "guaranteed_sla",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
    "internal_adapter_proof",
}


def main() -> int:
    with BLOCKER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("Japan fixture blocker columns do not match contract")
    if len(rows) != 1:
        failures.append("Japan fixture blocker must have one blocker row")
    for row in rows:
        if row["role_review_status"] != "pass_with_holds":
            failures.append("Japan fixture blocker requires pass-with-holds role review")
        if row["geometry_status"] != "geometry_not_requested_policy_blocks_replacement":
            failures.append("Japan fixture blocker requires no-geometry policy hold")
        if row["replacement_decision"] != "blocked_source_rows_not_validated_gsi_link_source_needed_geometry_not_accepted":
            failures.append("Japan fixture blocker must block replacement on source rows, GSI, and geometry")
        if row["allowed_use"] != "gap tracking and source acquisition planning only":
            failures.append("Japan fixture blocker allowed use is too broad")
        if "before any fixture replacement contract" not in row["required_next_step"]:
            failures.append("Japan fixture blocker must preserve before-contract dependency")
        if "source_content_blocker" not in row["current_rows"]:
            failures.append("Japan fixture blocker must preserve source-needed content blocker evidence")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"Japan fixture blocker missing blocked claims: {sorted(missing)}")
    if failures:
        print("Japan fixture blocker gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Japan fixture blocker gate: PASS")
    print("  checked role holds, GSI/source-row blocker, no-geometry status, replacement block, and claim holds")
    return 0


if __name__ == "__main__":
    sys.exit(main())
