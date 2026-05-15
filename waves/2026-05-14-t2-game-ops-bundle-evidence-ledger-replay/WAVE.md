---
wave: t2-game-ops-bundle-evidence-ledger-replay
date_open: 2026-05-14
status: done
source: data/t2-game-ops-bundle-evidence-blocker-relief.csv
---

# T2 Game/Ops Bundle Evidence Ledger Replay

## Mission

Replay accepted T2 game/ops bundle evidence relief into the optimizer
constraint ledger, budget, residual backlog, and selector outputs.

## Opening Rule

This wave may suppress only `game_ops_bundle_binding` blockers whose
`segment_bundle_id` appears in accepted relief rows. It must preserve unrelated
asset-condition debt, terminal-access evidence blockers, and source-acquisition
holds.

## Inputs Inherited

| Input | Source |
|---|---|
| T2 game/ops bundle evidence relief | `data/t2-game-ops-bundle-evidence-blocker-relief.csv` |
| Optimizer constraint ledger | `data/optimizer-constraint-ledger.csv` |
| Optimizer budget/backlog/selectors | `data/optimizer-constraint-budget.csv`; `data/optimizer-residual-blocker-backlog.csv`; `data/tier-optimizer-runs.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Ledger replay wiring | done | `optimizer_constraint_ledger_rows` consumes T2 game/ops relief rows; regression test |
| 02 - Artifact regeneration | done | ledger, budget, backlog, and tier optimizer outputs regenerated |
| 03 - Review and close | done | `CLOSE.md`; `panels/replay/review.md`; final gates |

## Done Criteria

- Accepted T2 game/ops bundle relief rows emit `game_ops_bundle_binding_relief`
  pass rows in the optimizer ledger.
- Matching `game_ops_bundle_binding` blockers are suppressed by
  `segment_bundle_id`.
- Budget grouping remains unique when a bundle also has asset-condition debt.
- Residual backlog and tier optimizer outputs are regenerated from the replayed
  ledger.
- Final gates pass before close.

## Non-Goals

- Do not relieve asset-condition debt carried by I-110 or any other bundle.
- Do not resolve T4 terminal-access evidence gaps.
- Do not refresh publication maps until selector gates have a full pass-ready
  selection.

