#!/usr/bin/env python3
"""Gate EU Rhine-Alpine GISCO Ports 2013 node field-mapping ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-eu-rhine-alpine-port-node-field-mapping-001.csv"

FIELDS = [
    "mapping_id",
    "source_table",
    "source_field",
    "candidate_contract_field",
    "mapping_status",
    "evidence_source",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_CONTRACT_FIELDS = {
    "node_source_id",
    "node_name",
    "jurisdiction_context",
    "source_custody_note",
    "geometry_ref",
    "node_class_context",
}
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
        failures.append("EU port node field mapping columns do not match contract")
    if len(rows) != 6:
        failures.append("EU port node field mapping must have six rows")
    if {row["candidate_contract_field"] for row in rows} != REQUIRED_CONTRACT_FIELDS:
        failures.append("EU port node field mapping must cover required candidate fields")
    for row in rows:
        if "record_validated" in row["mapping_status"]:
            failures.append(f"{row['mapping_id']} overclaims record validation")
        if row["candidate_contract_field"] == "geometry_ref":
            if row["mapping_status"] != "geometry_present_not_read_or_accepted":
                failures.append("geometry row must remain not read or accepted")
        elif not row["mapping_status"].startswith("field_header_mappable"):
            failures.append(f"{row['mapping_id']} must remain field-header mapping only")
        if "before" not in row["next_action"]:
            failures.append(f"{row['mapping_id']} next action must preserve before dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['mapping_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine port node field mapping gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine port node field mapping gate: PASS")
    print("  checked field mappings, header-only posture, geometry hold, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
