#!/usr/bin/env python3
"""Gate EU Rhine-Alpine link-fixture replacement blocker."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BLOCKER = ROOT / "data" / "international-eu-rhine-alpine-link-fixture-blocker-001.csv"
LINKS = ROOT / "data" / "eu_rhine_alpine_source_link_candidates.csv"
EXTRACTION = ROOT / "data" / "international-eu-rhine-alpine-parser-extraction-candidates-001.csv"

FIELDS = [
    "blocker_id",
    "replacement_target",
    "current_link_rows",
    "source_content_rows",
    "road_endpoint_status",
    "replacement_decision",
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


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    fields, rows = read_csv(BLOCKER)
    _, link_rows = read_csv(LINKS)
    _, extraction_rows = read_csv(EXTRACTION)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("EU link-fixture blocker columns do not match contract")
    if len(rows) != 1:
        failures.append("EU link-fixture blocker must contain one row")
    if len(link_rows) != 2:
        failures.append("EU link fixture blocker expects current two dry-run link rows")
    for row in link_rows:
        if row["evidence_label"] != "source-candidate":
            failures.append(f"{row['route_or_layer_id']} link row is not source-candidate")
        if not row["geometry_ref"].startswith("not_accepted:"):
            failures.append(f"{row['route_or_layer_id']} accepted geometry")
    for row in extraction_rows:
        if row["candidate_status"] != "source_content_extraction_candidate_not_promoted":
            failures.append(f"{row['candidate_id']} source-content row was promoted")
    for row in rows:
        if row["road_endpoint_status"] != "exact_road_link_endpoint_missing":
            failures.append("EU link blocker must preserve missing road endpoint status")
        if row["replacement_decision"] != "blocked_exact_road_link_endpoint_missing":
            failures.append("EU link blocker must block replacement")
        if row["allowed_use"] != "gap tracking and source acquisition planning only":
            failures.append("EU link blocker allowed use is too broad")
        if "before source-row extraction" not in row["required_next_step"]:
            failures.append("EU link blocker must preserve before-extraction dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"EU link blocker missing blocked claims: {sorted(missing)}")

    if failures:
        print("EU Rhine-Alpine link fixture blocker gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("EU Rhine-Alpine link fixture blocker gate: PASS")
    print("  checked blocked replacement, endpoint dependency, no-geometry rows, and claim holds")
    return 0


if __name__ == "__main__":
    sys.exit(main())
