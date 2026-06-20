#!/usr/bin/env python3
"""Build India bounded source-row validation ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LINKS = ROOT / "data" / "india_source_link_candidates.csv"
NEEDS = ROOT / "data" / "india_source_need_candidates.csv"
NODES = ROOT / "data" / "india_source_node_candidates.csv"
TARGETS = ROOT / "data" / "india_service_target_candidates.csv"
INVENTORY = ROOT / "data" / "international-india-source-field-inventory-001.csv"
OUTPUT = ROOT / "data" / "international-india-source-row-validation-001.csv"

FIELDS = [
    "validation_id",
    "artifact_path",
    "row_id",
    "source_id",
    "row_label",
    "inventory_status",
    "validation_result",
    "allowed_use",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def classify(row: dict[str, str]) -> tuple[str, str, str]:
    label = row["evidence_label"]
    if label == "source-candidate":
        return (
            "bounded_metadata_match_not_validated",
            "internal_parser_inspection_only",
            "run role review and source-content extraction before fixture replacement",
        )
    if label == "heuristic-held":
        return (
            "heuristic_fixture_hold_preserved",
            "fixture_gap_tracking_only",
            "replace with source-derived rows before fixture replacement",
        )
    if label == "held":
        return (
            "held_assumption_preserved",
            "assumption_tracking_only",
            "create target posture before target promotion",
        )
    raise ValueError(f"unsupported evidence label: {label}")


def main() -> None:
    inventory = {row["source_id"]: row for row in read_csv(INVENTORY)}
    rows: list[dict[str, str]] = []
    sources = [
        ("data/india_source_link_candidates.csv", LINKS, "route_or_layer_id"),
        ("data/india_source_need_candidates.csv", NEEDS, "need_id"),
        ("data/india_source_node_candidates.csv", NODES, "node_id"),
        ("data/india_service_target_candidates.csv", TARGETS, "target_gap_id"),
    ]
    count = 1
    for artifact, path, row_id_field in sources:
        for row in read_csv(path):
            source_id = row.get("source_id", "IND-SRC-SLA-001")
            inv = inventory[source_id]
            result, allowed, next_action = classify(row)
            rows.append(
                {
                    "validation_id": f"IND-ROWVAL-{count:03d}",
                    "artifact_path": artifact,
                    "row_id": row[row_id_field],
                    "source_id": source_id,
                    "row_label": row["evidence_label"],
                    "inventory_status": inv["inventory_status"],
                    "validation_result": result,
                    "allowed_use": allowed,
                    "blocked_claims": row["blocked_claims"],
                    "next_action": next_action,
                }
            )
            count += 1
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
