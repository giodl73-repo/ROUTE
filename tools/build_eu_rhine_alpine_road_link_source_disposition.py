#!/usr/bin/env python3
"""Build EU Rhine-Alpine road-link source disposition."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PAGE_LINKS = ROOT / "data" / "international-eu-rhine-alpine-gisco-transport-page-links-001.csv"
ENDPOINTS = ROOT / "data" / "international-eu-rhine-alpine-road-link-endpoint-candidates-001.csv"
METADATA = ROOT / "data" / "international-eu-rhine-alpine-road-feature-metadata-probe-001.csv"
BLOCKER = ROOT / "data" / "international-eu-rhine-alpine-link-fixture-blocker-001.csv"
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-road-link-source-disposition-001.csv"

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

BLOCKED_CLAIMS = (
    "official_network;official_corridor_designation;member_state_approval;"
    "route_designation;source_row_validation;fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "construction_ready;guaranteed_sla;travel_time_proof;delivery_commitment;"
    "numeric_roi;roi;eligibility;compliance;endorsement;validation;"
    "external_validation;public_readiness;external_readiness;"
    "internal_adapter_proof"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> None:
    metadata_rows = read_csv(METADATA)
    page_rows = read_csv(PAGE_LINKS)
    endpoint_rows = read_csv(ENDPOINTS)
    blocker_rows = read_csv(BLOCKER)

    documentation_lead = any(
        row["probe_result"] == "documentation_confirms_gisco_transport_v3_road_links_candidate"
        for row in metadata_rows
    )
    page_has_road = any(row["link_family"] == "road_link_candidate" for row in page_rows)
    endpoint_reached = any(row["endpoint_status"] == "candidate_reachable_not_accepted" for row in endpoint_rows)
    blocker_status = blocker_rows[0]["replacement_decision"] if blocker_rows else "missing_blocker"

    rows = [
        {
            "disposition_id": "EUR-ROAD-LINK-DISPOSITION-001",
            "source_family": "Eurostat GISCO Transport version 3 road links",
            "documentation_lead_status": "lead_exists_not_endpoint" if documentation_lead else "lead_missing",
            "official_page_status": "road_link_not_exposed" if not page_has_road else "road_link_candidate_exposed",
            "candidate_endpoint_status": "direct_candidates_not_found" if not endpoint_reached else "candidate_endpoint_reached",
            "fixture_replacement_status": blocker_status,
            "disposition": "official_endpoint_not_acquired_contact_or_alternative_source_required",
            "allowed_use": "source acquisition planning and gap explanation only",
            "blocked_claims": BLOCKED_CLAIMS,
            "required_next_step": "request or locate official road-link endpoint before source-row extraction and link fixture replacement",
        }
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
