#!/usr/bin/env python3
"""Build a role-review gate for Canada fixture replacement readiness."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DRY_RUN_LINKS = ROOT / "data" / "canada_source_link_candidates.csv"
EXTRACTED_LINKS = ROOT / "data" / "international-canada-parser-extraction-candidates-001.csv"
OUTPUT = ROOT / "data" / "international-canada-fixture-replacement-role-review-001.csv"

FIELDS = [
    "review_id",
    "role_lane",
    "role_source",
    "input_compared",
    "decision",
    "finding",
    "required_next_step",
    "blocked_claims",
]

BLOCKED = (
    "fixture_replacement;source_row_validation;geometry_acceptance;parsed_adapter;"
    "official_network;route_designation;agency_approval;construction_ready;"
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


def main() -> None:
    dry_run = read_csv(DRY_RUN_LINKS)
    extracted = read_csv(EXTRACTED_LINKS)
    dry_run_path = DRY_RUN_LINKS.relative_to(ROOT).as_posix()
    extracted_path = EXTRACTED_LINKS.relative_to(ROOT).as_posix()
    compared = (
        f"{dry_run_path}:{len(dry_run)} dry-run rows vs "
        f"{extracted_path}:{len(extracted)} extracted rows"
    )

    rows = [
        {
            "review_id": "CAN-REPLACE-ROLE-001",
            "role_lane": "Scope Keeper",
            "role_source": ".roles/editorial/scope-keeper.md",
            "input_compared": compared,
            "decision": "pass_with_risk",
            "finding": "source-derived candidates now occupy the internal link fixture under closeout scope",
            "required_next_step": "keep non-link fixture replacement map adapter and external uses blocked",
            "blocked_claims": BLOCKED,
        },
        {
            "review_id": "CAN-REPLACE-ROLE-002",
            "role_lane": "Citation Auditor",
            "role_source": ".roles/editorial/citation-auditor.md",
            "input_compared": compared,
            "decision": "pass_with_risk",
            "finding": "source owner date access note evidence label blocked claims and source-row validation are carried into the internal link fixture",
            "required_next_step": "preserve source custody and blocked claims in any future parser closeout",
            "blocked_claims": BLOCKED,
        },
        {
            "review_id": "CAN-REPLACE-ROLE-003",
            "role_lane": "Schematic Cartographer",
            "role_source": ".roles/parliament/schematic-cartographer.md",
            "input_compared": compared,
            "decision": "hold_for_map_or_fixture_use",
            "finding": "no-geometry rows cannot support map overlay or topology proof",
            "required_next_step": "define geometry policy and map caption posture before any map-facing replacement",
            "blocked_claims": BLOCKED,
        },
        {
            "review_id": "CAN-REPLACE-ROLE-004",
            "role_lane": "Traffic Engineer",
            "role_source": ".roles/parliament/traffic-engineer.md",
            "input_compared": compared,
            "decision": "hold_for_operational_claims",
            "finding": "route number class and name fields do not prove capacity reliability geometry safety or throughput",
            "required_next_step": "require operational source fields or explicit no-operational-claim posture before promotion",
            "blocked_claims": BLOCKED,
        },
        {
            "review_id": "CAN-REPLACE-ROLE-005",
            "role_lane": "State DOT Planner",
            "role_source": ".roles/stakeholders/state-dot.md",
            "input_compared": compared,
            "decision": "hold_for_authority_and_delivery_claims",
            "finding": "extracted rows do not establish Canadian authority review funding eligibility project delivery or designation",
            "required_next_step": "keep approval eligibility delivery and designation claims blocked unless jurisdiction-specific sources close",
            "blocked_claims": BLOCKED,
        },
    ]
    write_csv(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
