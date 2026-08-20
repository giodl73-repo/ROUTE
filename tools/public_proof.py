#!/usr/bin/env python3
"""ROUTE public proof — no secrets required."""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def run(label: str, args: list[str]) -> None:
    print(f"\n==> {label}")
    print(" ".join(args))
    completed = subprocess.run(args, cwd=ROOT)
    if completed.returncode != 0:
        raise SystemExit(f"public proof failed: {label} (exit {completed.returncode})")


def main() -> int:
    print("ROUTE public proof (no Census key required)")
    print("Repo:", ROOT)
    run("I-80 flagship packet check", [sys.executable, "tools/build_i80_flagship_packet.py", "--check"])
    run(
        "I-80 no-credential source gate",
        [
            sys.executable,
            "tools/prepare_i80_report_sources.py",
            "--execute",
            "--gate-no-credential",
        ],
    )

    packet = ROOT / "docs" / "packets" / "i80-flagship-review-packet.md"
    text = packet.read_text(encoding="utf-8")
    if "Hold and narrow" not in text:
        raise SystemExit("public proof failed: packet missing Hold and narrow posture")

    print(
        """
==> Public proof PASS
Read next:
  - docs/packets/i80-flagship-review-packet.md
  - docs/how-to/public-proof.md
  - docs/map-publication-scope.md

Still held / not proven by this path:
  - design/ yield
  - ACS-backed full report regen (optional; needs CENSUS_API_KEY)
  - full T1-T4 publication claims
"""
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
