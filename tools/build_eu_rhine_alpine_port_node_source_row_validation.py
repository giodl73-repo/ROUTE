#!/usr/bin/env python3
"""Build EU Rhine-Alpine port-node source-row validation ledger."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-eu-rhine-alpine-port-node-record-sample-001.csv"
ROLE_REVIEW = ROOT / "data" / "international-eu-rhine-alpine-port-node-role-review-001.csv"
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-port-node-source-row-validation-001.csv"

FIELDS = [
    "validation_id",
    "sample_id",
    "port_id",
    "required_fields_present",
    "point_join_status",
    "role_review_status",
    "validation_result",
    "allowed_use",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> None:
    roles = read_csv(ROLE_REVIEW)
    role_status = (
        "pass_with_holds"
        if roles and all(row["result"] == "pass_with_holds" for row in roles)
        else "role_review_not_passed"
    )
    rows: list[dict[str, str]] = []
    for index, sample in enumerate(read_csv(SAMPLE), start=1):
        required_present = all(
            sample[field]
            for field in [
                "port_id",
                "port_name",
                "country_code",
                "nuts_code",
                "ten_code",
                "port_hierarchy",
            ]
        )
        point_join = sample["point_layer_join"]
        rows.append(
            {
                "validation_id": f"EUR-PORT-NODE-ROWVAL-{index:03d}",
                "sample_id": sample["sample_id"],
                "port_id": sample["port_id"],
                "required_fields_present": str(required_present).lower(),
                "point_join_status": point_join,
                "role_review_status": role_status,
                "validation_result": "candidate_attribute_row_validated_geometry_held"
                if required_present and point_join == "point_record_present_geometry_not_read" and role_status == "pass_with_holds"
                else "candidate_attribute_row_not_validated",
                "allowed_use": "internal node-candidate source-row validation only",
                "blocked_claims": sample["blocked_claims"],
                "next_action": "write node fixture contract before any internal node replacement",
            }
        )
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
