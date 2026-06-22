#!/usr/bin/env python3
"""Gate state client intake packet artifacts."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
PACKET = DATA / "state-client-intake-packet-001.csv"
AGENDA = DATA / "state-client-intake-workshop-agenda-001.csv"
CLAIMS = DATA / "state-client-intake-held-claims-001.csv"
BRIEF = ROOT / "docs" / "briefs" / "state-client-intake-packet-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-client-intake-packet-001.md"

REQUIRED_PACKET = {
    "INTAKE-001",
    "INTAKE-002",
    "INTAKE-003",
    "INTAKE-004",
    "INTAKE-005",
    "INTAKE-006",
}
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
    brief = BRIEF.read_text(encoding="utf-8")
    review = REVIEW.read_text(encoding="utf-8")
    ids = {row["packet_item_id"] for row in packet}
    if ids != REQUIRED_PACKET:
        failures.append(f"packet ids mismatch: {sorted(ids)}")
    if len(agenda) != 4:
        failures.append("workshop agenda must contain four rows")
    if len(claims) != 3:
        failures.append("held claim guide must contain three rows")
    for row in packet + agenda + claims:
        missing = REQUIRED_BLOCKS - set(row["held_claims"].split("|"))
        if missing:
            failures.append(f"row missing held claims: {sorted(missing)}")
    if not any(row["ready_status"] == "ready_for_client_fill" for row in packet):
        failures.append("packet must include client-fill templates")
    if "not a public" not in brief:
        failures.append("brief must hold public readiness")
    if "state_client_intake_packet_ready_promotion_held" not in review:
        failures.append("review missing intake packet gate decision")
    if failures:
        print("State client intake packet gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("State client intake packet gate: PASS")
    print("  checked packet index, agenda, held claims, and boundary language")
    return 0


if __name__ == "__main__":
    sys.exit(main())
