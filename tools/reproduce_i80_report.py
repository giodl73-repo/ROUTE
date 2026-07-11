from __future__ import annotations

import argparse
import csv
import hashlib
import re
import subprocess
from pathlib import Path

import prepare_i80_report_sources as sources


ROOT = Path(__file__).resolve().parents[1]
REGENERATED = ROOT / "data" / "cache" / "i80-regenerated.md"
COMPARISON = ROOT / "data" / "cache" / "i80-report-comparison.csv"
STATUS = ROOT / "data" / "cache" / "i80-reproduction-status.csv"
CANONICAL = ROOT / "corpus" / "existing" / "i80.md"


def write_status(rows: list[dict[str, str]]) -> None:
    STATUS.parent.mkdir(parents=True, exist_ok=True)
    temporary = STATUS.with_suffix(".tmp")
    with temporary.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    temporary.replace(STATUS)


def fail(message: str) -> None:
    write_status([{"status": "failed", "source_id": "", "detail": message}])


def clear_generated_outputs() -> None:
    for path in [REGENERATED, COMPARISON]:
        if path.exists():
            path.unlink()


def extract_key_facts(markdown: str) -> dict[str, str]:
    match = re.search(
        r"^## Key Facts\s*$\n(.*?)(?=^## |\Z)",
        markdown,
        re.MULTILINE | re.DOTALL,
    )
    if not match:
        raise ValueError("Key Facts section missing")
    facts: dict[str, str] = {}
    for line in match.group(1).splitlines():
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if len(cells) == 3 and cells[0] not in {"Fact", "---"}:
            facts[f"fact:{cells[0]}"] = cells[1]
    return facts


def extract_dimensions(markdown: str) -> dict[str, str]:
    dimensions: dict[str, str] = {}
    for line in markdown.splitlines():
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if len(cells) != 7 or not re.fullmatch(r"[A-D]\d", cells[1]):
            continue
        dimensions[f"dimension:{cells[1]}:score"] = cells[3]
        dimensions[f"dimension:{cells[1]}:confidence"] = cells[5]
        dimensions[f"dimension:{cells[1]}:justification"] = cells[6]
    return dimensions


def extract_summary(markdown: str) -> dict[str, str]:
    summary: dict[str, str] = {}
    for prefix, key in [
        ("**Band totals**", "summary:band_totals"),
        ("**Confidence**", "summary:confidence"),
        ("**Unavailable dimensions**", "summary:unavailable"),
    ]:
        line = next(
            (line.strip() for line in markdown.splitlines() if line.startswith(prefix)),
            "",
        )
        summary[key] = line
    return summary


def extract_section(markdown: str, heading: str) -> str:
    match = re.search(
        rf"^## {re.escape(heading)}\s*$\n(.*?)(?=^## |\Z)",
        markdown,
        re.MULTILINE | re.DOTALL,
    )
    return match.group(1).strip() if match else ""


def extract_frontmatter(markdown: str) -> str:
    match = re.match(r"^---\n(.*?)\n---\n", markdown, re.DOTALL)
    return match.group(1).strip() if match else ""


def comparison_rows(current: str, regenerated: str) -> list[dict[str, str]]:
    current_values = {
        **extract_key_facts(current),
        **extract_dimensions(current),
        **extract_summary(current),
        "document:frontmatter": extract_frontmatter(current),
        "section:Overview": extract_section(current, "Overview"),
        "section:Flagship Claim Holds": extract_section(
            current, "Flagship Claim Holds"
        ),
        "section:Sources": extract_section(current, "Sources"),
        "document:sha256": hashlib.sha256(current.encode("utf-8")).hexdigest(),
    }
    regenerated_values = {
        **extract_key_facts(regenerated),
        **extract_dimensions(regenerated),
        **extract_summary(regenerated),
        "document:frontmatter": extract_frontmatter(regenerated),
        "section:Overview": extract_section(regenerated, "Overview"),
        "section:Flagship Claim Holds": extract_section(
            regenerated, "Flagship Claim Holds"
        ),
        "section:Sources": extract_section(regenerated, "Sources"),
        "document:sha256": hashlib.sha256(regenerated.encode("utf-8")).hexdigest(),
    }
    return [
        {
            "field": field,
            "current": current_values.get(field, ""),
            "regenerated": regenerated_values.get(field, ""),
            "changed": str(
                current_values.get(field, "") != regenerated_values.get(field, "")
            ).lower(),
        }
        for field in sorted(set(current_values) | set(regenerated_values))
    ]


def write_comparison(rows: list[dict[str, str]]) -> None:
    temporary = COMPARISON.with_suffix(".tmp")
    with temporary.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    temporary.replace(COMPARISON)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execute", action="store_true")
    args = parser.parse_args()
    clear_generated_outputs()

    try:
        contract = sources.load_contract()
        attempts = sources.execute_no_credential_sources() if args.execute else {}
        readiness = sources.readiness_rows(contract, attempts)
        sources.write_readiness(readiness, sources.DEFAULT_OUTPUT)
    except Exception as error:
        fail(f"source preparation failed: {error}")
        raise
    blockers = sources.gate(
        readiness, {row["source_id"] for row in readiness}
    )
    if blockers:
        write_status(
            [
                {
                    "status": "blocked",
                    "source_id": source_id,
                    "detail": next(
                        row["blocking_gap"]
                        for row in readiness
                        if row["source_id"] == source_id
                    ),
                }
                for source_id in blockers
            ]
        )
        raise SystemExit(f"I-80 reproduction blocked: {blockers}")

    binary = sources.ensure_route_binary()
    if binary is None:
        fail("ROUTE binary unavailable")
        raise SystemExit("I-80 reproduction blocked: ROUTE binary unavailable")
    result = subprocess.run(
        [
            str(binary),
            "report",
            "I80",
            "--output",
            str(REGENERATED.relative_to(ROOT)),
        ],
        cwd=ROOT,
        check=False,
    )
    if result.returncode != 0:
        fail(f"route report failed: {result.returncode}")
        raise SystemExit(f"I-80 report generation failed: {result.returncode}")

    try:
        rows = comparison_rows(
            CANONICAL.read_text(encoding="utf-8"),
            REGENERATED.read_text(encoding="utf-8"),
        )
        write_comparison(rows)
    except Exception as error:
        fail(f"comparison failed: {error}")
        raise
    changed = sum(row["changed"] == "true" for row in rows)
    write_status(
        [{"status": "generated", "source_id": "", "detail": f"changed={changed}"}]
    )
    print(f"wrote {REGENERATED.relative_to(ROOT)}")
    print(f"wrote {COMPARISON.relative_to(ROOT)} changed={changed}")


if __name__ == "__main__":
    main()
