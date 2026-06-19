#!/usr/bin/env python3
"""Build Canada adapter readiness ledgers from preflight rows.

The output is a machine-readable promotion ledger. It does not fetch, parse, or
validate Canadian source data; it decides what is ready for a future parser and
what remains source-needed or held.
"""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CUSTODY = ROOT / "data" / "international-canada-source-custody-preflight.csv"
COVERAGE = ROOT / "data" / "international-canada-adapter-coverage-preflight.csv"
FIELD_MAP = ROOT / "data" / "international-canada-adapter-field-map.csv"
READINESS = ROOT / "data" / "international-canada-source-adapter-readiness.csv"
GAPS = ROOT / "data" / "international-canada-source-adapter-gap-backlog.csv"


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def write_csv(path: Path, fieldnames: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def summarize_sources(source_ids: str, custody_by_id: dict[str, dict[str, str]]) -> str:
    if source_ids in {"none", "existing-route-labels", "internal-roles"}:
        return source_ids
    statuses: list[str] = []
    for source_id in source_ids.split(";"):
        source = custody_by_id.get(source_id)
        statuses.append(f"{source_id}:{source['source_status'] if source else 'missing'}")
    return "|".join(statuses)


def decision(coverage_result: str, source_binding_status: str) -> tuple[str, str, str]:
    if coverage_result == "source-candidate-found" and source_binding_status == "parse-ready-candidate":
        return (
            "ready_for_parse_not_promoted",
            "future source adapter may parse candidate source metadata and produce source-bound rows",
            "field_mapping_and_source_extraction_required",
        )
    if coverage_result == "preflight-ready":
        return (
            "carry_forward_internal",
            "safe to carry as adapter metadata or review control",
            "not_source_validation",
        )
    if coverage_result == "source-needed":
        return (
            "source_pack_required",
            "do not replace fixture rows until specific source custody is attached",
            "missing_source_custody",
        )
    if coverage_result == "held":
        return (
            "assumption_held",
            "keep as planning assumption only",
            "held_assumption",
        )
    return (
        "review_required",
        "manual review required before adapter use",
        "unknown_coverage_status",
    )


def main() -> None:
    custody_rows = read_csv(CUSTODY)
    coverage_rows = read_csv(COVERAGE)
    field_rows = read_csv(FIELD_MAP)
    custody_by_id = {row["source_id"]: row for row in custody_rows}
    coverage_by_field = {row["adapter_field"]: row for row in coverage_rows}

    readiness_rows: list[dict[str, str]] = []
    gap_rows: list[dict[str, str]] = []

    for field in field_rows:
        coverage = coverage_by_field[field["adapter_field"]]
        readiness_decision, allowed_use, gap_type = decision(
            coverage["coverage_result"], field["source_binding_status"]
        )
        row = {
            "adapter_field": field["adapter_field"],
            "coverage_result": coverage["coverage_result"],
            "source_binding_status": field["source_binding_status"],
            "source_status_summary": summarize_sources(field["candidate_source_ids"], custody_by_id),
            "readiness_decision": readiness_decision,
            "allowed_use": allowed_use,
            "blocked_claims": coverage["blocked_claims"],
            "next_action": coverage["next_evidence_step"],
        }
        readiness_rows.append(row)
        if readiness_decision != "carry_forward_internal":
            gap_rows.append(
                {
                    "adapter_field": field["adapter_field"],
                    "gap_type": gap_type,
                    "current_decision": readiness_decision,
                    "required_next_action": coverage["next_evidence_step"],
                    "blocked_claims": coverage["blocked_claims"],
                }
            )

    write_csv(
        READINESS,
        [
            "adapter_field",
            "coverage_result",
            "source_binding_status",
            "source_status_summary",
            "readiness_decision",
            "allowed_use",
            "blocked_claims",
            "next_action",
        ],
        readiness_rows,
    )
    write_csv(
        GAPS,
        [
            "adapter_field",
            "gap_type",
            "current_decision",
            "required_next_action",
            "blocked_claims",
        ],
        gap_rows,
    )
    print(f"wrote {READINESS}")
    print(f"wrote {GAPS}")


if __name__ == "__main__":
    main()
