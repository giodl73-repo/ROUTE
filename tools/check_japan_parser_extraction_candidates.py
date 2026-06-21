#!/usr/bin/env python3
"""Gate Japan parser extraction candidates before fixture replacement."""

from __future__ import annotations

import csv
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CANDIDATES = ROOT / "data" / "international-japan-parser-extraction-candidates-001.csv"

FIELDS = [
    "candidate_id",
    "target_table",
    "source_id",
    "source_family",
    "candidate_key",
    "candidate_label",
    "candidate_class",
    "geometry_ref",
    "source_owner",
    "source_date",
    "access_note",
    "evidence_label",
    "candidate_status",
    "blocked_claims",
    "next_action",
]
REQUIRED_TARGETS = {
    "japan_source_link_candidates",
    "japan_source_need_candidates",
    "japan_source_node_candidates",
}
REQUIRED_BLOCKS = {
    "source_row_validation",
    "fixture_replacement",
    "parsed_adapter",
    "geometry_acceptance",
    "topology_proof",
    "map_overlay",
    "terminal_performance",
    "node_completeness",
    "road_access_proof",
    "throughput_proof",
    "guaranteed_sla",
    "roi",
    "validation",
    "external_validation",
    "public_readiness",
    "external_readiness",
}


def main() -> int:
    with CANDIDATES.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        fields = list(reader.fieldnames or [])
        rows = list(reader)
    failures: list[str] = []
    if fields != FIELDS:
        failures.append("Japan parser extraction candidate columns do not match contract")
    if len(rows) != 6:
        failures.append("Japan parser extraction candidate table must have six bounded rows")
    if {row["target_table"] for row in rows} != REQUIRED_TARGETS:
        failures.append("Japan extraction candidates must cover link, node, and need target tables")
    if not any(row["source_id"] == "JPN-SRC-004" and row["evidence_label"] == "source-needed" for row in rows):
        failures.append("Japan extraction candidates must preserve GSI source-needed link blocker")
    if sum(1 for row in rows if row["target_table"] == "japan_source_need_candidates") < 3:
        failures.append("Japan extraction candidates must include three need/context candidates")
    if sum(1 for row in rows if row["target_table"] == "japan_source_node_candidates") < 2:
        failures.append("Japan extraction candidates must include two port-node context candidates")
    for row in rows:
        if row["source_id"] == "JPN-SRC-004":
            if row["candidate_status"] != "source_content_extraction_blocked_not_promoted":
                failures.append("GSI blocker must remain blocked, not promoted")
        elif row["evidence_label"] != "source-candidate":
            failures.append(f"{row['candidate_id']} has wrong evidence label")
        if row["candidate_status"] == "source_content_extraction_candidate_promoted":
            failures.append(f"{row['candidate_id']} promotes candidate status")
        if not row["geometry_ref"].startswith("not_requested:"):
            failures.append(f"{row['candidate_id']} accepts geometry")
        if "before" not in row["next_action"]:
            failures.append(f"{row['candidate_id']} next action must preserve before dependency")
        if "validated" in row["candidate_class"]:
            failures.append(f"{row['candidate_id']} implies validation")
        missing = REQUIRED_BLOCKS - set(row["blocked_claims"].split(";"))
        if missing:
            failures.append(f"{row['candidate_id']} missing blocked claims: {sorted(missing)}")
    if failures:
        print("Japan parser extraction candidate gate: FAIL")
        for failure in failures:
            print(f"  - {failure}")
        return 1
    print("Japan parser extraction candidate gate: PASS")
    print("  checked target coverage, GSI blocker, no-geometry posture, labels, and not-promoted status")
    return 0


if __name__ == "__main__":
    sys.exit(main())
