#!/usr/bin/env python3
"""Gate Canada source-payload access manifest before source parsing."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE_PACK = ROOT / "data" / "international-canada-adapter-source-pack-001.csv"
ACCESS = ROOT / "data" / "international-canada-source-payload-access-001.csv"

REQUIRED_FIELDS = [
    "payload_access_id",
    "source_id",
    "source_family",
    "payload_url_or_status",
    "owner_or_publisher",
    "cache_target",
    "access_mode",
    "payload_status",
    "live_fetch_status",
    "parser_task_id",
    "required_fields",
    "post_access_gate",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    source_fields, source_rows = read_csv(SOURCE_PACK)
    fields, rows = read_csv(ACCESS)
    failures: list[str] = []

    if fields != REQUIRED_FIELDS:
        failures.append("payload access columns do not match required contract")
    if len(rows) != len(source_rows):
        failures.append("payload access row count does not match Canada source pack")

    source_ids = {row["source_id"] for row in source_rows}
    access_ids = {row["source_id"] for row in rows}
    if source_ids != access_ids:
        failures.append("payload access source IDs do not match source pack")

    for row in rows:
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['source_id']} accepts evidence before payload validation")
        if row["payload_url_or_status"].startswith("http"):
            if row["payload_status"] != "payload-not-cached":
                failures.append(f"{row['source_id']} HTTP payload status must remain not cached")
            if row["live_fetch_status"] != "no-live-fetcher-reviewed":
                failures.append(f"{row['source_id']} live fetch status is not held")
            if not row["cache_target"].startswith("data/cache/canada/"):
                failures.append(f"{row['source_id']} cache target is not Canada-scoped")
        else:
            if row["cache_target"] != "none":
                failures.append(f"{row['source_id']} non-URL source must not name cache target")
            if row["live_fetch_status"] != "not-fetchable":
                failures.append(f"{row['source_id']} non-URL source must be not-fetchable")
        if not row["blocked_claims"]:
            failures.append(f"{row['source_id']} missing blocked claims")
        if "validated" in row["evidence_acceptance_status"]:
            failures.append(f"{row['source_id']} uses validation language in acceptance status")

    _ = source_fields
    if failures:
        print("Canada source-payload access gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada source-payload access gate: PASS")
    print("  checked source-pack coverage, held fetch status, cache targets, and claim blockers")
    return 0


if __name__ == "__main__":
    sys.exit(main())
