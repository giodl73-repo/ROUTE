#!/usr/bin/env python3
"""Build Canada parser extraction candidates from bounded route samples."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-canada-road-graph-filtered-route-sample-001.csv"
CONTRACT = ROOT / "data" / "international-canada-parser-output-contract-001.csv"
OUTPUT = ROOT / "data" / "international-canada-parser-extraction-candidates-001.csv"

FIELDS = [
    "candidate_id",
    "source_id",
    "source_family",
    "route_id",
    "route_name",
    "source_class",
    "geometry_ref",
    "source_owner",
    "source_date",
    "access_note",
    "evidence_label",
    "candidate_status",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def write_csv(path: Path, rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def link_contract() -> dict[str, str]:
    for row in read_csv(CONTRACT):
        if row["output_table"] == "canada_source_link_candidates":
            return row
    raise RuntimeError("missing canada_source_link_candidates contract")


def normalize(value: str) -> str:
    return "" if value in {"None", "none", "null"} else value


def main() -> None:
    contract = link_contract()
    rows: list[dict[str, str]] = []
    for index, sample in enumerate(read_csv(SAMPLE), start=1):
        route_id = normalize(sample["route_number_1"]) or f"object-{sample['object_id']}"
        route_name = normalize(sample["route_name_1"]) or f"route-number-{route_id}"
        rows.append(
            {
                "candidate_id": f"CAN-EXTRACT-LINK-{index:03d}",
                "source_id": "CAN-SRC-001",
                "source_family": "road_graph",
                "route_id": route_id,
                "route_name": route_name,
                "source_class": f"{sample['road_class']} | {sample['nhs_description']}",
                "geometry_ref": f"not_requested:{sample['sample_method']}:{sample['object_id']}",
                "source_owner": "Government of Canada; Natural Resources Canada; Federal Geospatial Platform",
                "source_date": "2026-06-18",
                "access_note": "bounded no-geometry extraction candidate; not fixture replacement",
                "evidence_label": contract["required_label"],
                "candidate_status": "source_extraction_candidate_not_promoted",
                "blocked_claims": contract["blocked_columns_or_values"],
                "next_action": "role-review extracted candidates before replacing dry-run fixture rows",
            }
        )

    write_csv(OUTPUT, rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
