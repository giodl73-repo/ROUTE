#!/usr/bin/env python3
"""Validate Canada extraction candidates against bounded source sample rows."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-canada-road-graph-filtered-route-sample-001.csv"
CANDIDATES = ROOT / "data" / "international-canada-parser-extraction-candidates-001.csv"
OUTPUT = ROOT / "data" / "international-canada-source-row-validation-001.csv"

FIELDS = [
    "validation_id",
    "candidate_id",
    "source_sample_id",
    "object_id",
    "route_id_match",
    "route_name_match",
    "source_class_match",
    "geometry_posture_match",
    "source_id_match",
    "validation_status",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "geometry_acceptance;fixture_replacement;parsed_adapter;official_network;"
    "route_designation;engineering_precision;agency_approval;construction_ready;"
    "guaranteed_sla;roi;eligibility;compliance;endorsement;validation;"
    "public_readiness;external_readiness"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def write_csv(rows: list[dict[str, str]]) -> None:
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def normalize(value: str) -> str:
    return "" if value in {"None", "none", "null"} else value


def expected_route_name(sample: dict[str, str], route_id: str) -> str:
    return normalize(sample["route_name_1"]) or f"route-number-{route_id}"


def main() -> None:
    samples = read_csv(SAMPLE)
    candidates = read_csv(CANDIDATES)
    rows: list[dict[str, str]] = []
    for index, (sample, candidate) in enumerate(zip(samples, candidates), start=1):
        route_id = normalize(sample["route_number_1"]) or f"object-{sample['object_id']}"
        source_class = f"{sample['road_class']} | {sample['nhs_description']}"
        geometry_ref = f"not_requested:{sample['sample_method']}:{sample['object_id']}"
        checks = {
            "route_id_match": candidate["route_id"] == route_id,
            "route_name_match": candidate["route_name"] == expected_route_name(sample, route_id),
            "source_class_match": candidate["source_class"] == source_class,
            "geometry_posture_match": candidate["geometry_ref"] == geometry_ref
            and sample["geometry_status"] == "not-requested",
            "source_id_match": candidate["source_id"] == sample["source_id"] == "CAN-SRC-001",
        }
        rows.append(
            {
                "validation_id": f"CAN-SOURCE-ROW-VALIDATION-{index:03d}",
                "candidate_id": candidate["candidate_id"],
                "source_sample_id": sample["sample_id"],
                "object_id": sample["object_id"],
                "route_id_match": str(checks["route_id_match"]).lower(),
                "route_name_match": str(checks["route_name_match"]).lower(),
                "source_class_match": str(checks["source_class_match"]).lower(),
                "geometry_posture_match": str(checks["geometry_posture_match"]).lower(),
                "source_id_match": str(checks["source_id_match"]).lower(),
                "validation_status": "candidate_source_row_validated"
                if all(checks.values())
                else "candidate_source_row_mismatch",
                "blocked_claims": BLOCKED,
                "next_action": "geometry policy and fixture replacement closeout remain required",
            }
        )
    write_csv(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
