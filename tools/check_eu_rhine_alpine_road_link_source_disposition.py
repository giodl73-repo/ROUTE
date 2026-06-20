#!/usr/bin/env python3
"""Gate EU Rhine-Alpine road-link source disposition."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DISPOSITION = ROOT / "data" / "international-eu-rhine-alpine-road-link-source-disposition-001.csv"

FIELDS = [
    "disposition_id",
    "source_family",
    "documentation_lead_status",
    "official_page_status",
    "candidate_endpoint_status",
    "fixture_replacement_status",
    "disposition",
    "allowed_use",
    "blocked_claims",
    "required_next_step",
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
    with DISPOSITION.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("EU road-link source disposition columns do not match contract")
    if len(rows) != 1:
        failures.append("EU road-link source disposition must have one row")
    row = rows[0] if rows else {}
    if row.get("documentation_lead_status") != "lead_exists_not_endpoint":
        failures.append("road-link disposition must preserve documentation lead without endpoint")
    if row.get("official_page_status") != "road_link_not_exposed":
        failures.append("road-link disposition must preserve official-page absence")
    if row.get("candidate_endpoint_status") != "direct_candidates_not_found":
        failures.append("road-link disposition must preserve direct candidate failure")
    if row.get("fixture_replacement_status") != "blocked_exact_road_link_endpoint_missing":
        failures.append("road-link disposition must keep fixture replacement blocked")
    if row.get("allowed_use") != "source acquisition planning and gap explanation only":
        failures.append("road-link disposition allowed use is too broad")
    if "before source-row extraction" not in row.get("required_next_step", ""):
        failures.append("road-link disposition must preserve before-extraction dependency")
    missing = REQUIRED_BLOCKS - set((row.get("blocked_claims") or "").split(";"))
    if missing:
        failures.append(f"road-link disposition missing blocked claims: {sorted(missing)}")

    if failures:
        print("EU Rhine-Alpine road-link source disposition gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("EU Rhine-Alpine road-link source disposition gate: PASS")
    print("  checked documentation lead, page absence, endpoint misses, blocker status, and claim holds")
    return 0


if __name__ == "__main__":
    sys.exit(main())
