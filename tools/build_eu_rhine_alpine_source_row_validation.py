#!/usr/bin/env python3
"""Build EU Rhine-Alpine bounded source-row validation ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LINKS = ROOT / "data" / "eu_rhine_alpine_source_link_candidates.csv"
NEEDS = ROOT / "data" / "eu_rhine_alpine_source_need_candidates.csv"
NODES = ROOT / "data" / "eu_rhine_alpine_source_node_candidates.csv"
TARGETS = ROOT / "data" / "eu_rhine_alpine_service_target_candidates.csv"
INVENTORY = ROOT / "data" / "international-eu-rhine-alpine-source-field-inventory-001.csv"
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-source-row-validation-001.csv"

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


def main() -> None:
    inventory = {row["source_id"]: row for row in read_csv(INVENTORY)}
    rows: list[dict[str, str]] = []
    sources = [
        ("data/eu_rhine_alpine_source_link_candidates.csv", LINKS, "route_or_layer_id"),
        ("data/eu_rhine_alpine_source_need_candidates.csv", NEEDS, "need_id"),
        ("data/eu_rhine_alpine_source_node_candidates.csv", NODES, "node_id"),
        ("data/eu_rhine_alpine_service_target_candidates.csv", TARGETS, "target_gap_id"),
    ]
    count = 1
    for artifact, path, row_id_field in sources:
        for row in read_csv(path):
            source_id = row.get("source_id", "EUR-SRC-SLA-001")
            inv = inventory[source_id]
            if row["evidence_label"] == "source-candidate":
                result = "bounded_metadata_match_not_validated"
                allowed = "internal_parser_inspection_only"
                next_action = "select exact parse fields and rerun role review before fixture replacement"
            elif row["evidence_label"] == "source-needed":
                result = "source_gap_preserved"
                allowed = "gap_tracking_only"
                next_action = "select source custody before row promotion"
            else:
                result = "held_assumption_preserved"
                allowed = "assumption_tracking_only"
                next_action = "create target posture before target promotion"
            rows.append(
                {
                    "validation_id": f"EUR-ROWVAL-{count:03d}",
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
