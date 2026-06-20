#!/usr/bin/env python3
"""Gate bounded EU Rhine-Alpine source-content sample rows."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-eu-rhine-alpine-source-content-sample-001.csv"

FIELDS = [
    "sample_id",
    "source_id",
    "source_family",
    "source_url",
    "source_line_ref",
    "content_summary",
    "route_or_dataset_hint",
    "source_owner",
    "source_date",
    "sample_status",
    "evidence_label",
    "blocked_claims",
    "next_action",
]
REQUIRED_SOURCES = {"EUR-SRC-001", "EUR-SRC-002", "EUR-SRC-003", "EUR-SRC-004"}
REQUIRED_BLOCKS = {
    "geometry_acceptance",
    "guaranteed_sla",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
    "internal_adapter_proof",
}


def main() -> int:
    with SAMPLE.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("EU source-content sample columns do not match contract")
    if {row["source_id"] for row in rows} != REQUIRED_SOURCES:
        failures.append("EU source-content sample must cover the four content-bearing source rows")
    if not any(row["route_or_dataset_hint"] == "current_corridor_set_rebase_needed" for row in rows):
        failures.append("EU sample must preserve current-corridor rebase warning")
    if not any("Rotterdam" in row["content_summary"] and "Genoa" in row["content_summary"] for row in rows):
        failures.append("EU sample must preserve bounded Rhine-Alpine endpoint context")
    for row in rows:
        if row["evidence_label"] != "source-candidate":
            failures.append(f"{row['sample_id']} has unsupported evidence label")
        if not row["source_url"].startswith("https://"):
            failures.append(f"{row['sample_id']} missing source URL")
        if "before" not in row["next_action"]:
            failures.append(f"{row['sample_id']} next action must preserve before dependency")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['sample_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("EU Rhine-Alpine source-content sample gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("EU Rhine-Alpine source-content sample gate: PASS")
    print("  checked source coverage, rebase warning, bounded context, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
