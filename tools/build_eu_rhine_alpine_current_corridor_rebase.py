#!/usr/bin/env python3
"""Build EU current-corridor rebase review for Rhine-Alpine work."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-eu-rhine-alpine-source-content-sample-001.csv"
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-current-corridor-rebase-001.csv"

FIELDS = [
    "review_id",
    "current_corridor_source_id",
    "legacy_context_source_id",
    "observed_current_corridor_hint",
    "observed_legacy_context_hint",
    "rebase_decision",
    "blocked_replacement_surface",
    "blocked_claims",
    "required_next_step",
]

BLOCKED = (
    "official_route_designation;member_state_approval;route_designation;"
    "geometry_acceptance;topology_proof;map_overlay;terminal_performance;"
    "node_completeness;road_access_proof;construction_ready;guaranteed_sla;"
    "travel_time_proof;delivery_commitment;numeric_roi;roi;eligibility;"
    "compliance;endorsement;validation;external_validation;public_readiness;"
    "external_readiness;internal_adapter_proof;fixture_replacement"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def sample(source_id: str) -> dict[str, str]:
    for row in read_csv(SAMPLE):
        if row["source_id"] == source_id:
            return row
    raise RuntimeError(f"missing source sample {source_id}")


def main() -> None:
    current = sample("EUR-SRC-001")
    legacy = sample("EUR-SRC-004")
    rows = [
        {
            "review_id": "EUR-REBASE-001",
            "current_corridor_source_id": current["source_id"],
            "legacy_context_source_id": legacy["source_id"],
            "observed_current_corridor_hint": current["route_or_dataset_hint"],
            "observed_legacy_context_hint": legacy["route_or_dataset_hint"],
            "rebase_decision": "current_corridor_rebase_required_before_replacement",
            "blocked_replacement_surface": "data/eu_rhine_alpine_source_link_candidates.csv;maps/international/eu-rhine-alpine-region.svg;internal adapter proof",
            "blocked_claims": BLOCKED,
            "required_next_step": "choose current European Transport Corridor scope or explicitly keep Rhine-Alpine as legacy context before fixture replacement",
        }
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
