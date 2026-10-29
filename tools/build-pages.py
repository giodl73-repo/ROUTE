"""Build ROUTE's bounded browser workbench from its native Rust core."""
import argparse
import csv
import hashlib
import io
import json
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def run(*args):
    subprocess.run(args, cwd=ROOT, check=True)



def canonical_blob(path):
    blob = subprocess.check_output(["git", "show", "HEAD:" + path], cwd=ROOT)
    if (ROOT / path).read_bytes().replace(b"\r\n", b"\n") != blob:
        raise ValueError("Uncommitted input differs from Git blob: " + path)
    return blob


def source_evidence(output):
    snapshot_path = "data/browser-i80-source-checklist-2026-10-29.json"
    contract_path = "data/i80-report-source-contract.csv"
    packet_path = "docs/packets/i80-flagship-review-packet.md"
    script_path = "tools/prepare_i80_report_sources.py"
    snapshot, contract, packet, script = map(canonical_blob, [snapshot_path, contract_path, packet_path, script_path])
    recorded = json.loads(snapshot)
    contracts = list(csv.DictReader(io.StringIO(contract.decode("utf-8"))))
    assert [row["contract"] for row in recorded["rows"]] == contracts
    for key, blob in [("contract_sha256", contract), ("preparation_script_sha256", script), ("reviewed_packet_sha256", packet)]:
        assert recorded[key] == hashlib.sha256(blob).hexdigest(), key
    assert b"HPMS 2023" in packet
    assert next(row for row in contracts if row["source_id"] == "SRC-I80-HPMS")["current_source_year"] == "2018"
    references = []
    for status in ["all", "ready", "blocked", "excluded"]:
        fixture = ROOT / f"fixtures/source-checklist-{status}.json"
        canonical = subprocess.check_output(["cargo", "run", "--quiet", "--locked", "-p", "route-web", "--bin", "route-source-checklist", "--", str(fixture)], cwd=ROOT, text=True, encoding="utf-8").strip()
        references.append({"input": fixture.read_text(encoding="utf-8").strip(), "canonical": canonical})
    assert json.loads(references[0]["canonical"])["selected_rows"] == recorded["rows"]
    legacy = [json.loads(line) for line in subprocess.check_output(["cargo", "run", "--quiet", "--locked", "-p", "route-web", "--example", "legacy_reference"], cwd=ROOT, text=True, encoding="utf-8").splitlines()]
    assert legacy == json.loads((ROOT / "fixtures/pre-checklist-legacy.json").read_text(encoding="utf-8"))["queries"]
    (ROOT / ".cache").mkdir(exist_ok=True)
    (ROOT / ".cache/source-checklist-references.json").write_text(json.dumps({"queries": references, "legacy": legacy}, indent=2), encoding="utf-8")
    sources = output / "sources"
    sources.mkdir(exist_ok=True)
    artifacts = []
    for name, path, blob in [("recorded-checklist.json", snapshot_path, snapshot), ("source-contract.csv", contract_path, contract), ("historical-review-packet.md", packet_path, packet)]:
        (sources / name).write_bytes(blob)
        artifacts.append({"path": "sources/" + name, "repository_path": path, "sha256": hashlib.sha256(blob).hexdigest(), "bytes": len(blob)})
    custody = {"schema": "route.source-artifact-custody.v1", "byte_scope": "exact committed LF repository metadata; not newly verified raw primary-source bytes", "artifacts": artifacts, "raw_primary_bytes_in_bundle": False, "primary_hashes_reverified": False, "vintage_discrepancy": recorded["vintage_discrepancy"]}
    (output / "source-custody.json").write_text(json.dumps(custody, indent=2), encoding="utf-8")
    (output / "source-note.md").write_bytes(canonical_blob("docs/browser-source-note.md"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("dist/ROUTE"))
    args = parser.parse_args()
    output = args.output.resolve()
    if output == ROOT / "dist" or ROOT / "dist" not in output.parents:
        raise ValueError("Output must be a directory inside dist")
    version = subprocess.check_output(["wasm-bindgen", "--version"], text=True).strip()
    if version != "wasm-bindgen 0.2.127":
        raise ValueError(f"Expected wasm-bindgen 0.2.127, found {version}")
    run("cargo", "build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "route-web", "--lib", "--features", "wasm")
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT))
    wasm = Path(metadata["target_directory"]) / "wasm32-unknown-unknown/release/route_web.wasm"
    output.mkdir(parents=True, exist_ok=True)
    run("wasm-bindgen", str(wasm), "--target", "web", "--out-dir", str(output / "pkg"))
    for name in ("index.html", "style.css", "app.js", "worker.js", "checklist.js", "checklist-worker.js"):
        shutil.copy2(ROOT / "web" / name, output / name)
    shutil.copy2(ROOT / "LICENSE", output / "LICENSE")
    shutil.copy2(ROOT / "THIRD-PARTY-LICENSES.txt", output / "THIRD-PARTY-LICENSES.txt")
    def capture(*args):
        return subprocess.check_output(args, cwd=ROOT, text=True, encoding="utf-8").strip()
    source_evidence(output)
    provenance = {"commit": capture("git", "rev-parse", "HEAD"), "dirty": bool(capture("git", "status", "--porcelain")), "rust": capture("rustc", "--version"), "bindings": version, "source": "https://github.com/giodl73-repo/ROUTE"}
    (output / "provenance.json").write_text(json.dumps(provenance, indent=2), encoding="utf-8")
    (output / ".nojekyll").touch()
    size = sum(path.stat().st_size for path in output.rglob("*") if path.is_file())
    if size >= 5_000_000:
        raise ValueError("Planner exceeds its 5 MB delivery budget")
    print(f"Pages planner: {size:,} bytes at {output}")


if __name__ == "__main__":
    main()
