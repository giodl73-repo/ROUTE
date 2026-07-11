from __future__ import annotations

import argparse
import csv
import os
import subprocess
import sys
import zipfile
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTRACT_PATH = ROOT / "data" / "i80-report-source-contract.csv"
DEFAULT_OUTPUT = ROOT / "data" / "cache" / "i80-report-source-readiness.csv"
I80_STATES = "CA,NV,UT,WY,NE,IA,IL,IN,OH,PA,NJ"
I80_STATE_SET = set(I80_STATES.split(","))
NO_CREDENTIAL_SOURCE_IDS = {
    "SRC-I80-TIGER",
    "SRC-I80-GAZETTEER",
    "SRC-I80-HPMS",
    "SRC-I80-FEMA",
}
READY_CAPABLE_STATUSES = {
    "automated",
    "automated-download-partial-extract",
    "automated-partial",
    "automated-endpoint-needs-health-check",
}


@dataclass
class Attempt:
    status: str
    detail: str


def load_contract(path: Path = CONTRACT_PATH) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as handle:
        rows = list(csv.DictReader(handle))
    if not rows:
        raise SystemExit(f"empty source contract: {path}")
    required = {
        "source_id",
        "artifact",
        "current_source_year",
        "acquisition_status",
        "blocking_gap",
        "next_action",
    }
    missing = required.difference(rows[0])
    if missing:
        raise SystemExit(f"source contract missing columns: {sorted(missing)}")
    return rows


def route_binary() -> Path | None:
    candidates = [
        ROOT / "target" / "debug" / "route.exe",
        ROOT / "target" / "debug" / "route",
    ]
    return next((path for path in candidates if path.exists()), None)


def ensure_route_binary() -> Path | None:
    existing = route_binary()
    if existing is not None:
        return existing

    if (ROOT / ".cargo" / "config.toml").exists():
        return None

    environment = dict(os.environ)
    environment["CARGO_PROFILE_DEV_DEBUG"] = "0"
    result = subprocess.run(
        ["cargo", "build", "-q", "--locked", "-p", "route"],
        cwd=ROOT,
        env=environment,
        check=False,
    )
    return route_binary() if result.returncode == 0 else None


def run_route(binary: Path, args: list[str]) -> Attempt:
    result = subprocess.run(
        [str(binary), *args],
        cwd=ROOT,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    output = "\n".join(
        part.strip() for part in [result.stdout, result.stderr] if part.strip()
    )
    if output:
        print(output)
    return Attempt(
        status="succeeded" if result.returncode == 0 else "failed",
        detail=output[-1000:] if output else f"exit code {result.returncode}",
    )


def extract_gazetteer() -> Attempt:
    archive_path = ROOT / "data" / "cache" / "2023_Gaz_counties_national.zip"
    if not archive_path.exists():
        return Attempt("blocked", f"missing {archive_path.relative_to(ROOT)}")
    try:
        with zipfile.ZipFile(archive_path) as archive:
            members = [
                name
                for name in archive.namelist()
                if name.endswith("counties_national.txt")
            ]
            if len(members) != 1:
                return Attempt(
                    "failed",
                    f"expected one county Gazetteer text file, found {len(members)}",
                )
            member = members[0]
            destination = ROOT / "data" / "cache" / Path(member).name
            content = archive.read(member)
            if not content.strip():
                return Attempt("failed", "Gazetteer text file is empty")
            temporary = destination.with_name(
                f"{destination.name}.{os.getpid()}.tmp"
            )
            with temporary.open("wb") as handle:
                handle.write(content)
                handle.flush()
                os.fsync(handle.fileno())
            temporary.replace(destination)
            return Attempt(
                "succeeded",
                f"extracted {destination.relative_to(ROOT)} ({len(content)} bytes)",
            )
    except (OSError, zipfile.BadZipFile) as error:
        return Attempt("failed", str(error))


def execute_no_credential_sources() -> dict[str, Attempt]:
    binary = ensure_route_binary()
    if binary is None:
        detail = (
            "ROUTE binary missing; clean clones build it with a compact locked "
            "profile, while local path-override users must build it explicitly"
        )
        return {
            source_id: Attempt("blocked", detail)
            for source_id in NO_CREDENTIAL_SOURCE_IDS
        }

    attempts: dict[str, Attempt] = {}
    manifest_attempt = run_route(binary, ["fetch"])
    attempts["SRC-I80-TIGER"] = manifest_attempt
    attempts["SRC-I80-GAZETTEER"] = (
        extract_gazetteer()
        if manifest_attempt.status == "succeeded"
        else Attempt("blocked", "manifest fetch failed")
    )
    attempts["SRC-I80-HPMS"] = run_route(
        binary, ["fetch-hpms", "--states", I80_STATES]
    )
    attempts["SRC-I80-FEMA"] = run_route(binary, ["fetch-fema-d1"])
    return attempts


def artifact_evidence(artifact: Path) -> tuple[int, str]:
    if not artifact.exists():
        return 0, "artifact missing"
    if artifact.stat().st_size == 0:
        return 0, "artifact is empty"

    if artifact.suffix.lower() == ".zip":
        try:
            with zipfile.ZipFile(artifact) as archive:
                members = [info for info in archive.infolist() if not info.is_dir()]
            return len(members), f"zip members={len(members)}"
        except zipfile.BadZipFile as error:
            return 0, f"invalid zip: {error}"

    if artifact.suffix.lower() == ".csv":
        try:
            with artifact.open(encoding="utf-8-sig", newline="") as handle:
                reader = csv.reader(handle)
                header = next(reader, [])
                records = sum(1 for _ in reader)
            if not header:
                return 0, "CSV header missing"
            return records, f"csv records={records}"
        except (OSError, UnicodeError, csv.Error) as error:
            return 0, f"CSV parse failed: {error}"

    try:
        lines = sum(1 for line in artifact.open(encoding="utf-8") if line.strip())
        return lines, f"nonempty lines={lines}"
    except (OSError, UnicodeError) as error:
        return 0, f"text parse failed: {error}"


def readiness_rows(
    contract: list[dict[str, str]], attempts: dict[str, Attempt]
) -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    for contract_row in contract:
        source_id = contract_row["source_id"]
        artifact = ROOT / contract_row["artifact"]
        count, evidence = artifact_evidence(artifact)
        attempt = attempts.get(source_id, Attempt("not-attempted", ""))
        artifact_ready = count > 0
        if source_id == "SRC-I80-HPMS":
            artifact_ready, count, evidence = hpms_i80_evidence(artifact)
        elif source_id == "SRC-I80-FEMA":
            artifact_ready, count, evidence = fema_i80_evidence(artifact)

        if (
            artifact_ready
            and contract_row["acquisition_status"] in READY_CAPABLE_STATUSES
        ):
            readiness = "ready"
            blocker = ""
        else:
            readiness = "blocked"
            blocker = contract_row["blocking_gap"]
            if evidence != "artifact missing":
                blocker = f"{blocker}; {evidence}" if blocker else evidence

        rows.append(
            {
                "source_id": source_id,
                "artifact": contract_row["artifact"],
                "current_source_year": contract_row["current_source_year"],
                "acquisition_status": contract_row["acquisition_status"],
                "attempt_status": attempt.status,
                "readiness_status": readiness,
                "evidence_count": str(count),
                "evidence_detail": evidence,
                "attempt_detail": attempt.detail.replace("\n", " | "),
                "blocking_gap": blocker,
                "next_action": contract_row["next_action"],
            }
        )
    return rows


def write_readiness(rows: list[dict[str, str]], output: Path) -> None:
    output.parent.mkdir(parents=True, exist_ok=True)
    fieldnames = list(rows[0])
    temporary = output.with_name(f"{output.name}.{os.getpid()}.tmp")
    with temporary.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)
    temporary.replace(output)


def gate(rows: list[dict[str, str]], source_ids: set[str]) -> list[str]:
    return [
        row["source_id"]
        for row in rows
        if row["source_id"] in source_ids and row["readiness_status"] != "ready"
    ]


def hpms_i80_evidence(artifact: Path) -> tuple[bool, int, str]:
    if not artifact.exists():
        return False, 0, "artifact missing"
    with artifact.open(encoding="utf-8-sig", newline="") as handle:
        reader = csv.DictReader(handle)
        rows = [row for row in reader if row.get("ROUTE_ID") == "I80"]
    covered = {row.get("STATE", "") for row in rows}
    missing = sorted(I80_STATE_SET.difference(covered))
    detail = f"I80 records={len(rows)} states={len(covered)}/{len(I80_STATE_SET)}"
    if missing:
        detail += f" missing={','.join(missing)}"
    return not missing and bool(rows), len(rows), detail


def fema_i80_evidence(artifact: Path) -> tuple[bool, int, str]:
    if not artifact.exists():
        return False, 0, "artifact missing"
    with artifact.open(encoding="utf-8-sig", newline="") as handle:
        reader = csv.DictReader(handle)
        rows = [row for row in reader if row.get("tile", "").startswith("I80-")]
    positive = sum(1 for row in rows if int(row.get("sfha_count", "0") or 0) > 0)
    return (
        bool(rows) and positive > 0,
        len(rows),
        f"I80 tiles={len(rows)} positive_tiles={positive}",
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--gate-no-credential", action="store_true")
    parser.add_argument("--gate-all", action="store_true")
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()

    contract = load_contract()
    attempts = execute_no_credential_sources() if args.execute else {}
    rows = readiness_rows(contract, attempts)
    write_readiness(rows, args.output)
    print(f"wrote {args.output.relative_to(ROOT)}")

    blocked = [row["source_id"] for row in rows if row["readiness_status"] == "blocked"]
    print(f"ready={len(rows) - len(blocked)} blocked={len(blocked)}")

    failures: list[str] = []
    if args.gate_no_credential:
        failures.extend(gate(rows, NO_CREDENTIAL_SOURCE_IDS))
    if args.gate_all:
        failures.extend(gate(rows, {row["source_id"] for row in rows}))
    if failures:
        raise SystemExit(f"source readiness gate failed: {sorted(set(failures))}")


if __name__ == "__main__":
    main()
