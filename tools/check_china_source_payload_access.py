#!/usr/bin/env python3
"""Gate China source-payload access manifest."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE_PACK = ROOT / "data" / "international-china-adapter-source-pack-001.csv"
ACCESS = ROOT / "data" / "international-china-source-payload-access-001.csv"

FIELDS = [
    "payload_access_id",
    "source_id",
    "source_family",
    "payload_url_or_status",
    "owner_or_publisher",
    "cache_target",
    "access_mode",
    "payload_status",
    "live_fetch_status",
    "required_fields",
    "post_access_gate",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]
REQUIRED_BLOCKS = {
    "official",
    "guaranteed SLA",
    "ROI",
    "validation",
    "public-readiness",
    "external-readiness",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    _, source_rows = read_csv(SOURCE_PACK)
    fields, access_rows = read_csv(ACCESS)
    source_ids = {row["source_id"] for row in source_rows}
    access_ids = {row["source_id"] for row in access_rows}
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("China payload access columns do not match contract")
    if len(access_rows) != len(source_rows):
        failures.append("China payload access row count does not match source pack")
    if access_ids != source_ids:
        failures.append("China payload access source IDs do not match source pack")
    url_rows = [row for row in access_rows if row["payload_url_or_status"].startswith("http")]
    if len(url_rows) < 5:
        failures.append("China payload access must retain five official URL probe candidates")
    for row in access_rows:
        source_id = row["source_id"]
        blocked_claims = row["blocked_claims"]
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{source_id} accepts evidence before probe")
        for required in REQUIRED_BLOCKS:
            if required not in blocked_claims:
                failures.append(f"{source_id} missing blocked claim token: {required}")
        if row["source_family"] == "service_targets":
            if "approval" not in blocked_claims:
                failures.append(f"{source_id} missing service-target approval hold")
        elif "policy alignment" not in blocked_claims:
            failures.append(f"{source_id} missing source-row policy-alignment hold")
        if row["payload_url_or_status"].startswith("http"):
            if row["access_mode"] != "manual-or-fletch-cache-candidate":
                failures.append(f"{source_id} URL source must remain cache candidate")
            if row["cache_target"] == "none":
                failures.append(f"{source_id} URL source missing cache target")
        else:
            if row["cache_target"] != "none":
                failures.append(f"{source_id} non-URL source should not have cache target")
            if row["live_fetch_status"] != "not-fetchable":
                failures.append(f"{source_id} non-URL source should be not-fetchable")
    if failures:
        print("China source-payload access gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("China source-payload access gate: PASS")
    print("  checked source coverage, cache candidates, held rows, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
