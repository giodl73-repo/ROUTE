#!/usr/bin/env python3
"""Build EU Rhine-Alpine GISCO Ports 2013 package manifest ledger."""

from __future__ import annotations

import csv
import io
import urllib.request
import zipfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-port-package-manifest-001.csv"
USER_AGENT = "ROUTE-EU-GISCO-port-package-manifest/0.1 evidence-not-accepted"
TIMEOUT_SECONDS = 30

FIELDS = [
    "manifest_id",
    "package_id",
    "package_format",
    "package_url",
    "bytes_read",
    "manifest_result",
    "key_entries",
    "dbf_fields",
    "geometry_files_present",
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


def fetch_zip(url: str) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(request, timeout=TIMEOUT_SECONDS) as response:
        return response.read()


def dbf_fields(dbf: bytes) -> str:
    header_len = int.from_bytes(dbf[8:10], "little")
    fields: list[str] = []
    pos = 32
    while pos < header_len - 1:
        desc = dbf[pos : pos + 32]
        if desc[0] == 0x0D:
            break
        name = desc[:11].split(b"\x00", 1)[0].decode("ascii", "replace")
        typ = chr(desc[11])
        length = desc[16]
        fields.append(f"{name}:{typ}:{length}")
        pos += 32
    return ";".join(fields)


def main() -> None:
    rows: list[dict[str, str]] = []
    for index, (package_id, package_format, url) in enumerate(PACKAGES, start=1):
        payload = fetch_zip(url)
        with zipfile.ZipFile(io.BytesIO(payload)) as zf:
            names = zf.namelist()
            if package_format == "shp_zip":
                key_entries = [
                    name
                    for name in names
                    if name.endswith(
                        (
                            "PORT_PT_2013.dbf",
                            "PORT_PT_2013.shp",
                            "PORT_PT_2013.shx",
                            "PORT_PT_2013.prj",
                            "PORT_AT_2013.dbf",
                        )
                    )
                ]
                fields = dbf_fields(zf.read("PORT_2013_SH/Data/PORT_PT_2013.dbf"))
                geometry_files = "PORT_PT_2013.shp;PORT_PT_2013.shx;PORT_PT_2013.prj"
                result = "point_layer_manifest_and_dbf_header_read"
                next_action = "map DBF fields to node candidate contract before node fixture replacement"
            else:
                key_entries = [
                    name
                    for name in names
                    if "PORT_2013.gdb/" in name and (name.endswith(".gdbtable") or name.endswith(".spx"))
                ][:8]
                fields = "not-inspected:gdb-binary-manifest-only"
                geometry_files = "gdb-binary-files-present-not-read"
                result = "gdb_manifest_read_fields_not_inspected"
                next_action = "prefer SHP DBF header for field mapping before node fixture replacement"
        rows.append(
            {
                "manifest_id": f"EUR-PORT-MANIFEST-{index:03d}",
                "package_id": package_id,
                "package_format": package_format,
                "package_url": url,
                "bytes_read": str(len(payload)),
                "manifest_result": result,
                "key_entries": ";".join(key_entries),
                "dbf_fields": fields,
                "geometry_files_present": geometry_files,
                "evidence_acceptance_status": "not-accepted",
                "allowed_use": "manifest and DBF-header inspection only; geometry not read or accepted",
                "blocked_claims": BLOCKED,
                "next_action": next_action,
            }
        )
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
