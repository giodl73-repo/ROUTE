#!/usr/bin/env python3
"""Build Canada target posture closeout."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TARGETS = ROOT / "data" / "canada_service_target_candidates.csv"
OUTPUT = ROOT / "data" / "international-canada-target-posture-001.csv"

FIELDS = [
    "posture_id",
    "target_table",
    "row_count",
    "target_status",
    "allowed_use",
    "blocked_claims",
    "next_action",
]

BLOCKED_CLAIMS = (
    "guaranteed_sla;travel_time_proof;delivery_commitment;official_approval;"
    "construction_ready;roi;compliance;endorsement;validation;public_readiness;"
    "external_readiness"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> None:
    target_rows = read_csv(TARGETS)
    rows = [
        {
            "posture_id": "CAN-TARGET-POSTURE-001",
            "target_table": "data/canada_service_target_candidates.csv",
            "row_count": str(len(target_rows)),
            "target_status": "held_planning_assumptions_accepted_for_internal_proof",
            "allowed_use": "internal adapter proof with explicit target holds only",
            "blocked_claims": BLOCKED_CLAIMS,
            "next_action": "do not promote SLA until target source and numeracy review close",
        }
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
