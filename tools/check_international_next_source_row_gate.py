#!/usr/bin/env python3
"""Gate international next source-row selection ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-next-source-row-gate-001.csv"

FIELDS = [
    "candidate_id",
    "region_or_lane",
    "current_depth",
    "next_unblocked_gate",
    "why_this_gate",
    "proof_value",
    "risk_or_blocker",
    "recommendation",
    "blocked_claims",
    "next_action",
]
REQUIRED_REGIONS = {"Canada", "EU Rhine-Alpine", "India", "Japan", "China"}
REQUIRED_BLOCKS = {
    "equal_depth_claim",
    "official_network",
    "country_or_regional_approval",
    "policy_alignment",
    "source_row_validation_until_gate_closes",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "guaranteed_sla",
    "numeric_roi",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
}


def main() -> int:
    with LEDGER.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("international next source-row gate columns do not match contract")
    if len(rows) != 5:
        failures.append("international next source-row gate must compare five regions")
    regions = {row["region_or_lane"] for row in rows}
    if regions != REQUIRED_REGIONS:
        failures.append(f"international next source-row gate regions mismatch: {sorted(regions)}")
    primary = [row for row in rows if row["recommendation"] == "primary"]
    if len(primary) != 1:
        failures.append("international next source-row gate must have exactly one primary recommendation")
    elif primary[0]["region_or_lane"] != "China" or primary[0]["next_unblocked_gate"] != "china_source_content_sample":
        failures.append("international next source-row gate must select China source-content sample as primary")
    if not any(row["region_or_lane"] == "EU Rhine-Alpine" and row["recommendation"] == "alternate" for row in rows):
        failures.append("international next source-row gate must keep EU road-link endpoint as alternate")
    if not any(row["region_or_lane"] == "Canada" and row["recommendation"] == "alternate" for row in rows):
        failures.append("international next source-row gate must keep Canada external packet as alternate")
    for row in rows:
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['candidate_id']} missing blocked claims: {sorted(missing)}")
        if row["recommendation"] not in {"primary", "alternate", "defer"}:
            failures.append(f"{row['candidate_id']} has unsupported recommendation")
        if "before" not in row["next_action"] and row["recommendation"] == "primary":
            failures.append(f"{row['candidate_id']} primary next action must preserve before dependency")
        if "equally proven" in row["why_this_gate"].lower() or "all regions validated" in row["why_this_gate"].lower():
            failures.append(f"{row['candidate_id']} risks equal-depth or validation overclaim")
    if failures:
        print("International next source-row gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("International next source-row gate: PASS")
    print("  checked regional comparison, primary selection, alternates, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
