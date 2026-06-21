#!/usr/bin/env python3
"""Build China source-payload access manifest."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE_PACK = ROOT / "data" / "international-china-adapter-source-pack-001.csv"
OUTPUT = ROOT / "data" / "international-china-source-payload-access-001.csv"

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


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def access_mode(value: str) -> tuple[str, str, str]:
    if value.startswith("http"):
        return "manual-or-fletch-cache-candidate", "payload-not-cached", "no-live-fetcher-reviewed"
    if value == "none":
        return "held-no-payload", "held", "not-fetchable"
    return "internal-fixture-reference", "local-reference-not-payload", "not-fetchable"


def main() -> None:
    rows: list[dict[str, str]] = []
    for source in read_csv(SOURCE_PACK):
        source_id = source["source_id"]
        mode, status, live = access_mode(source["source_path_or_status"])
        rows.append(
            {
                "payload_access_id": f"CHN-PAYLOAD-{source_id.replace('CHN-SRC-', '').replace('-', '')}",
                "source_id": source_id,
                "source_family": source["source_family"],
                "payload_url_or_status": source["source_path_or_status"],
                "owner_or_publisher": source["owner_or_publisher"],
                "cache_target": f"data/cache/china/{source_id.lower()}-payload.pending"
                if source["source_path_or_status"].startswith("http")
                else "none",
                "access_mode": mode,
                "payload_status": status,
                "live_fetch_status": live,
                "required_fields": source["required_fields"],
                "post_access_gate": "python tools/check_china_source_payload_access.py",
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
