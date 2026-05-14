---
wave: t3-lower-tier-feeder-gap-review
date_open: 2026-05-14
status: done
source: data/optimizer-residual-blocker-backlog.csv
---

# T3 Lower-Tier Feeder Gap Review

## Mission

Expand the residual T3 `lower_tier_feeder_gap` claim-blocker family into
route-level review rows before any feeder policy, evidence acceptance, or
blocker relief.

## Opening Rule

This wave may classify the six T3 lower-tier feeder-gap blockers and name the
next policy artifact, but it must not reduce blockers or promote map,
publication, or upgrade claims.

## Inputs Inherited

| Input | Source |
|---|---|
| Residual blocker backlog | `data/optimizer-residual-blocker-backlog.csv` |
| T3/T4 access gaps | `data/t3-t4-access-gaps.csv` |
| Optimizer claim-review docket | `data/optimizer-claim-review.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and feeder scope | done | this wave card and pulse plan |
| 02 - Feeder review artifact | done | `data/t3-lower-tier-feeder-gap-review.csv`; CLI regression test |
| 03 - Doctrine close | done | `CLOSE.md`; `panels/review/review.md`; manifest and index updates |

## Done Criteria

- The six backlog representative routes are represented exactly once.
- Review rows preserve all six lower-tier feeder claim blockers.
- Every row points to the feeder policy artifact needed before any relief.
- Final gates pass before close.

## Non-Goals

- Do not author the feeder policy in this wave.
- Do not alter T3/T4 access-gap rows, terminal-access evidence rows, or the
  optimizer constraint ledger.
- Do not reduce residual claim blockers.
