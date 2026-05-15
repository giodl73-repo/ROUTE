---
wave: residual-backlog-classifier-hygiene
date_open: 2026-05-14
status: done
source: data/optimizer-residual-blocker-backlog.csv
---

# Residual Backlog Classifier Hygiene

## Mission

Correct residual-backlog family classification after game/ops relief replay so
relieved pass rows do not keep a zero-claim game backlog alive.

## Opening Rule

This wave may change backlog classification and regenerate
`data/optimizer-residual-blocker-backlog.csv`. It must not change the optimizer
ledger, relieve asset debt, or reduce T4 terminal-access blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Optimizer constraint budget | `data/optimizer-constraint-budget.csv` |
| Residual blocker backlog | `data/optimizer-residual-blocker-backlog.csv` |
| T2 game/ops relief replay close | `waves/2026-05-14-t2-game-ops-bundle-evidence-ledger-replay/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Classifier fix | done | `optimizer_backlog_family`; regression test |
| 02 - Backlog regeneration | done | `data/optimizer-residual-blocker-backlog.csv` |
| 03 - Review and close | done | `CLOSE.md`; `panels/classifier/review.md`; final gates |

## Done Criteria

- Game/ops backlog priority requires live claim blockers, not merely a relief
  class name.
- Terminal-access and source-evidence priority also require live claim blockers.
- Asset debt remains visible when a relieved game/ops bundle still carries
  pavement debt.
- Residual backlog regenerates without the zero-claim game backlog row.
- Final gates pass before close.

## Non-Goals

- Do not reduce T4 terminal-access evidence blockers.
- Do not resolve asset-condition debt.
- Do not alter selector or map publication policy.

