#!/usr/bin/env python3
"""Scrape GISCO transport-network page links without accepting evidence."""

from __future__ import annotations

import csv
import html
import re
import sys
import urllib.parse
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-gisco-transport-page-links-001.csv"
PAGE_URL = "https://ec.europa.eu/eurostat/web/gisco/geodata/transport-networks"
USER_AGENT = "ROUTE-EU-Rhine-Alpine-gisco-link-scrape/0.1 evidence-not-accepted"
TIMEOUT_SECONDS = 20

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

BLOCKED_CLAIMS = (
    "official_network;official_corridor_designation;member_state_approval;"
    "route_designation;source_row_validation;fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "construction_ready;guaranteed_sla;travel_time_proof;delivery_commitment;"
    "numeric_roi;roi;eligibility;compliance;endorsement;validation;"
    "external_validation;public_readiness;external_readiness;"
    "internal_adapter_proof"
)


def classify(url: str) -> str:
    upper = url.upper()
    lower = url.lower()
    if "AIRP" in upper or "AIRPORT" in upper:
        return "airport_package_link"
    if "PORT_" in upper or "/PORT-" in upper:
        return "port_package_link"
    if "ROAD" in upper or "ROAD-LINK" in lower:
        return "road_link_candidate"
    return "other_gisco_transport_page_link"


def main() -> int:
    request = urllib.request.Request(PAGE_URL, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(request, timeout=TIMEOUT_SECONDS) as response:
        page = response.read().decode("utf-8", errors="replace")

    links = sorted(
        {
            urllib.parse.urljoin(PAGE_URL, html.unescape(match))
            for match in re.findall(r"href=[\"']([^\"']+)[\"']", page, flags=re.IGNORECASE)
        }
    )
    relevant = [
        link
        for link in links
        if "geodatafiles" in link
        or "/documents/d/gisco/" in link
        or link.rstrip("/") == PAGE_URL
    ]

    rows: list[dict[str, str]] = []
    for index, link in enumerate(relevant, start=1):
        family = classify(link)
        rows.append(
            {
                "link_id": f"EUR-GISCO-LINK-{index:03d}",
                "page_url": PAGE_URL,
                "link_url": link,
                "link_family": family,
                "link_status": "road_endpoint_not_exposed" if family != "road_link_candidate" else "road_candidate_visible_not_accepted",
                "evidence_acceptance_status": "not-accepted",
                "allowed_use": "official page link inventory only",
                "blocked_claims": BLOCKED_CLAIMS,
                "next_action": "find an official road-link endpoint before source-row extraction",
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
