#!/usr/bin/env python3
"""Build EU Rhine-Alpine source-payload access manifest."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE_PACK = ROOT / "data" / "international-eu-rhine-alpine-adapter-source-pack-001.csv"
PREFLIGHT = ROOT / "data" / "international-eu-rhine-alpine-parser-preflight-001.csv"
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-source-payload-access-001.csv"

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


def task_by_source() -> dict[str, dict[str, str]]:
    return {
        row["source_id"]: row
        for row in read_csv(PREFLIGHT)
        if row["source_id"] not in {"carry-forward", "internal-roles"}
    }


def access_mode(value: str) -> tuple[str, str, str]:
    if value.startswith("http"):
        return "manual-or-fletch-cache-candidate", "payload-not-cached", "no-live-fetcher-reviewed"
    if value in {"source-needed", "none"}:
        return "source-selection-required", "source-needed", "not-fetchable"
    return "held-no-payload", "held", "not-fetchable"


def main() -> None:
    tasks = task_by_source()
    rows: list[dict[str, str]] = []
    for source in read_csv(SOURCE_PACK):
        source_id = source["source_id"]
        task = tasks.get(source_id)
        mode, status, live = access_mode(source["source_path_or_status"])
        rows.append(
            {
                "payload_access_id": f"EUR-PAYLOAD-{source_id.replace('EUR-SRC-', '').replace('-', '')}",
                "source_id": source_id,
                "source_family": source["source_family"],
                "payload_url_or_status": source["source_path_or_status"],
                "owner_or_publisher": source["owner_or_publisher"],
                "cache_target": f"data/cache/eu-rhine-alpine/{source_id.lower()}-payload.pending"
                if source["source_path_or_status"].startswith("http")
                else "none",
                "access_mode": mode,
                "payload_status": status,
                "live_fetch_status": live,
                "parser_task_id": task["task_id"] if task else "none",
                "required_fields": source["required_fields"],
                "post_access_gate": "python tools/check_eu_rhine_alpine_source_payload_access.py",
                "evidence_acceptance_status": "not-accepted",
                "blocked_claims": source["claim_boundary"].removeprefix("no "),
                "next_action": source["next_action"],
            }
        )
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
