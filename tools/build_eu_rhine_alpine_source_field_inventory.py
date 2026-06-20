#!/usr/bin/env python3
"""Build EU Rhine-Alpine source field inventory."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PROBE = ROOT / "data" / "international-eu-rhine-alpine-source-payload-probe-001.csv"
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-source-field-inventory-001.csv"

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
    "official_corridor_designation;member_state_approval;route_designation;"
    "geometry_acceptance;topology_proof;terminal_performance;node_completeness;"
    "road_access_proof;construction_ready;guaranteed_sla;numeric_roi;roi;"
    "eligibility;compliance;endorsement;validation;external_validation;"
    "public_readiness;external_readiness"
)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


SOURCE_FAMILIES = {
    "EUR-SRC-001": "corridor_context",
    "EUR-SRC-002": "network_viewer",
    "EUR-SRC-003": "transport_geodata",
    "EUR-SRC-004": "rhine_alpine_context",
    "EUR-SRC-005": "rail_freight_context",
    "EUR-SRC-SLA-001": "service_targets",
}

FIELD_CANDIDATES = {
    "EUR-SRC-001": "corridor name; map publication date; corridor scope; access note",
    "EUR-SRC-002": "layer metadata; corridor layer name; network layer name; access note",
    "EUR-SRC-003": "dataset version; transport network class; scale; download format; access note",
    "EUR-SRC-004": "countries crossed; corridor description; corridor context; access note",
    "EUR-SRC-005": "freight corridor context; organization scope; access note",
    "EUR-SRC-SLA-001": "target id; role; target hours; basis; assumption label",
}


def main() -> None:
    probe_by_source = {row["source_id"]: row for row in read_csv(PROBE)}
    rows: list[dict[str, str]] = []
    for index, source_id in enumerate(SOURCE_FAMILIES, start=1):
        probe = probe_by_source[source_id]
        if source_id == "EUR-SRC-SLA-001":
            status = "held_no_payload"
            label = "held"
            basis = "not-fetchable target assumption row"
            next_action = "create EU target posture before any service target promotion"
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
                "inventory_id": f"EUR-FIELD-{index:03d}",
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
