#!/usr/bin/env python3
"""Build EU Rhine-Alpine road-link endpoint request packet preflight."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DISPOSITION = ROOT / "data" / "international-eu-rhine-alpine-road-link-source-disposition-001.csv"
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-road-link-endpoint-request-001.csv"

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

BLOCKED_CLAIMS = (
    "named_contact;agency_review;official_network;official_corridor_designation;"
    "member_state_approval;route_designation;source_row_validation;"
    "fixture_replacement;parsed_adapter;geometry_acceptance;topology_proof;"
    "map_overlay;construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;endorsement;"
    "validation;external_validation;public_readiness;external_readiness;"
    "internal_adapter_proof"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> None:
    disposition = read_csv(DISPOSITION)[0]
    basis = (
        "data/international-eu-rhine-alpine-road-link-source-disposition-001.csv;"
        "data/international-eu-rhine-alpine-gisco-transport-page-links-001.csv;"
        "data/international-eu-rhine-alpine-road-link-endpoint-candidates-001.csv;"
        "data/international-eu-rhine-alpine-link-fixture-blocker-001.csv"
    )
    rows = [
        {
            "request_id": "EUR-ROAD-LINK-REQ-001",
            "request_lane": "Eurostat GISCO support lane",
            "request_target": "official GISCO support or data-maintainer intake path",
            "request_basis_artifacts": basis,
            "ask": "Identify the official GISCO Transport version 3 road-link download/API endpoint and dataset metadata path, if publicly available.",
            "acceptable_response": "official URL plus dataset version, format, layer/table name, licensing/reuse note, and field documentation pointer",
            "current_status": "not_contacted_preflight_only",
            "allowed_use": "source acquisition request planning only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "send or route request before source-row extraction",
        },
        {
            "request_id": "EUR-ROAD-LINK-REQ-002",
            "request_lane": "TENtec/Mobility and Transport source lane",
            "request_target": "TENtec or Mobility and Transport public information intake path",
            "request_basis_artifacts": basis,
            "ask": "Clarify whether a public TENtec or European Transport Corridors road-network layer can provide no-geometry road-link attributes for adapter testing.",
            "acceptable_response": "official layer/API documentation or explicit statement that public road-link attributes are not available",
            "current_status": "not_contacted_preflight_only",
            "allowed_use": "source acquisition request planning only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "send or route request before source-row extraction",
        },
        {
            "request_id": "EUR-ROAD-LINK-REQ-003",
            "request_lane": "JRC EIGL documentation lane",
            "request_target": "JRC EIGL documentation follow-up path",
            "request_basis_artifacts": basis,
            "ask": "Confirm whether the documented GISCO Transport version 3 road links have a public distribution endpoint or only derived-model documentation availability.",
            "acceptable_response": "official endpoint, documentation pointer, or documented non-availability statement",
            "current_status": "not_contacted_preflight_only",
            "allowed_use": "source acquisition request planning only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "send or route request before source-row extraction",
        },
        {
            "request_id": "EUR-ROAD-LINK-REQ-004",
            "request_lane": "alternative public source lane",
            "request_target": "alternative source-selection review, not a replacement",
            "request_basis_artifacts": basis,
            "ask": "If official GISCO/TENtec road-link endpoint is unavailable, select an alternative public road-network source for a separate source-pack branch.",
            "acceptable_response": "new source selection row with owner, license, fields, geometry policy, role review, and no official-network claim boundary",
            "current_status": "alternative_not_selected",
            "allowed_use": "fallback source planning only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "create alternative source selection before any fallback parser contract",
        },
    ]
    if disposition["disposition"] != "official_endpoint_not_acquired_contact_or_alternative_source_required":
        raise RuntimeError("road-link disposition does not require endpoint request packet")
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
