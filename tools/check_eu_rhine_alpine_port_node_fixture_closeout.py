#!/usr/bin/env python3
"""Gate EU Rhine-Alpine port node fixture replacement closeout."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CLOSEOUT = ROOT / "data" / "international-eu-rhine-alpine-port-node-fixture-closeout-001.csv"
NODES = ROOT / "data" / "eu_rhine_alpine_source_node_candidates.csv"
LABELS = ROOT / "data" / "eu_rhine_alpine_adapter_evidence_labels.csv"

FIELDS = [
    "closeout_id",
    "replacement_target",
    "replacement_source",
    "row_count",
    "role_review_status",
    "source_row_validation_status",
    "geometry_contract",
    "replacement_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]
REQUIRED_NODES = {"NLRTM", "BEANR", "ITGOA", "CHBSL", "DEDUI"}
REQUIRED_BLOCKS = {
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
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
        failures.append("EU port-node closeout columns do not match contract")
    if len(rows) != 1:
        failures.append("EU port-node closeout must have one row")

    actual_nodes = {row["node_id"] for row in node_rows}
    missing_nodes = REQUIRED_NODES - actual_nodes
    if missing_nodes:
        failures.append(f"EU node fixture missing nodes: {sorted(missing_nodes)}")

    label_keys = {
        (row["artifact_path"], row["row_id"], row["evidence_label"])
        for row in label_rows
    }
    for row in node_rows:
        if row["source_id"] != "EUR-SRC-003":
            failures.append(f"{row['node_id']} has wrong node source")
        if row["evidence_label"] != "source-candidate":
            failures.append(f"{row['node_id']} is not source-candidate")
        if "internal node fixture" not in row["access_note"]:
            failures.append(f"{row['node_id']} missing internal node fixture access note")
        if "geometry not read or accepted" not in row["access_note"]:
            failures.append(f"{row['node_id']} missing no-geometry access note")
        key = ("data/eu_rhine_alpine_source_node_candidates.csv", row["node_id"], row["evidence_label"])
        if key not in label_keys:
            failures.append(f"{row['node_id']} missing evidence-label carry-forward")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"{row['node_id']} missing blocked claims: {sorted(missing_blocks)}")

    for row in rows:
        if row["replacement_status"] != "internal_node_fixture_replaced_no_geometry":
            failures.append("EU port-node closeout did not record no-geometry internal replacement")
        if row["geometry_contract"] != "no_geometry_attribute_rows_only":
            failures.append("EU port-node closeout geometry contract is too broad")
        if row["allowed_use"] != "internal adapter node-candidate fixture rows only":
            failures.append("EU port-node closeout allowed use is too broad")
        if int(row["row_count"]) != len(node_rows):
            failures.append("EU port-node closeout row count does not match fixture")
        missing_blocks = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing_blocks:
            failures.append(f"EU port-node closeout missing blocked claims: {sorted(missing_blocks)}")
        if "before" not in row["next_action"]:
            failures.append("EU port-node closeout next action must preserve before dependency")

    if failures:
        print("EU Rhine-Alpine port node fixture closeout gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("EU Rhine-Alpine port node fixture closeout gate: PASS")
    print("  checked no-geometry internal node rows, evidence labels, allowed use, and claim blocks")
    return 0


if __name__ == "__main__":
    sys.exit(main())
