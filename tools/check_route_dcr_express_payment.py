#!/usr/bin/env python3
"""Gate ROUTE DCR express payment policy artifacts."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
BRIEF = ROOT / "docs" / "briefs" / "route-dcr-express-payment-policy-001.md"
TIERS = DATA / "route-dcr-express-payment-tiers-001.csv"
POLICY = DATA / "route-dcr-express-payment-policy-001.csv"
NONCLAIMS = DATA / "route-dcr-express-payment-nonclaims-001.csv"
COCKPIT = ROOT / "docs" / "dcr" / "browser" / "route-dcr-cockpit.html"

REQUIRED_TIERS = {"standard", "priority", "verified"}
REQUIRED_POLICY = {
    "pricing_authority",
    "collection_authority",
    "refund_policy",
    "equity_policy",
    "service_boundary",
    "revenue_boundary",
}
REQUIRED_NONCLAIMS = {
    "pricing_authority",
    "collection",
    "refund_policy",
    "revenue_guarantee",
    "equity_policy",
    "guaranteed_sla",
    "procurement_readiness",
    "roi",
    "traffic_control",
    "public_readiness",
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
    tiers = read_csv(TIERS)
    policy = read_csv(POLICY)
    nonclaims = read_csv(NONCLAIMS)
    brief = BRIEF.read_text(encoding="utf-8")
    cockpit = COCKPIT.read_text(encoding="utf-8")

    require_set(failures, "tier", {row["tier_id"] for row in tiers}, REQUIRED_TIERS)
    require_set(
        failures, "policy", {row["policy_id"] for row in policy}, REQUIRED_POLICY
    )
    require_set(
        failures,
        "nonclaim",
        {row["blocked_claim"] for row in nonclaims},
        REQUIRED_NONCLAIMS,
    )

    for row in tiers:
        price = int(row["simulated_price_usd"])
        if row["tier_id"] == "standard" and price != 0:
            failures.append("standard tier must be zero price")
        if row["tier_id"] != "standard" and price <= 0:
            failures.append(f"paid tier must have positive simulated price: {row}")
        claims = split_claims(row["held_claims"])
        missing = {"pricing_authority", "collection", "revenue_guarantee"} - claims
        if missing:
            failures.append(f"tier missing payment holds {sorted(missing)}: {row}")
    for row in policy:
        if not row["required_owner"]:
            failures.append(f"policy row missing required owner: {row['policy_id']}")
        claims = split_claims(row["held_claims"])
        unknown = claims - REQUIRED_NONCLAIMS
        if unknown:
            failures.append(f"policy row has unknown held claims {sorted(unknown)}")
    for row in nonclaims:
        if not row["allowed_language"].startswith("ROUTE"):
            failures.append(f"nonclaim allowed language must start with ROUTE: {row}")

    required_phrases = [
        "operator payments",
        "does not set official prices",
        "collect payments",
        "refund",
        "equity",
        "revenue proxy",
        "not a forecast",
        "not pricing authority",
    ]
    for phrase in required_phrases:
        if phrase not in brief:
            failures.append(f"brief missing phrase: {phrase}")

    cockpit_phrases = [
        "Priority advisory",
        "Verified window",
        "revenueProxy",
        "pricing_status",
        "simulated only; owner authority required",
    ]
    for phrase in cockpit_phrases:
        if phrase not in cockpit:
            failures.append(f"cockpit missing phrase: {phrase}")

    if failures:
        print("ROUTE DCR express payment gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("ROUTE DCR express payment gate: PASS")
    print("  checked tiers, policy gates, nonclaims, cockpit hooks, and brief language")
    return 0


if __name__ == "__main__":
    sys.exit(main())
