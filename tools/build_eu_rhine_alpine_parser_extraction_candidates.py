#!/usr/bin/env python3
"""Build EU Rhine-Alpine parser extraction candidates from bounded source content."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SAMPLE = ROOT / "data" / "international-eu-rhine-alpine-source-content-sample-001.csv"
CONTRACT = ROOT / "data" / "international-eu-rhine-alpine-parser-output-contract-001.csv"
OUTPUT = ROOT / "data" / "international-eu-rhine-alpine-parser-extraction-candidates-001.csv"

FIELDS = [
    "candidate_id",
    "source_id",
    "source_family",
    "route_or_layer_id",
    "route_or_layer_name",
    "source_class",
    "geometry_ref",
    "source_owner",
    "source_date",
    "access_note",
    "evidence_label",
    "candidate_status",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def link_contract() -> dict[str, str]:
    for row in read_csv(CONTRACT):
        if row["output_table"] == "eu_rhine_alpine_source_link_candidates":
            return row
    raise RuntimeError("missing eu_rhine_alpine_source_link_candidates contract")


def by_source(rows: list[dict[str, str]], source_id: str) -> dict[str, str]:
    for row in rows:
        if row["source_id"] == source_id:
            return row
    raise RuntimeError(f"missing source content sample for {source_id}")


def main() -> None:
    contract = link_contract()
    samples = read_csv(SAMPLE)
    tentec = by_source(samples, "EUR-SRC-002")
    corridor = by_source(samples, "EUR-SRC-004")
    rows = [
        {
            "candidate_id": "EUR-EXTRACT-LINK-001",
            "source_id": "EUR-SRC-004",
            "source_family": corridor["source_family"],
            "route_or_layer_id": "RALP-CONTEXT-001",
            "route_or_layer_name": "Rhine - Alpine corridor context",
            "source_class": "rail_corridor_context_not_road_service_network",
            "geometry_ref": "not_requested:official_context_page:no_geometry",
            "source_owner": corridor["source_owner"],
            "source_date": corridor["source_date"],
            "access_note": "bounded source-content extraction candidate; not fixture replacement",
            "evidence_label": contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": contract["blocked_columns_or_values"],
            "next_action": "run role review and current-corridor rebase before fixture replacement",
        },
        {
            "candidate_id": "EUR-EXTRACT-LINK-002",
            "source_id": "EUR-SRC-002",
            "source_family": tentec["source_family"],
            "route_or_layer_id": "TENTEC-LAYER-CONTEXT-001",
            "route_or_layer_name": "TENtec map/API layer context",
            "source_class": "network_viewer_layer_context_not_downloaded_feature",
            "geometry_ref": "not_requested:network_viewer_context:no_geometry",
            "source_owner": tentec["source_owner"],
            "source_date": tentec["source_date"],
            "access_note": "bounded layer-context extraction candidate; no layer feature row accepted",
            "evidence_label": contract["required_label"],
            "candidate_status": "source_content_extraction_candidate_not_promoted",
            "blocked_claims": contract["blocked_columns_or_values"],
            "next_action": "select inspectable TENtec layer fields before fixture replacement",
        },
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
