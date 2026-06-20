#!/usr/bin/env python3
"""Gate EU Rhine-Alpine road-link endpoint request packet preflight."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
REQUEST = ROOT / "data" / "international-eu-rhine-alpine-road-link-endpoint-request-001.csv"

FIELDS = [
    "request_id",
    "request_lane",
    "request_target",
    "request_basis_artifacts",
    "ask",
    "acceptable_response",
    "current_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_LANES = {
    "Eurostat GISCO support lane",
    "TENtec/Mobility and Transport source lane",
    "JRC EIGL documentation lane",
    "alternative public source lane",
}
REQUIRED_BLOCKS = {
    "named_contact",
    "agency_review",
    "official_network",
    "source_row_validation",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "guaranteed_sla",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
    "internal_adapter_proof",
}


def main() -> int:
    with REQUEST.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("EU road-link endpoint request columns do not match contract")
    lanes = {row["request_lane"] for row in rows}
    missing_lanes = REQUIRED_LANES - lanes
    if missing_lanes:
        failures.append(f"endpoint request missing lanes: {sorted(missing_lanes)}")
    if len(rows) != 4:
        failures.append("endpoint request packet must contain four request lanes")
    for row in rows:
        if "not_contacted" not in row["current_status"] and row["current_status"] != "alternative_not_selected":
            failures.append(f"{row['request_id']} overclaims contact status")
        if row["allowed_use"] not in {"source acquisition request planning only", "fallback source planning only"}:
            failures.append(f"{row['request_id']} allowed use is too broad")
        if "before source-row extraction" not in row["next_action"] and "before any fallback parser contract" not in row["next_action"]:
            failures.append(f"{row['request_id']} next action must preserve before dependency")
        for artifact in [
            "road-link-source-disposition",
            "gisco-transport-page-links",
            "road-link-endpoint-candidates",
            "link-fixture-blocker",
        ]:
            if artifact not in row["request_basis_artifacts"]:
                failures.append(f"{row['request_id']} missing basis artifact {artifact}")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['request_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine road-link endpoint request gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine road-link endpoint request gate: PASS")
    print("  checked request lanes, source basis, no-contact status, and claim holds")
    return 0


if __name__ == "__main__":
    sys.exit(main())
