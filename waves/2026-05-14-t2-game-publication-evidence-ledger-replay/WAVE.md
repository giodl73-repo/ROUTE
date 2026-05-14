---
wave: t2-game-publication-evidence-ledger-replay
date_open: 2026-05-14
status: done
source: data/t2-game-publication-evidence-blocker-relief.csv
---

# T2 Game Publication Evidence Ledger Replay

## Mission

Wire accepted T2 game publication evidence blocker relief into the optimizer
constraint ledger so downstream budget, residual backlog, and selector outputs
reflect the relief.

## Opening Rule

Only accepted game publication evidence relief scenarios may suppress existing
T2 game publication readiness blocker rows. Other T2 game/ops bundle, T4
terminal evidence, source, and debt families remain governed by their source
artifacts.

## Inputs Inherited

| Input | Source |
|---|---|
| T2 game publication evidence blocker relief | `data/t2-game-publication-evidence-blocker-relief.csv` |
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

- Accepted game publication evidence relief scenarios no longer emit
  `game_ops_publication_readiness` claim-blocker rows in the optimizer
  constraint ledger.
- Relief scenarios emit pass rows with lineage to
  `data/t2-game-publication-evidence-blocker-relief.csv`.
- Budget, residual backlog, and optimizer run manifest are regenerated from the
  replayed ledger.
- Final gates pass before close.

## Non-Goals

- Do not alter unresolved T2 game/ops bundle, T4 terminal, source, or budget
  blocker families.
- Do not publish scenario hooks or final game overlays.
- Do not regenerate map PNGs.

