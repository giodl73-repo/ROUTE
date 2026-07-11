from __future__ import annotations

import argparse
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "docs" / "packets" / "i80-flagship-review-packet.md"


def read(relative_path: str) -> str:
    path = ROOT / relative_path
    if not path.exists():
        raise SystemExit(f"missing packet source: {relative_path}")
    return path.read_text(encoding="utf-8")


def section(text: str, heading: str) -> str:
    pattern = re.compile(
        rf"^## {re.escape(heading)}\s*$\n(.*?)(?=^## |\Z)",
        re.MULTILINE | re.DOTALL,
    )
    match = pattern.search(text)
    if not match:
        raise SystemExit(f"missing section: {heading}")
    return match.group(1).strip()


def line_matching(text: str, prefix: str) -> str:
    for line in text.splitlines():
        if line.startswith(prefix):
            return line
    raise SystemExit(f"missing line prefix: {prefix}")


def build_packet() -> str:
    corpus = read("corpus/existing/i80.md")
    gap = read("gaps/i80-flagship.md")
    review = read(
        "waves/2026-07-11-i80-flagship-stabilization/"
        "panels/i80-treatment-review/R1-consolidated.md"
    )
    validation = read(
        "docs/plans/i80-des-moines-transfer-resilience-validation.md"
    )
    docket_path = "docs/reviews/i80-external-review-docket.md"
    docket = read(docket_path)

    if "*[Human annotation" in corpus:
        raise SystemExit("I-80 corpus still contains annotation placeholders")

    packet = f"""---
name: I-80 Flagship Review Packet
slug: i80-flagship-review-packet
type: report
status: reviewed
rubric_version: v1.4
author: route-packet-builder
created: 2026-07-11
updated: 2026-07-11
sources:
  - corpus/existing/i80.md
  - gaps/i80-flagship.md
  - waves/2026-07-11-i80-flagship-stabilization/panels/i80-treatment-review/R1-consolidated.md
  - docs/plans/i80-des-moines-transfer-resilience-validation.md
  - {docket_path}
---

# I-80 Flagship Review Packet

## Review Posture

**Hold and narrow.**

ROUTE has a reviewed I-80 corridor record and a bounded Des Moines topology
hypothesis. It does not have an approved design, capital recommendation,
positive ROI, guaranteed SLA, agency endorsement, or publication-ready
intervention benefit.

## Ten-Minute Review

| Time | Topic | Decision focus |
|---:|---|---|
| 1 minute | Mission and claim boundary | Is the question narrow enough? |
| 2 minutes | I-80 corridor baseline | Which measurements are usable or held? |
| 2 minutes | Des Moines gap diagnosis | Is the topology hypothesis physically credible? |
| 2 minutes | Parliament decision | Were the right blockers applied? |
| 2 minutes | Validation plan | Are the promotion gates sufficient? |
| 1 minute | External review ask | Advance validation, narrow it, or reject it? |

## Corridor Baseline

{section(corpus, "Overview")}

{section(corpus, "Key Facts")}

{line_matching(corpus, "**Band totals**")}

{line_matching(corpus, "**Confidence**")}

### Score boundary

{section(corpus, "Flagship Claim Holds")}

## Current Gap Diagnosis

{section(gap, "Diagnosed Flagship Gap")}

### Current command evidence

{section(gap, "Current Command Evidence")}

## Parliament And Editorial Decision

{section(review, "Decision")}

### Binding findings

{section(review, "Binding Findings")}

## Validation Gates

{section(validation, "Promotion Gates")}

## Explicit Holds

{section(validation, "Explicit Holds")}

## External Review Ask

Reviewers receive this packet plus `{docket_path}`. They are not asked to endorse
construction. They are asked whether the validation plan should:

1. advance as written;
2. narrow to a smaller topology, demand, or community question;
3. add a missing evidence gate; or
4. be rejected because Des Moines is the wrong I-80 flagship hypothesis.

### Decision matrix

{section(docket, "Review Decision Matrix")}

### Current review results

{section(docket, "Review Results")}

## Regeneration

```powershell
npm run build:i80:packet
npm run check:i80:packet
```
"""
    return packet.replace("\r\n", "\n").rstrip() + "\n"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    generated = build_packet()
    if args.check:
        if not OUTPUT.exists():
            raise SystemExit(f"missing generated packet: {OUTPUT.relative_to(ROOT)}")
        existing = OUTPUT.read_text(encoding="utf-8").replace("\r\n", "\n")
        if existing != generated:
            raise SystemExit(
                "I-80 flagship packet is stale; run `npm run build:i80:packet`"
            )
        print("I-80 flagship packet is current")
        return

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(generated, encoding="utf-8", newline="\n")
    print(f"wrote {OUTPUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
