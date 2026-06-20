#!/usr/bin/env python3
"""Gate Canada node fixture replacement closeout."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CLOSEOUT = ROOT / "data" / "international-canada-node-fixture-replacement-closeout-001.csv"
NODES = ROOT / "data" / "canada_source_node_candidates.csv"
LABELS = ROOT / "data" / "canada_adapter_evidence_labels.csv"

FIELDS = [
    "closeout_id",
    "replacement_target",
    "replacement_source",
    "row_count",
    "role_review_status",
    "replacement_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_NODES = {
    "CAN-PORT-VANCOUVER",
    "CAN-PORT-MONTREAL",
    "CAN-PORT-HALIFAX",
}
REQUIRED_BLOCKS = {
    "terminal_performance",
    "node_completeness",
    "road_access_proof",
    "throughput_proof",
    "guaranteed_sla",
    "roi",
    "validation",
    "public_readiness",
    "external_readiness",
}


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    fields, rows = read_csv(CLOSEOUT)
    _, node_rows = read_csv(NODES)
    _, label_rows = read_csv(LABELS)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("node replacement closeout columns do not match required contract")
    if len(rows) != 1:
        failures.append("node replacement closeout must have one row")

    actual_nodes = {row["node_id"] for row in node_rows}
    if REQUIRED_NODES - actual_nodes:
        failures.append(f"node fixture missing nodes: {sorted(REQUIRED_NODES - actual_nodes)}")
    label_keys = {
        (row["artifact_path"], row["row_id"], row["evidence_label"])
        for row in label_rows
    }
    for row in node_rows:
        if row["evidence_label"] != "source-candidate":
            failures.append(f"{row['node_id']} is not source-candidate")
        if "internal node fixture" not in row["access_note"]:
            failures.append(f"{row['node_id']} missing internal node fixture access note")
        key = ("data/canada_source_node_candidates.csv", row["node_id"], row["evidence_label"])
        if key not in label_keys:
            failures.append(f"{row['node_id']} missing evidence-label carry-forward")

    for row in rows:
        if row["replacement_status"] != "internal_node_fixture_replaced":
            failures.append("node replacement closeout did not replace internal fixture")
        if row["allowed_use"] != "internal adapter node-catalog fixture rows only":
            failures.append("node replacement closeout allowed use is too broad")
        if int(row["row_count"]) != len(node_rows):
            failures.append("node replacement closeout row count does not match fixture")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"node closeout missing blocked claims: {sorted(missing_blocks)}")

    if failures:
        print("Canada node fixture replacement closeout gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada node fixture replacement closeout gate: PASS")
    print("  checked internal node fixture rows, evidence labels, allowed use, and claim blocks")
    return 0


if __name__ == "__main__":
    sys.exit(main())
