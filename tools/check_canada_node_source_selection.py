#!/usr/bin/env python3
"""Gate Canada node source-selection candidates."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SELECTION = ROOT / "data" / "international-canada-node-source-selection-001.csv"
PREFLIGHT = ROOT / "data" / "international-canada-adapter-promotion-preflight-001.csv"

FIELDS = [
    "selection_id",
    "source_id",
    "node_id",
    "node_label",
    "node_class",
    "source_url",
    "source_owner",
    "source_date",
    "selected_fields",
    "selection_status",
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
    fields, rows = read_csv(SELECTION)
    _, preflight_rows = read_csv(PREFLIGHT)
    failures: list[str] = []

    if fields != FIELDS:
        failures.append("node source-selection columns do not match required contract")
    if len(rows) != 3:
        failures.append("node source-selection must contain three named port source candidates")

    node_ids = {row["node_id"] for row in rows}
    missing_nodes = REQUIRED_NODES - node_ids
    if missing_nodes:
        failures.append(f"missing required node source candidates: {sorted(missing_nodes)}")

    promotion_holds = [
        row for row in preflight_rows
        if row["promotion_surface"] == "need_node_target_tables"
        and row["current_decision"] == "hold"
    ]
    if len(promotion_holds) != 1:
        failures.append("adapter promotion preflight must still hold need/node/target tables")

    for row in rows:
        if row["source_id"] != "CAN-SRC-005":
            failures.append(f"{row['selection_id']} is not tied to CAN-SRC-005")
        if row["node_class"] != "port_gateway":
            failures.append(f"{row['selection_id']} is not a port_gateway candidate")
        if not row["source_url"].startswith("https://"):
            failures.append(f"{row['selection_id']} does not use an https source URL")
        if row["selection_status"] != "source-selected-not-promoted":
            failures.append(f"{row['selection_id']} promoted the node source")
        if row["allowed_use"] != "node source-custody candidate only":
            failures.append(f"{row['selection_id']} allowed use is too broad")
        blocked = set(row["blocked_claims"].split(";"))
        missing_blocks = REQUIRED_BLOCKS - blocked
        if missing_blocks:
            failures.append(f"{row['selection_id']} missing blocked claims: {sorted(missing_blocks)}")

    if failures:
        print("Canada node source-selection gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada node source-selection gate: PASS")
    print("  checked named port sources, source-custody status, promotion hold, and claim blocks")
    return 0


if __name__ == "__main__":
    sys.exit(main())
