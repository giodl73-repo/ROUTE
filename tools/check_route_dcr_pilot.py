#!/usr/bin/env python3
"""Gate ROUTE DCR pilot scope artifacts."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "route-dcr-pilot-scope-001.md"
PHASES = DATA / "route-dcr-pilot-phases-001.csv"
CADENCE = DATA / "route-dcr-pilot-cadence-001.csv"
DELIVERABLES = DATA / "route-dcr-pilot-deliverables-001.csv"
ACCEPTANCE = DATA / "route-dcr-pilot-acceptance-001.csv"
NONCLAIMS = DATA / "route-dcr-pilot-nonclaims-001.csv"

REQUIRED_PHASES = {
    "scope_lock",
    "source_connection",
    "monitoring_setup",
    "simulation_cadence",
    "operator_review",
    "executive_readout",
    "renewal_decision",
}
REQUIRED_CADENCE = {
    "weekly_exceptions",
    "monthly_simulation",
    "event_driven_review",
    "operator_review",
    "executive_readout",
    "quarterly_closeout",
}
REQUIRED_DELIVERABLES = {
    "monitored_signal_ledger",
    "promise_at_risk_board",
    "simulation_comparison_packets",
    "switch_playbook_register",
    "signage_routing_advisory_queue",
    "ev_support_queue",
    "evidence_boundary_ledger",
    "executive_readout",
    "renewal_recommendation",
}
REQUIRED_ACCEPTANCE = {
    "sponsor_scope",
    "source_access",
    "operator_boundary",
    "tabletop_transition",
    "cadence_readiness",
    "claim_boundary",
    "renewal_decision",
}
REQUIRED_NONCLAIMS = {
    "traffic_control",
    "legal_detour",
    "incident_command",
    "guaranteed_sla",
    "ev_availability",
    "construction",
    "roi",
    "endorsement",
    "public_readiness",
    "procurement_readiness",
    "live_integration",
    "automated_control",
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


def main() -> int:
    failures: list[str] = []
    phases = read_csv(PHASES)
    cadence = read_csv(CADENCE)
    deliverables = read_csv(DELIVERABLES)
    acceptance = read_csv(ACCEPTANCE)
    nonclaims = read_csv(NONCLAIMS)
    brief = BRIEF.read_text(encoding="utf-8")

    require_set(failures, "phase", {row["phase_id"] for row in phases}, REQUIRED_PHASES)
    require_set(
        failures, "cadence", {row["cadence_id"] for row in cadence}, REQUIRED_CADENCE
    )
    require_set(
        failures,
        "deliverable",
        {row["deliverable_id"] for row in deliverables},
        REQUIRED_DELIVERABLES,
    )
    require_set(
        failures,
        "acceptance",
        {row["gate_id"] for row in acceptance},
        REQUIRED_ACCEPTANCE,
    )
    require_set(
        failures,
        "nonclaim",
        {row["blocked_claim"] for row in nonclaims},
        REQUIRED_NONCLAIMS,
    )

    for row in phases:
        if not row["authority_boundary"]:
            failures.append(f"phase missing authority boundary: {row['phase_id']}")
        if not row["exit_output"]:
            failures.append(f"phase missing exit output: {row['phase_id']}")
    for row in cadence + deliverables + acceptance:
        claims = split_claims(row["held_claims"])
        if not claims:
            failures.append(f"missing held claims on row: {row}")
        unknown = claims - REQUIRED_NONCLAIMS
        if unknown:
            failures.append(f"unknown held claims {sorted(unknown)} on row: {row}")
    for row in nonclaims:
        if not row["allowed_language"].startswith("ROUTE"):
            failures.append(f"nonclaim allowed language must start with ROUTE: {row}")

    required_phrases = [
        "30-90 day Decision Control Room pilot",
        "monitor service-promise drift",
        "signage/routing",
        "EV support posture",
        "ROUTE remains advisory",
        "not a live operations integration",
        "operator authority stays with the buyer",
    ]
    for phrase in required_phrases:
        if phrase not in brief:
            failures.append(f"brief missing phrase: {phrase}")

    if failures:
        print("ROUTE DCR pilot gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("ROUTE DCR pilot gate: PASS")
    print("  checked phases, cadence, deliverables, acceptance, nonclaims, and boundaries")
    return 0


if __name__ == "__main__":
    sys.exit(main())
