#!/usr/bin/env python3
"""Gate international adaptive proof closeout ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-adaptive-proof-closeout-001.csv"

FIELDS = [
    "closeout_id",
    "proof_lane",
    "depth_level",
    "input_artifacts",
    "allowed_claim",
    "blocked_claims",
    "next_action",
]
REQUIRED_LANES = {
    "Canada",
    "EU Rhine-Alpine",
    "India",
    "Japan",
    "China",
    "multi-region maps",
    "international_system",
}
REQUIRED_BLOCKS = {
    "single_depth_equivalence",
    "official_network",
    "country_or_regional_approval",
    "policy_alignment",
    "source_row_validation_where_not_closed",
    "fixture_replacement_where_not_closed",
    "parsed_adapter_where_not_closed",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "terminal_performance",
    "guaranteed_sla",
    "numeric_roi",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
}
PROHIBITED = {
    "official network proof",
    "approved network",
    "validated network",
    "guaranteed sla",
    "roi proof",
    "public ready",
    "external validation complete",
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("international adaptive closeout columns do not match contract")
    if len(rows) != 7:
        failures.append("international adaptive closeout must have seven proof lanes")
    lanes = {row["proof_lane"] for row in rows}
    if lanes != REQUIRED_LANES:
        failures.append(f"international adaptive closeout lanes mismatch: {sorted(lanes)}")
    if not any(row["proof_lane"] == "Canada" and "depth" in row["depth_level"] for row in rows):
        failures.append("international adaptive closeout must preserve Canada as depth proof")
    if not any(row["proof_lane"] == "China" and "content_depth_started" in row["depth_level"] for row in rows):
        failures.append("international adaptive closeout must preserve China as content-depth-started proof with source-row validation held")
    if not any(row["proof_lane"] == "multi-region maps" and "breadth" in row["depth_level"] for row in rows):
        failures.append("international adaptive closeout must preserve map breadth as a fixture only")
    for row in rows:
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['closeout_id']} missing blocked claims: {sorted(missing)}")
        if not row["input_artifacts"]:
            failures.append(f"{row['closeout_id']} missing input artifacts")
        text = " ".join([row["allowed_claim"], row["next_action"]]).lower()
        for phrase in PROHIBITED:
            if phrase in text:
                failures.append(f"{row['closeout_id']} promotes prohibited phrase: {phrase}")
    decision = [row for row in rows if row["proof_lane"] == "international_system"]
    if len(decision) != 1:
        failures.append("international adaptive closeout requires one system decision row")
    elif "equally proven" not in decision[0]["next_action"]:
        failures.append("international adaptive closeout must warn against equal-depth overclaim")
    if failures:
        print("International adaptive proof closeout gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("International adaptive proof closeout gate: PASS")
    print("  checked proof lanes, depth distinctions, map breadth, blocked claims, and overclaim guards")
    return 0


if __name__ == "__main__":
    sys.exit(main())
