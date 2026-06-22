#!/usr/bin/env python3
"""Gate Texas diagnostic readout outputs."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
READOUT = DATA / "state-texas-diagnostic-readout-001.csv"
SCORECARD = DATA / "state-texas-diagnostic-scorecard-001.csv"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-diagnostic-readout-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-diagnostic-readout-001.md"

REQUIRED_TOPICS = {
    "pipeline_result",
    "role_spread",
    "resilience_overlay",
    "source_asks",
    "promotion_boundary",
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
    readout = read_csv(READOUT)
    scorecard = read_csv(SCORECARD)
    brief = BRIEF.read_text(encoding="utf-8")
    review = REVIEW.read_text(encoding="utf-8")
    topics = {row["topic"] for row in readout}
    if topics != REQUIRED_TOPICS:
        failures.append(f"readout topics mismatch: {sorted(topics)}")
    if len(scorecard) != 6:
        failures.append("Texas scorecard must have six candidate rows")
    if any(row["source_status"] != "held" or row["promotion_status"] != "held" for row in scorecard):
        failures.append("all Texas scorecard rows must hold source and promotion status")
    for row in readout + scorecard:
        missing = REQUIRED_BLOCKS - set(row["held_claims"].split("|"))
        if missing:
            failures.append(f"row missing held claims: {sorted(missing)}")
    if "not a TxDOT plan" not in brief:
        failures.append("brief must hold TxDOT plan boundary")
    if "texas_diagnostic_readout_ready_source_backed_pilot_next" not in review:
        failures.append("review missing Texas diagnostic gate decision")
    if failures:
        print("Texas diagnostic readout gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Texas diagnostic readout gate: PASS")
    print("  checked readout topics, scorecard holds, held claims, and boundary language")
    return 0


if __name__ == "__main__":
    sys.exit(main())
