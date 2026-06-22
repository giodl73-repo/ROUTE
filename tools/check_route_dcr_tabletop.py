#!/usr/bin/env python3
"""Gate ROUTE DCR tabletop scope artifacts."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "route-dcr-tabletop-scope-001.md"
SCENARIOS = DATA / "route-dcr-tabletop-scenarios-001.csv"
INPUTS = DATA / "route-dcr-tabletop-inputs-001.csv"
OUTPUTS = DATA / "route-dcr-tabletop-outputs-001.csv"
NONCLAIMS = DATA / "route-dcr-tabletop-nonclaims-001.csv"

REQUIRED_DECISIONS = {
    "reroute and EV support posture",
    "terminal access and signage posture",
    "recovery and communications posture",
    "asset and investment posture",
    "reliability and investment posture",
}
REQUIRED_INPUTS = {
    "service hierarchy",
    "incident or closure event",
    "evidence boundary",
}
REQUIRED_NONCLAIMS = {
    "traffic_control",
    "legal_detour",
    "guaranteed_sla",
    "ev_availability",
    "incident_command",
    "construction",
    "roi",
    "endorsement",
    "public_readiness",
}


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def split_claims(value: str) -> set[str]:
    return {part.strip() for part in value.split("|") if part.strip()}


def main() -> int:
    failures: list[str] = []
    scenarios = read_csv(SCENARIOS)
    inputs = read_csv(INPUTS)
    outputs = read_csv(OUTPUTS)
    nonclaims = read_csv(NONCLAIMS)
    brief = BRIEF.read_text(encoding="utf-8")

    decisions = {row["simulated_decision"] for row in scenarios}
    if decisions != REQUIRED_DECISIONS:
        failures.append(f"scenario decision mismatch: {sorted(decisions)}")
    if len(outputs) < 6:
        failures.append("expected at least six DCR outputs")
    required_inputs = {
        row["input_class"] for row in inputs if row["required_for_tabletop"] == "yes"
    }
    if required_inputs != REQUIRED_INPUTS:
        failures.append(f"required input mismatch: {sorted(required_inputs)}")
    nonclaim_keys = {row["blocked_claim"] for row in nonclaims}
    if nonclaim_keys != REQUIRED_NONCLAIMS:
        failures.append(f"nonclaim mismatch: {sorted(nonclaim_keys)}")
    for row in scenarios + outputs:
        claims = split_claims(row["held_claims"])
        if not claims:
            failures.append(f"missing held claims on {row}")
        if "traffic_control" not in claims and "operator_boundary" in row:
            failures.append(f"scenario must hold traffic_control: {row['scenario_id']}")
    for row in scenarios:
        if not row["operator_boundary"]:
            failures.append(f"scenario missing operator boundary: {row['scenario_id']}")
        if row["evidence_status"] not in {"heuristic", "source_needed"}:
            failures.append(f"bad evidence status: {row['scenario_id']}")
    required_phrases = [
        "one-day Decision Control Room tabletop",
        "ROUTE is advisory",
        "not a live operations integration",
        "traffic-control",
        "EV availability guarantee",
    ]
    for phrase in required_phrases:
        if phrase not in brief:
            failures.append(f"brief missing phrase: {phrase}")

    if failures:
        print("ROUTE DCR tabletop gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("ROUTE DCR tabletop gate: PASS")
    print("  checked scenarios, required inputs, outputs, nonclaims, and boundaries")
    return 0


if __name__ == "__main__":
    sys.exit(main())
