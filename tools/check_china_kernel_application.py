#!/usr/bin/env python3
"""Gate China proof-kernel application ledger."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "data" / "international-china-kernel-application-001.csv"

FIELDS = [
    "kernel_step",
    "china_status",
    "china_artifact",
    "canada_eu_india_japan_comparison",
    "promotion_decision",
    "blocked_claims",
    "next_action",
]
REQUIRED_STEPS = {
    "source_custody",
    "parser_contract",
    "fixture_replacement",
    "target_posture",
    "review_packet",
}
REQUIRED_BLOCKS = {
    "official_corridor_designation",
    "policy_alignment",
    "route_designation",
    "source_row_validation",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
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
        failures.append("China kernel application columns do not match contract")
    if len(rows) != 5:
        failures.append("China kernel application must have five rows")
    steps = {row["kernel_step"] for row in rows}
    if steps != REQUIRED_STEPS:
        failures.append(f"China kernel steps mismatch: {sorted(steps)}")
    for row in rows:
        step = row.get("kernel_step", "<missing>")
        if row["promotion_decision"] not in {"preflight_ready_not_promoted", "held"}:
            failures.append(f"{step} has unsupported promotion decision: {row['promotion_decision']}")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{step} missing blocked claims: {sorted(missing)}")
        if step != "source_custody" and row["china_status"] == "source_pack_preflight_declared":
            failures.append(f"{step} cannot claim source-pack preflight")
        if "before" not in row["next_action"] and step != "review_packet":
            failures.append(f"{step} next action must name a before-promotion dependency")
    if failures:
        print("China kernel application gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("China kernel application gate: PASS")
    print("  checked kernel steps, promotion holds, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
