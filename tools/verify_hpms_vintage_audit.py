"""Verify recorded first-party code evidence offline; never inspect/acquire a cache."""
import hashlib
import json
import subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]

def blob(path):
    data = subprocess.check_output(["git", "show", "HEAD:" + path], cwd=ROOT)
    actual = (ROOT / path).read_bytes().replace(b"\r\n", b"\n")
    if actual != data:
        raise ValueError("Uncommitted evidence input: " + path)
    return data

def verify():
    audit = json.loads(blob("data/browser-hpms-vintage-audit.json"))
    assert audit["schema"] == "route.hpms-label-origin-audit.v1"
    assert audit["configured_acquisition_year"] == 2018
    assert audit["packet_claimed_year"] == 2023
    assert audit["matched_packet_input_year"] is None
    for key in ["historical_execution_attested", "packet_values_relinked", "raw_publisher_bytes_verified", "historical_packet_changed", "historical_checklist_changed"]:
        assert audit[key] is False
    assert len(audit["evidence"]) == 9
    for evidence in audit["evidence"]:
        data = blob(evidence.get("frozen_copy", evidence["path"]))
        assert hashlib.sha256(data).hexdigest() == evidence["blob_sha256"]
        lines = data.decode("utf-8").splitlines(keepends=True)
        excerpt = "".join(lines[evidence["start_line"] - 1:evidence["end_line"]])
        assert excerpt == evidence["excerpt"]
        assert hashlib.sha256(excerpt.encode()).hexdigest() == evidence["excerpt_sha256"]
    for path in ["crates/route-score/src/score.rs", "crates/route-report/src/lib.rs"]:
        production = blob(path).decode().split("#[cfg(test)]")[0]
        assert "HPMS 2023" not in production
        assert "input source year unverified" in production
    recorded = audit["local_normalized_cache_observation"]
    assert not any(recorded[k] for k in ["estimators_or_populations_matched", "has_year_column", "raw_rows_published", "acquisition_performed"])
    assert recorded["i80_upper_median_aadt"] != recorded["packet_reported_graph_mean_aadt"]
    print("HPMS label audit: nine code excerpts/hashes and future citation guard verified; dated cache observation not re-verified")
    return audit

if __name__ == "__main__":
    verify()
