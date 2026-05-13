---
wave: instrument-calibration
date_open: 2026-05-08
date_close: 2026-05-09
status: done
source: git-history
---

# Instrument Calibration

## Mission

Make the scorer reliable enough for Atlas and downstream claims: all dimensions
live or labeled, v1.4 rubric alignment, score artifacts regenerated, confidence
risks exposed, and Instrument closeout recorded.

## Commit-Derived Pulses

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Live dimension joins and v1.4 rubric | done | `d46029f` through `b3326fe` |
| 02 - Map/scorer truth-label hardening | done | `b0ee411`, `13154ee`, `6cc218e`, `dcae406`, `791ae76`, `4ef1895`, `7cfe998` |
| 03 - Atlas scoring and proxy/source hardening | done | `1276ca1` through `1c93f62` |
| 04 - Confidence ledgers and Instrument closeout | done | `5d47aa9` through `7e6539c` |

## Close Evidence

Instrument closes with v1.4 scoring, confidence labels, risk ledgers,
dimension-level summaries, optional FPM reliability joins, and tests that make
score artifacts harder to drift.
