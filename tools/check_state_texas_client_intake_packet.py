#!/usr/bin/env python3
"""Gate Texas-specific client intake packet artifacts."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
PACKET = DATA / "state-texas-client-intake-packet-001.csv"
AGENDA = DATA / "state-texas-client-intake-workshop-agenda-001.csv"
CLAIMS = DATA / "state-texas-client-intake-held-claims-001.csv"
ASKS = DATA / "state-texas-client-intake-source-asks-001.csv"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-client-intake-packet-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-client-intake-packet-001.md"

REQUIRED_BLOCKS = {
    "official_designation",
    "legal_sla",
    "construction",
    "numeric_roi",
    "roi",
    "eligibility",
    "compliance",
    "endorsement",
    "validation",
    "public_readiness",
    "state_approval",
    "source_backed_full_inventory",
}


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> int:
    failures: list[str] = []
    packet = read_csv(PACKET)
    agenda = read_csv(AGENDA)
    claims = read_csv(CLAIMS)
    asks = read_csv(ASKS)
    brief = BRIEF.read_text(encoding="utf-8")
    review = REVIEW.read_text(encoding="utf-8")

    if len(packet) != 5:
        failures.append("Texas intake packet must contain five packet rows")
    if len(agenda) != 4:
        failures.append("Texas workshop agenda must contain four rows")
    if len(claims) != 4:
        failures.append("Texas held-claim guide must contain four rows")
    if len(asks) != 4:
        failures.append("Texas source asks must contain four rows")
    for row in packet + agenda + claims + asks:
        missing = REQUIRED_BLOCKS - set(row["held_claims"].split("|"))
        if missing:
            failures.append(f"row missing held claims: {sorted(missing)}")
    if "not a TxDOT plan" not in brief:
        failures.append("brief must hold TxDOT official-plan boundary")
    if "texas_client_intake_packet_ready_promotion_held" not in review:
        failures.append("review missing Texas intake gate decision")
    if not any(row["source_surface"] == "failure_evidence" for row in asks):
        failures.append("Texas asks must include failure evidence")
    if failures:
        print("Texas client intake packet gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Texas client intake packet gate: PASS")
    print("  checked packet, agenda, source asks, held claims, and boundary language")
    return 0


if __name__ == "__main__":
    sys.exit(main())
