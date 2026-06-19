#!/usr/bin/env python3
"""Probe selected Canada node sources without accepting node evidence."""

from __future__ import annotations

import csv
import sys
import urllib.error
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SELECTION = ROOT / "data" / "international-canada-node-source-selection-001.csv"
OUTPUT = ROOT / "data" / "international-canada-node-source-probe-001.csv"
USER_AGENT = "ROUTE-Canada-node-source-probe/0.1 evidence-not-accepted"
TIMEOUT_SECONDS = 20
SAMPLE_BYTES = 65536

FIELDS = [
    "probe_id",
    "selection_id",
    "node_id",
    "node_label",
    "source_url",
    "probe_method",
    "http_status",
    "final_url",
    "content_type",
    "bytes_sampled",
    "probe_result",
    "evidence_acceptance_status",
    "blocked_claims",
    "next_action",
]


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def probe_url(url: str) -> tuple[str, str, str, int, str]:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=TIMEOUT_SECONDS) as response:
            sample = response.read(SAMPLE_BYTES)
            return (
                str(response.status),
                response.geturl(),
                response.headers.get("content-type", "unknown"),
                len(sample),
                "reachable-sampled-not-accepted",
            )
    except urllib.error.HTTPError as exc:
        return (
            str(exc.code),
            exc.geturl(),
            exc.headers.get("content-type", "unknown"),
            0,
            "http-error-not-accepted",
        )
    except Exception as exc:  # noqa: BLE001 - preserve probe failure as evidence metadata.
        return "none", url, "unknown", 0, f"probe-error-not-accepted:{type(exc).__name__}"


def main() -> int:
    rows: list[dict[str, str]] = []
    for selected in read_csv(SELECTION):
        status, final_url, content_type, bytes_sampled, result = probe_url(selected["source_url"])
        rows.append(
            {
                "probe_id": selected["selection_id"].replace("SOURCE", "PROBE"),
                "selection_id": selected["selection_id"],
                "node_id": selected["node_id"],
                "node_label": selected["node_label"],
                "source_url": selected["source_url"],
                "probe_method": "http-get-sample",
                "http_status": status,
                "final_url": final_url,
                "content_type": content_type,
                "bytes_sampled": str(bytes_sampled),
                "probe_result": result,
                "evidence_acceptance_status": "not-accepted",
                "blocked_claims": selected["blocked_claims"],
                "next_action": "inspect terminal fields before node fixture replacement",
            }
        )

    with OUTPUT.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"wrote {OUTPUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
