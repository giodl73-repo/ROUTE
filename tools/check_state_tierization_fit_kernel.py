#!/usr/bin/env python3
"""Gate state tierization fit diagnostics."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PROFILE = ROOT / "data" / "state-tierization-fit-role-vector-profile-001.csv"
COVERAGE = ROOT / "data" / "state-tierization-fit-state-coverage-001.csv"
REVIEW = ROOT / "docs" / "reviews" / "state-tierization-fit-kernel-001.md"

REQUIRED_FAMILIES = {
    "statewide_trunk_gateway",
    "regional_redundancy_load_shedding",
    "rural_access_continuity",
    "terminal_local_access",
    "resilience_recovery_exposure",
    "maintenance_non_promotion",
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
PROHIBITED = {
    "official state tiers",
    "legal slas",
    "construction readiness",
    "numeric roi",
    "state approval",
    "source-backed full inventory",
}


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> int:
    failures: list[str] = []
    profile = read_csv(PROFILE)
    coverage = read_csv(COVERAGE)
    review = REVIEW.read_text(encoding="utf-8")

    families = {row["signal_family"] for row in profile}
    if families != REQUIRED_FAMILIES:
        failures.append(f"vector families mismatch: {sorted(families)}")
    if len(profile) != 6:
        failures.append("profile must contain six vector families")
    if len(coverage) != 40:
        failures.append(f"coverage must contain forty state samples, found {len(coverage)}")
    for row in profile:
        if int(row["sample_support_rows"]) <= 0:
            failures.append(f"{row['vector_id']} has no sample support")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['vector_id']} missing blocked claims: {sorted(missing)}")
    for row in coverage:
        if row["sample_rows"] != "6":
            failures.append(f"{row['state']} should have six sample rows")
        if row["fit_decision"] != "fit_sample_complete_source_inventory_required":
            failures.append(f"{row['state']} did not reach bounded complete sample decision")
        for role_field in ["t1_count", "t2_count", "t3_count", "t4_count"]:
            if int(row[role_field]) <= 0:
                failures.append(f"{row['state']} missing {role_field}")
        if int(row["m_count"]) + int(row["x_count"]) <= 0:
            failures.append(f"{row['state']} missing non-promotion row")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['state']} missing blocked claims: {sorted(missing)}")
    lower_review = review.lower()
    promoted_section = lower_review.split("## what this does not prove", 1)[0]
    if "state_fit_kernel_ready_for_source_inventory_adapter" not in review:
        failures.append("review missing fit kernel gate decision")
    for phrase in PROHIBITED:
        if phrase in promoted_section:
            failures.append(f"review may promote prohibited phrase: {phrase}")
    if failures:
        print("State tierization fit kernel gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("State tierization fit kernel gate: PASS")
    print("  checked vector families, forty state samples, role spread, non-promotion, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
