#!/usr/bin/env python3
"""Gate ROUTE DCR renewal artifacts."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "route-dcr-renewal-gate-001.md"
ASSETS = DATA / "route-dcr-renewal-assets-001.csv"
PATHS = DATA / "route-dcr-renewal-paths-001.csv"
GATES = DATA / "route-dcr-renewal-gates-001.csv"

REQUIRED_ASSETS = {
    "service_hierarchy",
    "promise_ledger",
    "failure_mode_ledger",
    "monitoring_history",
    "simulation_library",
    "switch_playbooks",
    "evidence_boundary",
    "executive_cadence",
    "source_custody",
    "renewal_backlog",
}
REQUIRED_PATHS = {
    "quarterly_review",
    "event_review",
    "workbench_buildout",
    "source_integration",
    "signage_routing_package",
    "ev_support_package",
    "stop_or_hold",
}
REQUIRED_GATES = {
    "asset_changed",
    "decision_repeated",
    "event_occurred",
    "source_gate_opened",
    "claim_boundary_mattered",
    "operator_review_happened",
    "no_path_stop",
    "authority_boundary_preserved",
}
BLOCKED_CLAIMS = {
    "official_plan",
    "endorsement",
    "guaranteed_sla",
    "public_readiness",
    "incident_command",
    "traffic_control",
    "live_integration",
    "automated_control",
    "legal_detour",
    "ev_availability",
    "roi",
    "procurement_readiness",
    "construction",
}


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def split_claims(value: str) -> set[str]:
    return {part.strip() for part in value.split("|") if part.strip()}


def require_set(
    failures: list[str], label: str, observed: set[str], expected: set[str]
) -> None:
    if observed != expected:
        failures.append(
            f"{label} mismatch: expected {sorted(expected)}, observed {sorted(observed)}"
        )


def check_claims(failures: list[str], label: str, row: dict[str, str]) -> None:
    claims = split_claims(row["blocked_claims"])
    if not claims:
        failures.append(f"{label} row missing blocked claims: {row}")
    unknown = claims - BLOCKED_CLAIMS
    if unknown:
        failures.append(f"{label} row has unknown claims {sorted(unknown)}: {row}")


def main() -> int:
    failures: list[str] = []
    assets = read_csv(ASSETS)
    paths = read_csv(PATHS)
    gates = read_csv(GATES)
    brief = BRIEF.read_text(encoding="utf-8")

    require_set(failures, "asset", {row["asset_id"] for row in assets}, REQUIRED_ASSETS)
    require_set(failures, "path", {row["path_id"] for row in paths}, REQUIRED_PATHS)
    require_set(failures, "gate", {row["gate_id"] for row in gates}, REQUIRED_GATES)

    for row in assets:
        check_claims(failures, "asset", row)
        if len(row["why_buyer_keeps_paying"]) < 40:
            failures.append(f"asset lacks renewal value language: {row['asset_id']}")
    for row in paths:
        if not row["stop_condition"]:
            failures.append(f"path missing stop condition: {row['path_id']}")
        if not row["required_evidence"]:
            failures.append(f"path missing required evidence: {row['path_id']}")
    for row in gates:
        check_claims(failures, "gate", row)
        if not row["renewal_signal"]:
            failures.append(f"gate missing renewal signal: {row['gate_id']}")

    required_phrases = [
        "why a buyer pays after the proposal",
        "maintained state of the system",
        "signage/routing",
        "EV support",
        "Stop or hold",
        "Do not close on ROI proof",
        "operator-approved action",
    ]
    for phrase in required_phrases:
        if phrase not in brief:
            failures.append(f"brief missing phrase: {phrase}")

    if failures:
        print("ROUTE DCR renewal gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("ROUTE DCR renewal gate: PASS")
    print("  checked renewal assets, paths, gates, blocked claims, and brief language")
    return 0


if __name__ == "__main__":
    sys.exit(main())
