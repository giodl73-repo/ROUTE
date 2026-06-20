#!/usr/bin/env python3
"""Gate EU Rhine-Alpine parser extraction candidates before fixture replacement."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CANDIDATES = ROOT / "data" / "international-eu-rhine-alpine-parser-extraction-candidates-001.csv"

FIELDS = [
    "candidate_id",
    "source_id",
    "source_family",
    "route_or_layer_id",
    "route_or_layer_name",
    "source_class",
    "geometry_ref",
    "source_owner",
    "source_date",
    "access_note",
    "evidence_label",
    "candidate_status",
    "blocked_claims",
    "next_action",
]
REQUIRED_BLOCKS = {
    "geometry_acceptance",
    "guaranteed_sla",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
}


def main() -> int:
    with CANDIDATES.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("EU parser extraction candidate columns do not match contract")
    if len(rows) != 2:
        failures.append("EU parser extraction candidate table must have two bounded rows")
    if not any(row["route_or_layer_name"] == "Rhine - Alpine corridor context" for row in rows):
        failures.append("EU extraction candidates must preserve bounded Rhine-Alpine context")
    for row in rows:
        if row["evidence_label"] != "source-candidate":
            failures.append(f"{row['candidate_id']} has wrong evidence label")
        if row["candidate_status"] != "source_content_extraction_candidate_not_promoted":
            failures.append(f"{row['candidate_id']} promotes candidate status")
        if not row["geometry_ref"].startswith("not_requested:"):
            failures.append(f"{row['candidate_id']} accepts geometry")
        if "before" not in row["next_action"]:
            failures.append(f"{row['candidate_id']} next action must preserve before dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['candidate_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine parser extraction candidate gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine parser extraction candidate gate: PASS")
    print("  checked candidate values, no-geometry posture, labels, and not-promoted status")
    return 0


if __name__ == "__main__":
    sys.exit(main())
