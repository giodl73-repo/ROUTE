#!/usr/bin/env python3
"""Gate bounded EU Rhine-Alpine GISCO Ports 2013 node record sample."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-port-node-record-sample-001.csv"

FIELDS = [
    "sample_id",
    "source_id",
    "package_id",
    "port_id",
    "port_name",
    "country_code",
    "nuts_code",
    "ten_code",
    "port_hierarchy",
    "point_layer_join",
    "sample_reason",
    "evidence_acceptance_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_PORTS = {"NLRTM", "BEANR", "ITGOA", "CHBSL", "DEDUI"}
REQUIRED_NAMES = {"Rotterdam", "Antwerpen", "Genova", "Basel", "Duisburg"}
REQUIRED_BLOCKS = {
    "fixture_replacement",
    "internal_adapter_proof",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "terminal_performance",
    "node_completeness",
    "road_access_proof",
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
        failures.append("EU port node record sample columns do not match contract")
    if len(rows) != 5:
        failures.append("EU port node record sample must have five bounded rows")
    if {row["port_id"] for row in rows} != REQUIRED_PORTS:
        failures.append("EU port node record sample missing required anchor port IDs")
    if {row["port_name"] for row in rows} != REQUIRED_NAMES:
        failures.append("EU port node record sample missing required anchor port names")
    for row in rows:
        if row["source_id"] != "EUR-SRC-003":
            failures.append(f"{row['sample_id']} must trace to GISCO source")
        if row["package_id"] != "EUR-PORT-PACKAGE-002":
            failures.append(f"{row['sample_id']} must trace to SHP package")
        if row["point_layer_join"] != "point_record_present_geometry_not_read":
            failures.append(f"{row['sample_id']} must preserve point join with geometry held")
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['sample_id']} accepts evidence prematurely")
        if "geometry and node replacement held" not in row["allowed_use"]:
            failures.append(f"{row['sample_id']} must hold geometry and node replacement")
        if "before node fixture replacement" not in row["next_action"]:
            failures.append(f"{row['sample_id']} must preserve node replacement dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['sample_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine port node record sample gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine port node record sample gate: PASS")
    print("  checked bounded anchor records, point joins, geometry hold, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
