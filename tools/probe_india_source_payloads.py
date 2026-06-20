#!/usr/bin/env python3
"""Probe India source-payload URLs without accepting evidence."""

from __future__ import annotations

import csv
import sys
import urllib.error
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ACCESS = ROOT / "data" / "international-india-source-payload-access-001.csv"
OUTPUT = ROOT / "data" / "international-india-source-payload-probe-001.csv"
USER_AGENT = "ROUTE-India-payload-probe/0.1 evidence-not-accepted"
TIMEOUT_SECONDS = 20
SAMPLE_BYTES = 65536

FIELDS = [
    "probe_id",
    "source_id",
    "payload_url_or_status",
    "probe_url",
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
        return str(exc.code), exc.geturl(), exc.headers.get("content-type", "unknown"), 0, "http-error-not-accepted"
    except Exception as exc:  # noqa: BLE001 - probes record failures without failing generation.
        return "none", url, "unknown", 0, f"probe-error-not-accepted:{type(exc).__name__}"


def main() -> int:
    rows: list[dict[str, str]] = []
    for access in read_csv(ACCESS):
        source_id = access["source_id"]
        target = access["payload_url_or_status"]
        if target.startswith("http"):
            status, final_url, content_type, bytes_sampled, result = probe_url(target)
            method = "http-get-sample"
        else:
            status = "not-applicable"
            final_url = target
            content_type = "not-applicable"
            bytes_sampled = 0
            result = access["payload_status"]
            method = "not-fetchable"
        rows.append(
            {
                "probe_id": f"IND-PROBE-{source_id.replace('IND-SRC-', '').replace('-', '')}",
                "source_id": source_id,
                "payload_url_or_status": access["payload_url_or_status"],
                "probe_url": target,
                "probe_method": method,
                "http_status": status,
                "final_url": final_url,
                "content_type": content_type,
                "bytes_sampled": str(bytes_sampled),
                "probe_result": result,
                "evidence_acceptance_status": "not-accepted",
                "blocked_claims": access["blocked_claims"],
                "next_action": access["next_action"],
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
