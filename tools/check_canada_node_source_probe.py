#!/usr/bin/env python3
"""Gate Canada node source-probe output."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SELECTION = ROOT / "data" / "international-canada-node-source-selection-001.csv"
PROBE = ROOT / "data" / "international-canada-node-source-probe-001.csv"

FIELDS = [
    "probe_id",
    "selection_id",
    "node_id",
    "node_label",
    "source_url",
    "probe_method",
    "http_status",
    "final_url",
    "content_type",
    "bytes_sampled",
    "probe_result",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]

REQUIRED_BLOCKS = {
    "port_endorsement",
    "terminal_performance",
    "node_completeness",
    "throughput_proof",
    "road_access_proof",
    "construction_ready",
    "guaranteed_sla",
    "roi",
    "compliance",
    "endorsement",
    "validation",
    "public_readiness",
    "external_readiness",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    _, selection_rows = read_csv(SELECTION)
    fields, probe_rows = read_csv(PROBE)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("node source-probe columns do not match required contract")
    if len(probe_rows) != len(selection_rows):
        failures.append("node source-probe row count does not match selected node sources")

    selected_by_id = {row["selection_id"]: row for row in selection_rows}
    for row in probe_rows:
        selected = selected_by_id.get(row["selection_id"])
        if selected is None:
            failures.append(f"{row['selection_id']} missing from source selection")
            continue
        if row["node_id"] != selected["node_id"]:
            failures.append(f"{row['selection_id']} node id changed from selection")
        if row["source_url"] != selected["source_url"]:
            failures.append(f"{row['selection_id']} source URL changed from selection")
        if row["probe_method"] != "http-get-sample":
            failures.append(f"{row['selection_id']} was not sampled by HTTP")
        if row["http_status"] == "not-applicable":
            failures.append(f"{row['selection_id']} missing HTTP status")
        if int(row["bytes_sampled"]) < 0:
            failures.append(f"{row['selection_id']} has invalid bytes_sampled")
        if row["evidence_acceptance_status"] != "not-accepted":
            failures.append(f"{row['selection_id']} accepted probe evidence")
        if row["probe_result"] in {"accepted", "validated", "approved", "node_fixture_replaced"}:
            failures.append(f"{row['selection_id']} has prohibited promotion wording")
        blocked = set(row["blocked_claims"].split(";"))
        missing_blocks = REQUIRED_BLOCKS - blocked
        if missing_blocks:
            failures.append(f"{row['selection_id']} missing blocked claims: {sorted(missing_blocks)}")

    if failures:
        print("Canada node source-probe gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada node source-probe gate: PASS")
    print("  checked probe coverage, HTTP metadata posture, not-accepted status, and claim blocks")
    return 0


if __name__ == "__main__":
    sys.exit(main())
