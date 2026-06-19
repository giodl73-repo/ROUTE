#!/usr/bin/env python3
"""Gate the Canada adapter promotion preflight ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PREFLIGHT = ROOT / "data" / "international-canada-adapter-promotion-preflight-001.csv"
LINK_FIXTURE = ROOT / "data" / "canada_source_link_candidates.csv"
CLOSEOUT = ROOT / "data" / "international-canada-link-fixture-replacement-closeout-001.csv"

FIELDS = [
    "preflight_id",
    "promotion_surface",
    "closed_inputs",
    "current_decision",
    "blocker",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

REQUIRED_SURFACES = {
    "link_candidate_fixture",
    "geometry_topology",
    "need_node_target_tables",
    "authority_operational_public_use",
    "promotion_decision",
}

REQUIRED_BLOCKS = {
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "official_network",
    "route_designation",
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
    fields, rows = read_csv(PREFLIGHT)
    _, link_rows = read_csv(LINK_FIXTURE)
    _, closeout_rows = read_csv(CLOSEOUT)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("adapter promotion preflight columns do not match required contract")
    if len(rows) != 5:
        failures.append("adapter promotion preflight must have five surface rows")

    surfaces = {row["promotion_surface"] for row in rows}
    missing_surfaces = REQUIRED_SURFACES - surfaces
    if missing_surfaces:
        failures.append(f"missing promotion surfaces: {sorted(missing_surfaces)}")

    ready_rows = [row for row in rows if row["current_decision"] == "internal_link_fixture_ready"]
    if len(ready_rows) != 1:
        failures.append("expected exactly one internal_link_fixture_ready row")
    elif ready_rows[0]["promotion_surface"] != "link_candidate_fixture":
        failures.append("internal_link_fixture_ready row must be link_candidate_fixture")

    promotion_rows = [row for row in rows if row["promotion_surface"] == "promotion_decision"]
    if len(promotion_rows) != 1:
        failures.append("expected exactly one promotion_decision row")
    elif promotion_rows[0]["current_decision"] != "parsed_adapter_promotion_held":
        failures.append("promotion_decision must hold parsed adapter promotion")

    for row in rows:
        if row["current_decision"] == "parsed_adapter_promoted":
            failures.append(f"{row['preflight_id']} promoted a parsed adapter")
        if row["allowed_use"] not in {"none", "internal parser link-candidate fixture rows only"}:
            failures.append(f"{row['preflight_id']} allowed use is too broad")
        blocked = set(row["blocked_claims"].split(";"))
        missing_blocks = REQUIRED_BLOCKS - blocked
        if missing_blocks:
            failures.append(f"{row['preflight_id']} missing blocked claims: {sorted(missing_blocks)}")

    if len(closeout_rows) != 1:
        failures.append("link fixture replacement closeout must contain one closeout row")
    elif closeout_rows[0]["replacement_status"] != "internal_link_fixture_replaced":
        failures.append("link fixture replacement closeout is not replaced")

    if len(link_rows) != 5:
        failures.append("link fixture must contain five source-derived rows")
    for index, row in enumerate(link_rows, start=1):
        if not row["geometry_ref"].startswith("not_requested:"):
            failures.append(f"link fixture row {index} accepted geometry")
        if "source-derived no-geometry internal link fixture" not in row["access_note"]:
            failures.append(f"link fixture row {index} missing source-derived internal fixture note")

    if failures:
        print("Canada adapter promotion preflight gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada adapter promotion preflight gate: PASS")
    print("  checked surfaces, link-fixture readiness, promotion hold, and claim blocks")
    return 0


if __name__ == "__main__":
    sys.exit(main())
