#!/usr/bin/env python3
"""Build EU Rhine-Alpine link-fixture replacement blocker."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LINKS = ROOT / "data" / "eu_rhine_alpine_source_link_candidates.csv"
EXTRACTION = ROOT / "data" / "international-eu-rhine-alpine-parser-extraction-candidates-001.csv"
METADATA_PROBE = ROOT / "data" / "international-eu-rhine-alpine-road-feature-metadata-probe-001.csv"
ENDPOINT_CANDIDATES = ROOT / "data" / "international-eu-rhine-alpine-road-link-endpoint-candidates-001.csv"
PAGE_LINKS = ROOT / "data" / "international-eu-rhine-alpine-gisco-transport-page-links-001.csv"
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-link-fixture-blocker-001.csv"

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

BLOCKED_CLAIMS = (
    "official_network;official_corridor_designation;member_state_approval;"
    "route_designation;source_row_validation;fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;endorsement;"
    "validation;external_validation;public_readiness;external_readiness;"
    "internal_adapter_proof"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def road_endpoint_status() -> str:
    endpoint_rows = read_csv(ENDPOINT_CANDIDATES) if ENDPOINT_CANDIDATES.exists() else []
    page_rows = read_csv(PAGE_LINKS) if PAGE_LINKS.exists() else []
    page_has_no_road = page_rows and all(row["link_family"] != "road_link_candidate" for row in page_rows)
    if endpoint_rows and all(row["endpoint_status"] != "candidate_reachable_not_accepted" for row in endpoint_rows):
        if page_has_no_road:
            return "official_page_scraped_candidates_probed_road_link_endpoint_missing"
        return "candidate_endpoints_probed_exact_road_link_endpoint_missing"
    rows = read_csv(METADATA_PROBE)
    road_rows = [row for row in rows if row["selected_for"] == "road_feature_probe"]
    if road_rows and all("endpoint" in row["next_action"] for row in road_rows):
        return "exact_road_link_endpoint_missing"
    return "road_endpoint_status_unknown"


def main() -> None:
    links = read_csv(LINKS)
    extraction = read_csv(EXTRACTION)
    rows = [
        {
            "blocker_id": "EUR-LINK-FIXTURE-BLOCKER-001",
            "replacement_target": "data/eu_rhine_alpine_source_link_candidates.csv",
            "current_link_rows": f"{len(links)} metadata/no-geometry dry-run rows",
            "source_content_rows": f"{len(extraction)} context extraction rows not promoted",
            "road_endpoint_status": road_endpoint_status(),
            "replacement_decision": "blocked_exact_road_link_endpoint_missing",
            "allowed_use": "gap tracking and source acquisition planning only",
            "blocked_claims": BLOCKED_CLAIMS,
            "required_next_step": "locate exact GISCO Transport version 3 road-link endpoint before source-row extraction and link fixture replacement",
        }
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
