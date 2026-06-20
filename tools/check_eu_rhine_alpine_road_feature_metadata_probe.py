#!/usr/bin/env python3
"""Gate EU Rhine-Alpine road-feature and port-node metadata probe ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-road-feature-metadata-probe-001.csv"

FIELDS = [
    "probe_id",
    "selection_id",
    "source_id",
    "selected_for",
    "probe_surface",
    "probe_url",
    "probe_result",
    "observed_metadata",
    "evidence_acceptance_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_RESULTS = {
    "reachable_page_no_direct_road_link_table_in_sample",
    "documentation_confirms_gisco_transport_v3_road_links_candidate",
    "metadata_confirms_ports_2013_download_options",
    "scope_context_only_current_corridor_rebase_still_required",
}
REQUIRED_BLOCKS = {
    "fixture_replacement",
    "internal_adapter_proof",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "terminal_performance",
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
        failures.append("EU road-feature metadata probe columns do not match contract")
    if len(rows) != 4:
        failures.append("EU road-feature metadata probe must have four rows")
    if {row["probe_result"] for row in rows} != REQUIRED_RESULTS:
        failures.append("EU road-feature metadata probe results mismatch")
    if not any("Transport version 3" in row["observed_metadata"] and "Road links" in row["observed_metadata"] for row in rows):
        failures.append("EU road-feature metadata probe must preserve Transport v3 road-link lead")
    if not any("Ports 2013" in row["observed_metadata"] and "SHP" in row["observed_metadata"] for row in rows):
        failures.append("EU road-feature metadata probe must preserve Ports 2013 package lead")
    for row in rows:
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['probe_id']} accepts evidence prematurely")
        if not row["probe_url"].startswith("https://"):
            failures.append(f"{row['probe_id']} missing probe URL")
        if "before" not in row["next_action"]:
            failures.append(f"{row['probe_id']} next action must preserve before dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['probe_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine road-feature metadata probe gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine road-feature metadata probe gate: PASS")
    print("  checked road-link lead, port package lead, scope boundary, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
