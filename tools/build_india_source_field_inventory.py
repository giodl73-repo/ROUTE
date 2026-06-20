#!/usr/bin/env python3
"""Build India source field inventory."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PROBE = ROOT / "data" / "international-india-source-payload-probe-001.csv"
OUTPUT = ROOT / "data" / "international-india-source-field-inventory-001.csv"

FIELDS = [
    "inventory_id",
    "source_id",
    "source_family",
    "inventory_basis",
    "candidate_fields",
    "inventory_status",
    "evidence_label",
    "blocked_claims",
    "next_action",
]

BLOCKED = (
    "official_corridor_designation;national_approval;state_approval;"
    "route_designation;source_row_validation;fixture_replacement;"
    "geometry_acceptance;topology_proof;map_overlay;terminal_performance;"
    "construction_ready;guaranteed_sla;travel_time_proof;"
    "delivery_commitment;numeric_roi;roi;eligibility;compliance;"
    "endorsement;validation;external_validation;public_readiness;"
    "external_readiness"
)

SOURCE_FAMILIES = {
    "IND-SRC-001": "highway_ministry_context",
    "IND-SRC-002": "highway_authority_context",
    "IND-SRC-003": "port_system_context",
    "IND-SRC-004": "port_statistics_context",
    "IND-SRC-005": "hierarchy_fixture_context",
    "IND-SRC-SLA-001": "service_targets",
}

FIELD_CANDIDATES = {
    "IND-SRC-001": "program name; publication date; highway policy context; document title; access note",
    "IND-SRC-002": "road asset context; network responsibility; project/highway reference; metadata pointer; access note",
    "IND-SRC-003": "major port name; port governance context; ministry division; access note",
    "IND-SRC-004": "port name; statistics period; cargo context; table title; publication link; access note",
    "IND-SRC-005": "candidate tier; service role; readiness basis; evidence label; claim boundary",
    "IND-SRC-SLA-001": "target id; target class; assumption label; local basis; numeracy review",
}


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main() -> None:
    probe_by_source = {row["source_id"]: row for row in read_csv(PROBE)}
    rows: list[dict[str, str]] = []
    for index, source_id in enumerate(SOURCE_FAMILIES, start=1):
        probe = probe_by_source[source_id]
        if source_id == "IND-SRC-SLA-001":
            status = "held_no_payload"
            label = "held"
            basis = "not-fetchable target assumption row"
            next_action = "create India target posture before any service target promotion"
        elif source_id == "IND-SRC-005":
            status = "local_fixture_reference_not_source_payload"
            label = "heuristic-held"
            basis = "local hierarchy fixture reference; evidence not accepted"
            next_action = "complete source-row validation role review and geometry policy before replacing fixture rows"
        elif probe["http_status"] == "200":
            status = "metadata_reachable_fields_not_parsed"
            label = "source-candidate"
            basis = f"http {probe['http_status']} {probe['content_type']} sample; evidence not accepted"
            next_action = "select exact parse fields before source-row validation or fixture replacement"
        else:
            status = "metadata_probe_not_usable"
            label = "source-needed"
            basis = f"http {probe['http_status']} {probe['probe_result']}; evidence not accepted"
            next_action = "resolve usable metadata source before parser promotion"
        rows.append(
            {
                "inventory_id": f"IND-FIELD-{index:03d}",
                "source_id": source_id,
                "source_family": SOURCE_FAMILIES[source_id],
                "inventory_basis": basis,
                "candidate_fields": FIELD_CANDIDATES[source_id],
                "inventory_status": status,
                "evidence_label": label,
                "blocked_claims": BLOCKED,
                "next_action": next_action,
            }
        )
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
