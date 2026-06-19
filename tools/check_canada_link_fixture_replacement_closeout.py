#!/usr/bin/env python3
"""Gate Canada internal link-fixture replacement closeout."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CLOSEOUT = ROOT / "data" / "international-canada-link-fixture-replacement-closeout-001.csv"
LINKS = ROOT / "data" / "canada_source_link_candidates.csv"

FIELDS = [
    "closeout_id",
    "replacement_target",
    "replacement_source",
    "row_count",
    "source_row_validation_status",
    "geometry_contract",
    "replacement_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

REQUIRED_BLOCKS = {
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "parsed_adapter",
    "official_network",
    "route_designation",
    "engineering_precision",
    "agency_approval",
    "construction_ready",
    "guaranteed_sla",
    "roi",
    "eligibility",
    "compliance",
    "endorsement",
    "validation",
    "public_readiness",
    "external_readiness",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    fields, rows = read_csv(CLOSEOUT)
    _, links = read_csv(LINKS)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("link-fixture closeout columns do not match required contract")
    if len(rows) != 1:
        failures.append("link-fixture closeout must have exactly one row")
    row = rows[0] if rows else {}
    if row.get("replacement_status") != "internal_link_fixture_replaced":
        failures.append("link-fixture closeout does not record internal replacement")
    if row.get("allowed_use") != "internal parser link-candidate fixture rows only":
        failures.append("link-fixture closeout allowed use is too broad")
    if row.get("geometry_contract") != "no_geometry_candidate_rows_allowed":
        failures.append("link-fixture closeout does not preserve no-geometry contract")
    if row.get("row_count") != "5" or len(links) != 5:
        failures.append("link fixture must contain five source-derived rows")
    for index, link in enumerate(links, start=1):
        if not link["geometry_ref"].startswith("not_requested:"):
            failures.append(f"link row {index} accepted geometry")
        if "internal link fixture" not in link["access_note"]:
            failures.append(f"link row {index} missing internal fixture access note")
    blocked = set((row.get("blocked_claims") or "").split(";"))
    missing = REQUIRED_BLOCKS - blocked
    if missing:
        failures.append(f"link-fixture closeout missing blocked claims: {sorted(missing)}")
    if failures:
        print("Canada link-fixture replacement closeout gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Canada link-fixture replacement closeout gate: PASS")
    print("  checked internal replacement, no-geometry rows, allowed use, and claim holds")
    return 0


if __name__ == "__main__":
    sys.exit(main())
