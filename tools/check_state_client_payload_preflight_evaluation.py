#!/usr/bin/env python3
"""Gate generic state client payload preflight evaluation."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
EVALUATION = ROOT / "data" / "state-client-payload-preflight-evaluation-001.csv"
REVIEW = ROOT / "docs" / "reviews" / "state-client-payload-preflight-evaluation-001.md"

REQUIRED_AREAS = {
    "manifest_completeness",
    "segment_shape",
    "priority_node_references",
    "terminal_access_references",
    "restriction_failure_references",
    "non_promotion_references",
    "source_custody",
    "promotion_readiness",
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


def main() -> int:
    failures: list[str] = []
    with EVALUATION.open(newline="", encoding="utf-8") as f:
        rows = list(csv.DictReader(f))
    review = REVIEW.read_text(encoding="utf-8")
    areas = {row["check_area"] for row in rows}
    if areas != REQUIRED_AREAS:
        failures.append(f"evaluation areas mismatch: {sorted(areas)}")
    if len(rows) != 8:
        failures.append("evaluation must contain eight rows")
    pass_rows = [row for row in rows if row["evaluation_status"] == "pass"]
    hold_rows = [row for row in rows if row["evaluation_status"] == "hold"]
    if len(pass_rows) < 5:
        failures.append("template integrity should pass at least five checks")
    if len(hold_rows) < 2:
        failures.append("source custody and promotion readiness must remain held")
    for row in rows:
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['check_id']} missing blocked claims: {sorted(missing)}")
    if not any(row["check_area"] == "source_custody" and row["evaluation_status"] == "hold" for row in rows):
        failures.append("source custody must be held")
    if not any(row["check_area"] == "promotion_readiness" and row["evaluation_status"] == "hold" for row in rows):
        failures.append("promotion readiness must be held")
    if "state_client_payload_preflight_ready_for_filled_payload" not in review:
        failures.append("review missing preflight gate decision")
    if "Real client data reviewed | no" not in review:
        failures.append("review must state real client data was not reviewed")
    if failures:
        print("State client payload preflight evaluation gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("State client payload preflight evaluation gate: PASS")
    print("  checked template integrity, held source custody, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
