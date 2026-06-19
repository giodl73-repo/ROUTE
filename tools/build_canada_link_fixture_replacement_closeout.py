#!/usr/bin/env python3
"""Build closeout for Canada internal link-fixture replacement."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LINKS = ROOT / "data" / "canada_source_link_candidates.csv"
EXTRACTION = ROOT / "data" / "international-canada-parser-extraction-candidates-001.csv"
VALIDATION = ROOT / "data" / "international-canada-source-row-validation-001.csv"
CONTRACT = ROOT / "data" / "international-canada-fixture-replacement-contract-001.csv"
OUTPUT = ROOT / "data" / "international-canada-link-fixture-replacement-closeout-001.csv"

FIELDS = [
    "closeout_id",
    "replacement_target",
    "replacement_source",
    "row_count",
    "source_row_validation_status",
    "geometry_contract",
    "replacement_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> None:
    links = read_csv(LINKS)
    extraction = read_csv(EXTRACTION)
    validation = read_csv(VALIDATION)
    contract = read_csv(CONTRACT)[0]
    validation_status = (
        "candidate_source_rows_validated"
        if validation and all(row["validation_status"] == "candidate_source_row_validated" for row in validation)
        else "candidate_source_rows_not_validated"
    )
    row = {
        "closeout_id": "CAN-LINK-FIXTURE-REPLACE-001",
        "replacement_target": "data/canada_source_link_candidates.csv",
        "replacement_source": "data/international-canada-parser-extraction-candidates-001.csv",
        "row_count": str(len(links)),
        "source_row_validation_status": validation_status,
        "geometry_contract": contract["geometry_contract"],
        "replacement_status": "internal_link_fixture_replaced"
        if len(links) == len(extraction)
        else "internal_link_fixture_not_replaced",
        "allowed_use": contract["allowed_use"],
        "blocked_claims": contract["blocked_claims"],
        "next_action": "keep map adapter official operational and external uses blocked",
    }
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerow(row)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
