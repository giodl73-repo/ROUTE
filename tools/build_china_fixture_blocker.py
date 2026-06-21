#!/usr/bin/env python3
"""Build China fixture replacement blocker."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ROLE_REVIEW = ROOT / "data" / "international-china-dry-run-role-review-001.csv"
LINKS = ROOT / "data" / "china_source_link_candidates.csv"
NODES = ROOT / "data" / "china_source_node_candidates.csv"
NEEDS = ROOT / "data" / "china_source_need_candidates.csv"
TARGETS = ROOT / "data" / "china_service_target_candidates.csv"
OUTPUT = ROOT / "data" / "international-china-fixture-blocker-001.csv"

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
    "official_network;official_corridor_designation;policy_alignment;"
    "route_designation;source_row_validation;fixture_replacement;"
    "parsed_adapter;geometry_acceptance;topology_proof;map_overlay;"
    "terminal_performance;node_completeness;road_access_proof;"
    "throughput_proof;construction_ready;guaranteed_sla;"
    "travel_time_proof;delivery_commitment;numeric_roi;roi;"
    "eligibility;compliance;endorsement;validation;external_validation;"
    "public_readiness;external_readiness;internal_adapter_proof"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> None:
    role_rows = read_csv(ROLE_REVIEW)
    link_rows = read_csv(LINKS)
    node_rows = read_csv(NODES)
    need_rows = read_csv(NEEDS)
    target_rows = read_csv(TARGETS)
    role_status = "pass_with_holds" if role_rows and all(row["result"] == "pass_with_holds" for row in role_rows) else "not_passed"
    link_labels = sorted({row["evidence_label"] for row in link_rows})
    node_labels = sorted({row["evidence_label"] for row in node_rows})
    need_labels = sorted({row["evidence_label"] for row in need_rows})
    target_labels = sorted({row["evidence_label"] for row in target_rows})
    geometry_refs = sorted({row["geometry_ref"] for row in link_rows})
    rows = [
        {
            "blocker_id": "CHN-FIXTURE-BLOCKER-001",
            "replacement_target": "China dry-run link/node/need/target fixture tables",
            "current_rows": f"links={len(link_rows)}:{','.join(link_labels)};nodes={len(node_rows)}:{','.join(node_labels)};needs={len(need_rows)}:{','.join(need_labels)};targets={len(target_rows)}:{','.join(target_labels)}",
            "role_review_status": role_status,
            "geometry_status": "geometry_not_accepted:" + ",".join(geometry_refs),
            "replacement_decision": "blocked_dry_run_rows_not_source_validated_geometry_not_accepted",
            "allowed_use": "gap tracking and source acquisition planning only",
            "blocked_claims": BLOCKED_CLAIMS,
            "required_next_step": "create source-row validation and geometry policy before any fixture replacement contract",
        }
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
