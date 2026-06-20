#!/usr/bin/env python3
"""Gate EU Rhine-Alpine road-link endpoint candidate probe."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-road-link-endpoint-candidates-001.csv"

FIELDS = [
    "candidate_id",
    "candidate_url",
    "candidate_basis",
    "probe_method",
    "http_status",
    "content_type",
    "content_length",
    "endpoint_status",
    "evidence_acceptance_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_BLOCKS = {
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
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("EU road-link endpoint candidate columns do not match contract")
    if len(rows) != 10:
        failures.append("EU road-link endpoint candidate probe must include ten candidate URLs")
    if not any("ROAD_2013_SH.zip" in row["candidate_url"] for row in rows):
        failures.append("EU road-link endpoint candidates must include road shapefile naming guess")
    if not any("transport-2013-sh" in row["candidate_url"] for row in rows):
        failures.append("EU road-link endpoint candidates must include document-route naming guess")
    for row in rows:
        if row["probe_method"] != "http-head":
            failures.append(f"{row['candidate_id']} must use bounded HEAD probe")
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['candidate_id']} accepts evidence prematurely")
        if row["allowed_use"] != "endpoint acquisition triage only":
            failures.append(f"{row['candidate_id']} allowed use is too broad")
        if row["endpoint_status"] == "candidate_reachable_not_accepted":
            failures.append(f"{row['candidate_id']} reached an endpoint; create a package-access gate before proceeding")
        if "before source-row extraction" not in row["next_action"]:
            failures.append(f"{row['candidate_id']} next action must preserve before-extraction dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['candidate_id']} missing blocked claims: {sorted(missing)}")

    if failures:
        print("EU Rhine-Alpine road-link endpoint candidate gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("EU Rhine-Alpine road-link endpoint candidate gate: PASS")
    print("  checked endpoint attempts, not-accepted posture, blocked claims, and extraction hold")
    return 0


if __name__ == "__main__":
    sys.exit(main())
