#!/usr/bin/env python3
"""Gate Canada source-payload probe output."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ACCESS = ROOT / "data" / "international-canada-source-payload-access-001.csv"
RESOLUTION = ROOT / "data" / "international-canada-source-payload-resolution-001.csv"
PROBE = ROOT / "data" / "international-canada-source-payload-probe-001.csv"

FIELDS = [
    "probe_id",
    "source_id",
    "payload_url_or_status",
    "probe_url",
    "probe_method",
    "http_status",
    "final_url",
    "content_type",
    "bytes_sampled",
    "probe_result",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    if not path.exists():
        return [], []
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    _, access_rows = read_csv(ACCESS)
    _, resolution_rows = read_csv(RESOLUTION)
    fields, probe_rows = read_csv(PROBE)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("probe columns do not match required contract")
    if len(probe_rows) != len(access_rows):
        failures.append("probe row count does not match source-payload access rows")

    access_by_source = {row["source_id"]: row for row in access_rows}
    primary_resolution = {
        row["source_id"]: row["resolved_url"]
        for row in resolution_rows
        if row["probe_priority"] == "primary"
    }
    for row in probe_rows:
        source_id = row["source_id"]
        access = access_by_source.get(source_id)
        if access is None:
            failures.append(f"{source_id} missing from payload access manifest")
            continue
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{source_id} accepts evidence from probe")
        expected_probe_url = primary_resolution.get(source_id, access["payload_url_or_status"])
        if row["probe_url"] != expected_probe_url:
            failures.append(f"{source_id} probe_url does not match resolved access target")
        if not row["blocked_claims"]:
            failures.append(f"{source_id} missing blocked claims")
        if row["probe_url"].startswith("http"):
            if row["probe_method"] != "http-get-sample":
                failures.append(f"{source_id} URL source was not probed with HTTP sample")
            if row["http_status"] == "not-applicable":
                failures.append(f"{source_id} URL source has no HTTP status")
            if int(row["bytes_sampled"]) < 0:
                failures.append(f"{source_id} has invalid bytes_sampled")
        else:
            if row["probe_method"] != "not-fetchable":
                failures.append(f"{source_id} non-URL source should remain not-fetchable")
            if row["http_status"] != "not-applicable":
                failures.append(f"{source_id} non-URL source should not have HTTP status")
        if row["probe_result"] in {"accepted", "validated", "approved"}:
            failures.append(f"{source_id} has prohibited acceptance wording")

    if failures:
        print("Canada source-payload probe gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada source-payload probe gate: PASS")
    print("  checked probe coverage, HTTP metadata posture, and not-accepted evidence status")
    return 0


if __name__ == "__main__":
    sys.exit(main())
