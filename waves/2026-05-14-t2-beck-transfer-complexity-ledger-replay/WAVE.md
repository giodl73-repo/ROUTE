---
wave: t2-beck-transfer-complexity-ledger-replay
date_open: 2026-05-14
status: done
source: data/t2-beck-transfer-complexity-blocker-relief.csv
---

# T2 Beck Transfer Complexity Ledger Replay

## Mission

Wire accepted T2 Beck transfer-complexity blocker relief into the optimizer
constraint ledger so downstream budget and residual backlog counts reflect the
relief.

## Opening Rule

Only accepted transfer-complexity relief routes may suppress existing T2 Beck
transfer-complexity blocker rows. Other Beck T2, T3, T4, game, source, and debt
families remain governed by their source artifacts.

## Inputs Inherited

| Input | Source |
|---|---|
| T2 Beck transfer-complexity blocker relief | `data/t2-beck-transfer-complexity-blocker-relief.csv` |
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

- Accepted transfer-complexity relief routes no longer emit
  `beck_transfer_complexity` claim-blocker rows in the optimizer constraint
  ledger.
- Relief routes emit pass rows with lineage to
  `data/t2-beck-transfer-complexity-blocker-relief.csv`.
- Budget and residual backlog are regenerated from the replayed ledger.
- Final gates pass before close.

## Non-Goals

- Do not alter unresolved T2 label-density, T2 long-connector, T3, T4, source,
  game, or budget blocker families.
- Do not publish final Beck replacement geometry.
- Do not change relief artifacts after replay.
