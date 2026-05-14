---
wave: t3-lower-tier-feeder-gap-ledger-replay
date_open: 2026-05-14
status: done
source: data/t3-lower-tier-feeder-gap-blocker-relief.csv
---

# T3 Lower-Tier Feeder Gap Ledger Replay

## Mission

Wire accepted T3 lower-tier feeder-gap blocker relief into the optimizer
constraint ledger so downstream budget and residual backlog counts reflect the
relief.

## Opening Rule

Only accepted T3 feeder relief rows may suppress existing
`lower_tier_feeder_gap` blocker rows. Terminal-access evidence, game, source,
T2, asset, and debt families remain governed by their source artifacts.

## Inputs Inherited

| Input | Source |
|---|---|
| T3 lower-tier feeder-gap blocker relief | `data/t3-lower-tier-feeder-gap-blocker-relief.csv` |
| Prior optimizer constraint ledger | `data/optimizer-constraint-ledger.csv` |
| Prior optimizer constraint budget | `data/optimizer-constraint-budget.csv` |
| Prior optimizer residual blocker backlog | `data/optimizer-residual-blocker-backlog.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and replay scope | done | this wave card and pulse plan |
| 02 - Constraint-ledger replay | done | `data/optimizer-constraint-ledger.csv`; CLI regression test |
| 03 - Budget/backlog close | done | `data/optimizer-constraint-budget.csv`; `data/optimizer-residual-blocker-backlog.csv`; `CLOSE.md`; `panels/replay/review.md` |

## Done Criteria

- Accepted T3 feeder relief routes no longer emit raw `lower_tier_feeder_gap`
  claim-blocker rows in the optimizer constraint ledger.
- Relief routes emit pass rows with lineage to
  `data/t3-lower-tier-feeder-gap-blocker-relief.csv`.
- Budget and residual backlog are regenerated from the replayed ledger.
- Final gates pass before close.

## Non-Goals

- Do not alter unresolved T2 label-density, T2 long-connector, game, source,
  T4 terminal-access, asset, or budget debt families.
- Do not change feeder policy or relief artifacts after replay.
