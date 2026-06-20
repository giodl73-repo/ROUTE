#!/usr/bin/env python3
"""Probe EU Rhine-Alpine road-link endpoint candidates without accepting evidence."""

from __future__ import annotations

import csv
import sys
import urllib.error
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-road-link-endpoint-candidates-001.csv"
USER_AGENT = "ROUTE-EU-Rhine-Alpine-road-link-endpoint-probe/0.1 evidence-not-accepted"
TIMEOUT_SECONDS = 20

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

BLOCKED_CLAIMS = (
    "official_network;official_corridor_designation;member_state_approval;"
    "route_designation;source_row_validation;fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "construction_ready;guaranteed_sla;travel_time_proof;delivery_commitment;"
    "numeric_roi;roi;eligibility;compliance;endorsement;validation;"
    "external_validation;public_readiness;external_readiness;"
    "internal_adapter_proof"
)

CANDIDATES = [
    (
        "EUR-ROAD-ENDPOINT-001",
        "https://ec.europa.eu/eurostat/cache/GISCO/geodatafiles/TRANSPORT_2013.zip",
        "geodatafiles naming guess from GISCO transport source family",
    ),
    (
        "EUR-ROAD-ENDPOINT-002",
        "https://ec.europa.eu/eurostat/cache/GISCO/geodatafiles/TRANSPORT_2013_SH.zip",
        "geodatafiles shapefile naming guess from GISCO transport source family",
    ),
    (
        "EUR-ROAD-ENDPOINT-003",
        "https://ec.europa.eu/eurostat/cache/GISCO/geodatafiles/TRAN_2013.zip",
        "abbreviated transport naming guess",
    ),
    (
        "EUR-ROAD-ENDPOINT-004",
        "https://ec.europa.eu/eurostat/cache/GISCO/geodatafiles/TRAN_2013_SH.zip",
        "abbreviated transport shapefile naming guess",
    ),
    (
        "EUR-ROAD-ENDPOINT-005",
        "https://ec.europa.eu/eurostat/cache/GISCO/geodatafiles/ROAD_2013.zip",
        "road-link package naming guess",
    ),
    (
        "EUR-ROAD-ENDPOINT-006",
        "https://ec.europa.eu/eurostat/cache/GISCO/geodatafiles/ROAD_2013_SH.zip",
        "road-link shapefile naming guess",
    ),
    (
        "EUR-ROAD-ENDPOINT-007",
        "https://ec.europa.eu/eurostat/cache/GISCO/geodatafiles/RD_2013.zip",
        "abbreviated road package naming guess",
    ),
    (
        "EUR-ROAD-ENDPOINT-008",
        "https://ec.europa.eu/eurostat/cache/GISCO/geodatafiles/RD_2013_SH.zip",
        "abbreviated road shapefile naming guess",
    ),
    (
        "EUR-ROAD-ENDPOINT-009",
        "https://ec.europa.eu/eurostat/documents/d/gisco/transport-2013-sh",
        "new Eurostat document route naming guess",
    ),
    (
        "EUR-ROAD-ENDPOINT-010",
        "https://ec.europa.eu/eurostat/documents/d/gisco/road-2013-sh",
        "new Eurostat road document route naming guess",
    ),
]


def probe_head(url: str) -> tuple[str, str, str, str]:
    request = urllib.request.Request(url, method="HEAD", headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=TIMEOUT_SECONDS) as response:
            return (
                str(response.status),
                response.headers.get("content-type", "unknown"),
                response.headers.get("content-length", ""),
                "candidate_reachable_not_accepted",
            )
    except urllib.error.HTTPError as exc:
        return (
            str(exc.code),
            exc.headers.get("content-type", "unknown"),
            exc.headers.get("content-length", ""),
            "candidate_not_found_not_accepted" if exc.code == 404 else "candidate_http_error_not_accepted",
        )
    except Exception as exc:  # noqa: BLE001 - endpoint probes record failures without failing generation.
        return "none", "unknown", "", f"candidate_probe_error_not_accepted:{type(exc).__name__}"


def main() -> int:
    rows: list[dict[str, str]] = []
    for candidate_id, url, basis in CANDIDATES:
        status, content_type, content_length, endpoint_status = probe_head(url)
        rows.append(
            {
                "candidate_id": candidate_id,
                "candidate_url": url,
                "candidate_basis": basis,
                "probe_method": "http-head",
                "http_status": status,
                "content_type": content_type,
                "content_length": content_length,
                "endpoint_status": endpoint_status,
                "evidence_acceptance_status": "not-accepted",
                "allowed_use": "endpoint acquisition triage only",
                "blocked_claims": BLOCKED_CLAIMS,
                "next_action": "find an official GISCO road-link endpoint before source-row extraction",
            }
        )

    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
