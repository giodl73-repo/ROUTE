#!/usr/bin/env python3
"""Gate Canada parser extraction candidates before fixture replacement."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CANDIDATES = ROOT / "data" / "international-canada-parser-extraction-candidates-001.csv"

FIELDS = [
    "candidate_id",
    "source_id",
    "source_family",
    "route_id",
    "route_name",
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


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return list(reader.fieldnames or []), list(reader)


def main() -> int:
    fields, rows = read_csv(CANDIDATES)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("extraction candidate columns do not match required contract")
    if not rows:
        failures.append("extraction candidate table has no rows")
    if len(rows) > 5:
        failures.append("extraction candidate table exceeds bounded five-row sample")
    if not any(row["route_name"] == "Autoroute Transcanadienne" for row in rows):
        failures.append("extraction candidates do not preserve sampled route name")
    for row in rows:
        if row["source_id"] != "CAN-SRC-001":
            failures.append(f"{row['candidate_id']} is not CAN-SRC-001")
        if row["evidence_label"] != "parse-ready-candidate":
            failures.append(f"{row['candidate_id']} has wrong evidence label")
        if row["candidate_status"] != "source_extraction_candidate_not_promoted":
            failures.append(f"{row['candidate_id']} promotes candidate status")
        if not row["geometry_ref"].startswith("not_requested:"):
            failures.append(f"{row['candidate_id']} has accepted geometry reference")
        if not row["blocked_claims"]:
            failures.append(f"{row['candidate_id']} missing blocked claims")
        if not row["route_id"] or not row["route_name"] or not row["source_class"]:
            failures.append(f"{row['candidate_id']} missing required link candidate values")

    if failures:
        print("Canada parser extraction candidate gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1

    print("Canada parser extraction candidate gate: PASS")
    print("  checked candidate values, no-geometry posture, evidence labels, and not-promoted status")
    return 0


if __name__ == "__main__":
    sys.exit(main())
