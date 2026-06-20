#!/usr/bin/env python3
"""Build EU source-content row validation for extraction candidates."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-eu-rhine-alpine-source-content-sample-001.csv"
CANDIDATES = ROOT / "data" / "international-eu-rhine-alpine-parser-extraction-candidates-001.csv"
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-source-content-row-validation-001.csv"

FIELDS = [
    "validation_id",
    "candidate_id",
    "source_id",
    "sample_id",
    "candidate_status",
    "sample_status",
    "validation_result",
    "allowed_use",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> None:
    samples = {row["source_id"]: row for row in read_csv(SAMPLE)}
    rows: list[dict[str, str]] = []
    for index, candidate in enumerate(read_csv(CANDIDATES), start=1):
        source = samples[candidate["source_id"]]
        rows.append(
            {
                "validation_id": f"EUR-CONTENT-ROWVAL-{index:03d}",
                "candidate_id": candidate["candidate_id"],
                "source_id": candidate["source_id"],
                "sample_id": source["sample_id"],
                "candidate_status": candidate["candidate_status"],
                "sample_status": source["sample_status"],
                "validation_result": "source_content_candidate_row_validated_not_road_feature",
                "allowed_use": "internal_parser_inspection_only",
                "blocked_claims": candidate["blocked_claims"],
                "next_action": "close current-corridor rebase and road-feature source selection before fixture replacement",
            }
        )
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
