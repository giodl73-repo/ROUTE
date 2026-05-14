---
wave: t1-schematic-geometry-ledger-replay
date_open: 2026-05-14
status: done
source: data/t1-schematic-geometry-blocker-relief.csv
---

# T1 Schematic Geometry Ledger Replay

## Mission

Wire accepted T1 schematic geometry blocker relief into the optimizer constraint
ledger so downstream budget and residual backlog counts reflect the relief.

## Opening Rule

Only the accepted relief pairs may suppress existing T1 schematic blocker rows.
All other optimizer blocker families remain governed by their source artifacts.

## Inputs Inherited

| Input | Source |
|---|---|
| T1 schematic geometry blocker relief | `data/t1-schematic-geometry-blocker-relief.csv` |
| Prior optimizer constraint ledger | `data/optimizer-constraint-ledger.csv` |
| Prior optimizer constraint budget | `data/optimizer-constraint-budget.csv` |
| Prior optimizer residual blocker backlog | `data/optimizer-residual-blocker-backlog.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and replay scope | done | this wave card and pulse plans |
| 02 - Constraint-ledger replay | done | `data/optimizer-constraint-ledger.csv`; CLI regression test |
| 03 - Budget/backlog close | done | `data/optimizer-constraint-budget.csv`; `data/optimizer-residual-blocker-backlog.csv`; `CLOSE.md`; `panels/replay/review.md` |

## Done Criteria

- Accepted relief routes no longer emit `schematic_geometry` or
  `beck_schematic_geometry` claim-blocker rows in the optimizer constraint
  ledger.
- Relief pairs emit pass rows with lineage to
  `data/t1-schematic-geometry-blocker-relief.csv`.
- Budget and residual backlog are regenerated from the replayed ledger.
- Final gates pass before close.

## Non-Goals

- Do not alter unresolved T2, T3, T4, source, game, or budget blocker families.
- Do not publish final Beck replacement geometry.
- Do not change tier selector rules beyond regenerated outputs.
