#!/usr/bin/env python3
"""Gate state payload promotion closeout outputs."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CLOSEOUT = ROOT / "data" / "state-payload-promotion-closeout-001.csv"
ACTION = ROOT / "data" / "state-payload-promotion-next-actions-001.csv"
REVIEW = ROOT / "docs" / "reviews" / "state-payload-promotion-closeout-001.md"

REQUIRED_STEPS = {
    "fit_kernel",
    "source_adapter_contract",
    "payload_scaffold",
    "payload_preflight",
    "candidate_tierization",
    "role_review_evaluation",
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
    closeout = read_csv(CLOSEOUT)
    actions = read_csv(ACTION)
    review = REVIEW.read_text(encoding="utf-8")
    steps = {row["chain_step"] for row in closeout}
    if steps != REQUIRED_STEPS:
        failures.append(f"closeout steps mismatch: {sorted(steps)}")
    if len(actions) != 4:
        failures.append("expected four next-action rows")
    if not any(row["chain_step"] == "role_review_evaluation" and row["status"] == "held" for row in closeout):
        failures.append("role review evaluation must remain held")
    if not any("client intake" in row["allowed_use"] or "client" in row["allowed_use"] for row in closeout):
        failures.append("closeout must preserve client-intake allowed use")
    for row in closeout + actions:
        missing = REQUIRED_BLOCKS - set(row["held_claims"].split("|"))
        if missing:
            failures.append(f"row missing held claims: {sorted(missing)}")
    if "state_payload_pathway_ready_for_client_intake_promotion_held" not in review:
        failures.append("review missing promotion closeout gate decision")
    if "not ready for public claim promotion" not in review:
        failures.append("review must state public claim promotion is not ready")
    if failures:
        print("State payload promotion closeout gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("State payload promotion closeout gate: PASS")
    print("  checked chain status, next actions, held promotion, and held claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
