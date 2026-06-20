#!/usr/bin/env python3
"""Build EU Rhine-Alpine GISCO Ports 2013 package-access metadata ledger."""

from __future__ import annotations

import csv
import urllib.error
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-port-package-access-001.csv"
USER_AGENT = "ROUTE-EU-GISCO-port-package-access/0.1 evidence-not-accepted"
TIMEOUT_SECONDS = 20

FIELDS = [
    "package_id",
    "metadata_probe_id",
    "source_id",
    "package_format",
    "package_url",
    "http_method",
    "http_status",
    "content_type",
    "content_length_bytes",
    "access_result",
    "evidence_acceptance_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "official_network;official_corridor_designation;member_state_approval;"
    "route_designation;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;"
    "endorsement;validation;external_validation;public_readiness;"
    "external_readiness;fixture_replacement;internal_adapter_proof"
)

PACKAGES = [
    (
        "EUR-PORT-PACKAGE-001",
        "gdb_zip",
        "https://ec.europa.eu/eurostat/cache/GISCO/geodatafiles/PORT_2013.zip",
    ),
    (
        "EUR-PORT-PACKAGE-002",
        "shp_zip",
        "https://ec.europa.eu/eurostat/cache/GISCO/geodatafiles/PORT_2013_SH.zip",
    ),
]


def head(url: str) -> tuple[str, str, str, str]:
    request = urllib.request.Request(url, method="HEAD", headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=TIMEOUT_SECONDS) as response:
            status = str(response.status)
            content_type = response.headers.get("content-type", "unknown")
            content_length = response.headers.get("content-length", "unknown")
            return status, content_type, content_length, "head_reachable_not_downloaded"
    except urllib.error.HTTPError as exc:
        return str(exc.code), exc.headers.get("content-type", "unknown"), "unknown", "head_http_error_not_accepted"
    except Exception as exc:  # noqa: BLE001 - access probes record bounded failures.
        return "none", "unknown", "unknown", f"head_error_not_accepted:{type(exc).__name__}"


def main() -> None:
    rows: list[dict[str, str]] = []
    for package_id, package_format, url in PACKAGES:
        status, content_type, content_length, result = head(url)
        rows.append(
            {
                "package_id": package_id,
                "metadata_probe_id": "EUR-METADATA-PROBE-003",
                "source_id": "EUR-SRC-003",
                "package_format": package_format,
                "package_url": url,
                "http_method": "HEAD",
                "http_status": status,
                "content_type": content_type,
                "content_length_bytes": content_length,
                "access_result": result,
                "evidence_acceptance_status": "not-accepted",
                "allowed_use": "package-access metadata only; no download or geometry acceptance",
                "blocked_claims": BLOCKED,
                "next_action": "inspect package manifest and fields before node fixture replacement",
            }
        )
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
