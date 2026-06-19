#!/usr/bin/env python3
"""Build Canada parser mapping ledger from field inventory and feature sample."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
INVENTORY = ROOT / "data" / "international-canada-source-field-inventory-001.csv"
SAMPLE = ROOT / "data" / "international-canada-road-graph-feature-sample-001.csv"
OUTPUT = ROOT / "data" / "international-canada-parser-mapping-ledger-001.csv"

FIELDS = [
    "mapping_id",
    "source_id",
    "source_field",
    "field_alias",
    "sample_values",
    "target_output_table",
    "target_column",
    "mapping_role",
    "mapping_status",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]

MAPPINGS = [
    ("roadclass", "source_class", "road_class", "candidate-class-field"),
    ("type_code", "source_class", "type_code", "candidate-class-field"),
    ("desc_en", "source_class", "nhs_description", "candidate-class-field"),
    ("rtnumber1", "route_id", "route_number_1", "candidate-route-field"),
    ("rtename1", "route_name", "route_name_1", "candidate-route-field"),
    ("Shape", "geometry_ref", None, "candidate-geometry-field"),
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def write_csv(path: Path, rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def sample_values(sample_rows: list[dict[str, str]], sample_column: str | None) -> str:
    if sample_column is None:
        return "not-requested"
    values = []
    for row in sample_rows:
        value = row.get(sample_column, "")
        if value not in values:
            values.append(value)
    return "|".join(values)


def main() -> None:
    inventory_rows = {
        row["field_name"]: row
        for row in read_csv(INVENTORY)
        if row["source_id"] == "CAN-SRC-001"
    }
    sample_rows = read_csv(SAMPLE)
    rows: list[dict[str, str]] = []

    for index, (field_name, target_column, sample_column, role) in enumerate(MAPPINGS, start=1):
        inventory = inventory_rows[field_name]
        rows.append(
            {
                "mapping_id": f"CAN-MAP-001-{index:03d}",
                "source_id": "CAN-SRC-001",
                "source_field": field_name,
                "field_alias": inventory["field_alias"],
                "sample_values": sample_values(sample_rows, sample_column),
                "target_output_table": "canada_source_link_candidates",
                "target_column": target_column,
                "mapping_role": role,
                "mapping_status": "candidate_mapping_not_promoted",
                "evidence_acceptance_status": "not-accepted",
                "blocked_claims": inventory["blocked_claims"],
                "next_action": "review mapping before parser extraction or fixture replacement",
            }
        )

    write_csv(OUTPUT, rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
