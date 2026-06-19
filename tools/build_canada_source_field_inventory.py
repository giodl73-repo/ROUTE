#!/usr/bin/env python3
"""Build Canada source field inventory from resolved payload metadata."""

from __future__ import annotations

import csv
import json
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ACCESS = ROOT / "data" / "international-canada-source-payload-access-001.csv"
RESOLUTION = ROOT / "data" / "international-canada-source-payload-resolution-001.csv"
OUTPUT = ROOT / "data" / "international-canada-source-field-inventory-001.csv"
USER_AGENT = "ROUTE-Canada-field-inventory/0.1 evidence-not-accepted"

FIELDS = [
    "inventory_id",
    "source_id",
    "source_family",
    "inventory_method",
    "inventory_status",
    "field_name",
    "field_type",
    "field_alias",
    "required_field_match",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    if not path.exists():
        return []
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def write_csv(path: Path, rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def layer_metadata_url(source_id: str) -> str | None:
    for row in read_csv(RESOLUTION):
        if row["source_id"] == source_id and row["resolution_type"] == "esri-rest-layer":
            return row["resolved_url"]
    return None


def fetch_json(url: str) -> dict:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(request, timeout=30) as response:
        return json.loads(response.read().decode("utf-8"))


def required_match(field_name: str, field_alias: str, required_fields: str) -> str:
    haystack = f"{field_name} {field_alias}".lower()
    required = required_fields.lower()
    if "route" in required and ("route" in haystack or "name" in haystack):
        return "candidate-route-field"
    if "source class" in required and ("class" in haystack or "type" in haystack):
        return "candidate-class-field"
    if "geometry" in required and "shape" in haystack:
        return "candidate-geometry-field"
    return "unmatched"


def main() -> None:
    rows: list[dict[str, str]] = []
    for source in read_csv(ACCESS):
        source_id = source["source_id"]
        layer_url = layer_metadata_url(source_id)
        if layer_url:
            metadata = fetch_json(layer_url)
            for index, field in enumerate(metadata.get("fields", []), start=1):
                name = field.get("name", "")
                alias = field.get("alias", "")
                rows.append(
                    {
                        "inventory_id": f"CAN-FIELD-{source_id}-{index:03d}",
                        "source_id": source_id,
                        "source_family": source["source_family"],
                        "inventory_method": "esri-rest-layer-fields",
                        "inventory_status": "field-candidate-not-accepted",
                        "field_name": name,
                        "field_type": field.get("type", ""),
                        "field_alias": alias,
                        "required_field_match": required_match(
                            name, alias, source["required_fields"]
                        ),
                        "evidence_acceptance_status": "not-accepted",
                        "blocked_claims": source["blocked_claims"],
                        "next_action": "map candidate fields to parser output contract before extraction",
                    }
                )
            continue

        rows.append(
            {
                "inventory_id": f"CAN-FIELD-{source_id}-HELD",
                "source_id": source_id,
                "source_family": source["source_family"],
                "inventory_method": source["access_mode"],
                "inventory_status": "field-inventory-held",
                "field_name": "none",
                "field_type": "none",
                "field_alias": "none",
                "required_field_match": "manual-or-source-selection-required",
                "evidence_acceptance_status": "not-accepted",
                "blocked_claims": source["blocked_claims"],
                "next_action": source["next_action"],
            }
        )

    write_csv(OUTPUT, rows)
    print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
