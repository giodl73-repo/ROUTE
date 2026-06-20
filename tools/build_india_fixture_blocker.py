#!/usr/bin/env python3
"""Build India fixture replacement blocker."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ROW_VALIDATION = ROOT / "data" / "international-india-source-row-validation-001.csv"
ROLE_REVIEW = ROOT / "data" / "international-india-role-review-001.csv"
GEOMETRY_POLICY = ROOT / "data" / "international-india-geometry-policy-001.csv"
OUTPUT = ROOT / "data" / "international-india-fixture-blocker-001.csv"

FIELDS = [
    "blocker_id",
    "replacement_target",
    "current_rows",
    "role_review_status",
    "geometry_status",
    "replacement_decision",
    "allowed_use",
    "blocked_claims",
    "required_next_step",
]

BLOCKED_CLAIMS = (
    "official_network;official_corridor_designation;national_approval;"
    "state_approval;route_designation;source_row_validation;"
    "fixture_replacement;parsed_adapter;geometry_acceptance;topology_proof;"
    "map_overlay;terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;endorsement;"
    "validation;external_validation;public_readiness;external_readiness;"
    "internal_adapter_proof"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> None:
    validation_rows = read_csv(ROW_VALIDATION)
    role_rows = read_csv(ROLE_REVIEW)
    policy_rows = read_csv(GEOMETRY_POLICY)
    labels = sorted({row["row_label"] for row in validation_rows})
    role_status = "pass_with_holds" if role_rows and all(row["result"] == "pass_with_holds" for row in role_rows) else "not_passed"
    geometry_status = (
        "geometry_not_requested_policy_blocks_replacement"
        if policy_rows and all(row["current_geometry_status"] == "not_requested" for row in policy_rows)
        else "geometry_status_unclear"
    )
    rows = [
        {
            "blocker_id": "IND-FIXTURE-BLOCKER-001",
            "replacement_target": "India dry-run link/node/need/target fixture tables",
            "current_rows": f"{len(validation_rows)} dry-run rows with labels {','.join(labels)}",
            "role_review_status": role_status,
            "geometry_status": geometry_status,
            "replacement_decision": "blocked_source_rows_not_validated_geometry_not_accepted",
            "allowed_use": "gap tracking and source acquisition planning only",
            "blocked_claims": BLOCKED_CLAIMS,
            "required_next_step": "extract source-derived no-geometry rows or close geometry intake before any fixture replacement contract",
        }
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
