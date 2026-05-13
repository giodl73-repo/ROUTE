---
name: Constraint Ledger Blocker Burn-Down R1 Schematic Cartographer
slug: blocker-burndown-r1-schematic-cartographer
type: review
status: reviewed
rubric_version: v1.0
author: route-review
created: 2026-05-13
updated: 2026-05-13
sources:
  - .roles/parliament/schematic-cartographer.md
  - waves/2026-05-13-constraint-ledger-blocker-burndown/WAVE.md
---

# R1 - Schematic Cartographer

## Verdict

Pass after gate-name correction.

## Findings

| Severity | Artifact | Finding | Fix |
|---|---|---|---|
| WARN | `plans/pulse-05.md` | A generic `route beck --gate` is not the implemented gate surface. | Use `beck-t1-diagnostics`, `beck-t2-diagnostics`, `t1-beck-alignment`, and `map-atlas` gates. |
| NOTE | `plans/pulse-02.md` | T4 zone assignment should not become a national schematic shortcut. | Keep T4 rows in zone/local inset treatment unless a higher-tier contact witness exists. |

