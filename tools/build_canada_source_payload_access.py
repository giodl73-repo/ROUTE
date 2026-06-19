#!/usr/bin/env python3
"""Build Canada source-payload access manifest from the source pack.

This manifest is a pre-parser access plan. It does not fetch URLs, inspect
payload fields, or validate any Canada adapter output.
"""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE_PACK = ROOT / "data" / "international-canada-adapter-source-pack-001.csv"
PREFLIGHT = ROOT / "data" / "international-canada-parser-preflight-001.csv"
OUTPUT = ROOT / "data" / "international-canada-source-payload-access-001.csv"

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
    "parser_task_id",
    "required_fields",
    "post_access_gate",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def write_csv(path: Path, rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def cache_target(source_id: str) -> str:
    return f"data/cache/canada/{source_id.lower()}-payload.pending"


def access_mode(source_path_or_status: str) -> tuple[str, str, str]:
    if source_path_or_status.startswith("http"):
        return (
            "manual-or-fletch-cache-candidate",
            "payload-not-cached",
            "no-live-fetcher-reviewed",
        )
    if source_path_or_status == "source-needed":
        return (
            "source-selection-required",
            "source-needed",
            "not-fetchable",
        )
    return (
        "held-no-payload",
        "held",
        "not-fetchable",
    )


def task_by_source(preflight_rows: list[dict[str, str]]) -> dict[str, dict[str, str]]:
    return {
        row["source_id"]: row
        for row in preflight_rows
        if row["source_id"] not in {"carry-forward", "internal-roles"}
    }


def main() -> None:
    source_rows = read_csv(SOURCE_PACK)
    task_rows = task_by_source(read_csv(PREFLIGHT))
    rows: list[dict[str, str]] = []

    for source in source_rows:
        source_id = source["source_id"]
        task = task_rows.get(source_id)
        mode, payload_status, live_fetch_status = access_mode(source["source_path_or_status"])
        rows.append(
            {
                "payload_access_id": f"CAN-PAYLOAD-{source_id.replace('CAN-SRC-', '').replace('-', '')}",
                "source_id": source_id,
                "source_family": source["source_family"],
                "payload_url_or_status": source["source_path_or_status"],
                "owner_or_publisher": source["owner_or_publisher"],
                "cache_target": cache_target(source_id) if source["source_path_or_status"].startswith("http") else "none",
                "access_mode": mode,
                "payload_status": payload_status,
                "live_fetch_status": live_fetch_status,
                "parser_task_id": task["task_id"] if task else "none",
                "required_fields": source["required_fields"],
                "post_access_gate": "python tools/check_canada_source_payload_access.py",
                "evidence_acceptance_status": "not-accepted",
                "blocked_claims": source["claim_boundary"].removeprefix("no "),
                "next_action": source["next_action"],
            }
        )

    write_csv(OUTPUT, rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
