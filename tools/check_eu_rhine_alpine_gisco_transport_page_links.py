#!/usr/bin/env python3
"""Gate GISCO transport-network page link scrape."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-gisco-transport-page-links-001.csv"

FIELDS = [
    "link_id",
    "page_url",
    "link_url",
    "link_family",
    "link_status",
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
        failures.append("GISCO page-link ledger columns do not match contract")
    if len(rows) < 8:
        failures.append("GISCO page-link ledger should expose transport-page package links")
    if not any("PORT_2013_SH.zip" in row["link_url"] for row in rows):
        failures.append("GISCO page-link ledger must preserve visible Ports 2013 SHP link")
    if not any("AIRP_SH.zip" in row["link_url"] for row in rows):
        failures.append("GISCO page-link ledger must preserve visible airport SHP link")
    if any(row["link_family"] == "road_link_candidate" for row in rows):
        failures.append("GISCO page-link ledger found road candidate; endpoint acquisition gate must be promoted")
    for row in rows:
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['link_id']} accepts evidence prematurely")
        if row["allowed_use"] != "official page link inventory only":
            failures.append(f"{row['link_id']} allowed use is too broad")
        if "before source-row extraction" not in row["next_action"]:
            failures.append(f"{row['link_id']} next action must preserve before-extraction dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['link_id']} missing blocked claims: {sorted(missing)}")

    if failures:
        print("EU Rhine-Alpine GISCO transport page-link gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("EU Rhine-Alpine GISCO transport page-link gate: PASS")
    print("  checked official page links, port/airport exposure, road-link absence, and claim holds")
    return 0


if __name__ == "__main__":
    sys.exit(main())
