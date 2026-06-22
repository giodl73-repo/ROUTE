#!/usr/bin/env python3
"""Gate Texas buyer review packet outputs."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
DECISION = DATA / "state-texas-buyer-review-decision-001.csv"
AGENDA = DATA / "state-texas-buyer-review-agenda-001.csv"
SOURCE_REQUEST = DATA / "state-texas-buyer-source-request-001.csv"
OBJECTIONS = DATA / "state-texas-buyer-objection-response-001.csv"
BRIEF = ROOT / "docs" / "briefs" / "state-texas-buyer-review-packet-001.md"
REVIEW = ROOT / "docs" / "reviews" / "state-texas-buyer-review-packet-001.md"

REQUIRED_DECISIONS = {
    "why_now",
    "what_buyer_gets",
    "what_buyer_must_supply",
    "what_is_not_included",
    "go_no_go",
}
REQUIRED_AGENDA = {
    "opening_boundary",
    "service_priorities",
    "source_owner_review",
    "deliverable_review",
    "decision_close",
}
REQUIRED_SURFACES = {
    "segment_inventory",
    "priority_nodes",
    "failure_evidence",
    "terminal_access",
    "non_promotion",
    "claim_boundary",
}
REQUIRED_BLOCKS = {
    "official_designation",
    "legal_sla",
    "construction",
    "cost",
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
    decisions = read_csv(DECISION)
    agenda = read_csv(AGENDA)
    requests = read_csv(SOURCE_REQUEST)
    objections = read_csv(OBJECTIONS)
    brief = BRIEF.read_text(encoding="utf-8")
    review = REVIEW.read_text(encoding="utf-8")

    decision_topics = {row["decision_topic"] for row in decisions}
    if decision_topics != REQUIRED_DECISIONS:
        failures.append(f"decision topics mismatch: {sorted(decision_topics)}")
    agenda_items = {row["agenda_item"] for row in agenda}
    if agenda_items != REQUIRED_AGENDA:
        failures.append(f"agenda items mismatch: {sorted(agenda_items)}")
    surfaces = {row["source_surface"] for row in requests}
    if surfaces != REQUIRED_SURFACES:
        failures.append(f"source surfaces mismatch: {sorted(surfaces)}")
    if len(objections) != 6:
        failures.append("expected six objection response rows")
    if not all(row["recommended_posture"] == "buyer_review_only" for row in decisions):
        failures.append("decision rows must stay buyer-review only")
    if not all(row["blocks_if_missing"] == "paid_pilot_start" for row in requests):
        failures.append("source request rows must block paid pilot start when missing")
    if not all(row["decision_rule"] == "answer_without_promoting_held_claims" for row in objections):
        failures.append("objection rows must answer without promoting held claims")
    for row in decisions + agenda + requests + objections:
        missing = REQUIRED_BLOCKS - set(row["held_claims"].split("|"))
        if missing:
            failures.append(f"row missing held claims: {sorted(missing)}")
    if "not adoption" not in brief:
        failures.append("brief must say the desired close is not adoption")
    if "not a price quote, procurement response, TxDOT plan" not in brief:
        failures.append("brief must hold commercial and TxDOT boundaries")
    if "texas_buyer_review_packet_ready_for_sponsor_conversation" not in review:
        failures.append("review missing buyer review gate decision")

    if failures:
        print("Texas buyer review packet gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Texas buyer review packet gate: PASS")
    print("  checked decision memo, agenda, source request, objections, and held claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
