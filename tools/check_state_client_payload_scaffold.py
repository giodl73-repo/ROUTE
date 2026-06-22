#!/usr/bin/env python3
"""Gate generic state client payload scaffold artifacts."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
REVIEW = ROOT / "docs" / "reviews" / "state-client-payload-scaffold-001.md"
MANIFEST = DATA / "state-client-payload-manifest-001.csv"
PREFLIGHT = DATA / "state-client-payload-preflight-001.csv"

TEMPLATES = {
    "PAYLOAD-SEGMENTS": (
        DATA / "state-client-payload-segment-template-001.csv",
        {"source_segment_id", "route_label", "from_ref", "to_ref", "owner_or_jurisdiction", "road_class"},
    ),
    "PAYLOAD-NODES": (
        DATA / "state-client-payload-priority-node-template-001.csv",
        {"node_id", "node_label", "node_class", "jurisdiction", "source_ref"},
    ),
    "PAYLOAD-TERMINALS": (
        DATA / "state-client-payload-terminal-access-template-001.csv",
        {"terminal_id", "terminal_class", "access_route_ref", "nearest_tier_node", "source_ref"},
    ),
    "PAYLOAD-FAILURES": (
        DATA / "state-client-payload-restriction-failure-template-001.csv",
        {"restriction_id", "segment_ref", "restriction_type", "failure_metric_ref", "source_ref"},
    ),
    "PAYLOAD-NON-PROMOTION": (
        DATA / "state-client-payload-non-promotion-template-001.csv",
        {"source_segment_id", "coverage_status", "non_promotion_reason", "review_owner"},
    ),
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


def fieldnames(path: Path) -> set[str]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.reader(f)
        return set(next(reader))


def main() -> int:
    failures: list[str] = []
    manifest = read_csv(MANIFEST)
    preflight = read_csv(PREFLIGHT)
    review = REVIEW.read_text(encoding="utf-8")

    manifest_ids = {row["payload_id"] for row in manifest}
    if manifest_ids != set(TEMPLATES):
        failures.append(f"manifest payload ids mismatch: {sorted(manifest_ids)}")
    if len(preflight) != len(TEMPLATES):
        failures.append("preflight must contain one row per template")
    for payload_id, (path, required) in TEMPLATES.items():
        if not path.exists():
            failures.append(f"missing template: {path}")
            continue
        fields = fieldnames(path)
        missing = required - fields
        if missing:
            failures.append(f"{payload_id} missing columns: {sorted(missing)}")
        rows = read_csv(path)
        if not rows:
            failures.append(f"{payload_id} must include at least one example row")
    for row in manifest + preflight:
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"{row.get('payload_id', row.get('preflight_id'))} missing blocked claims: {sorted(missing_blocks)}")
    if any(row["template_status"] != "pass" for row in preflight):
        failures.append("all scaffold template preflight rows must pass")
    if any(row["client_data_status"] != "not-provided" for row in preflight):
        failures.append("scaffold must not imply client data has been provided")
    if "state_client_payload_scaffold_ready_for_first_client_fill" not in review:
        failures.append("review missing scaffold gate decision")
    if "does not validate client data" not in review:
        failures.append("review must state client data is not validated")
    if failures:
        print("State client payload scaffold gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("State client payload scaffold gate: PASS")
    print("  checked templates, manifest, preflight status, and blocked claims")
    return 0


if __name__ == "__main__":
    sys.exit(main())
