#!/usr/bin/env python3
"""Gate Canada media proof card claim boundaries."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CARD = ROOT / "data" / "international-canada-media-proof-card-001.csv"

FIELDS = [
    "card_id",
    "media_question",
    "safe_answer",
    "cite",
    "status",
    "blocked_claims",
]
REQUIRED_BLOCKS = {
    "official_network",
    "route_designation",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "agency_approval",
    "port_endorsement",
    "terminal_performance",
    "node_completeness",
    "road_access_proof",
    "throughput_proof",
    "construction_ready",
    "guaranteed_sla",
    "roi",
    "validation",
    "public_readiness",
    "external_readiness",
}
PROHIBITED_SAFE_ANSWER_TEXT = {
    "official canadian network",
    "approved by",
    "guaranteed sla",
    "validated by",
    "proves roi",
    "construction ready",
    "public ready",
}


def main() -> int:
    with CARD.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("Canada media proof card columns do not match contract")
    if len(rows) != 3:
        failures.append("Canada media proof card must have three media rows")
    for row in rows:
        answer = row["safe_answer"].lower()
        for phrase in PROHIBITED_SAFE_ANSWER_TEXT:
            if phrase in answer:
                failures.append(f"{row['card_id']} safe answer contains prohibited phrase: {phrase}")
        if "internal" not in answer and row["card_id"] == "CAN-MEDIA-PROOF-001":
            failures.append("main Canada proof answer must say internal")
        if not row["cite"]:
            failures.append(f"{row['card_id']} missing cite")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"{row['card_id']} missing blocked claims: {sorted(missing_blocks)}")

    if failures:
        print("Canada media proof card gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada media proof card gate: PASS")
    print("  checked safe answers, citations, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
