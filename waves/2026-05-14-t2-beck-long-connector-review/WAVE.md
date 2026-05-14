---
wave: t2-beck-long-connector-review
date_open: 2026-05-14
status: done
source: data/optimizer-residual-blocker-backlog.csv
---

# T2 Beck Long Connector Review

## Mission

Expand the residual T2 `beck_long_connector` claim-blocker family into
route-level review rows before any connector policy, acceptance, relief, or
ledger replay.

## Opening Rule

This wave may classify long-connector T2 Beck rows and name the next connector
policy artifact, but it must not reduce blockers or promote map, promotion, or
publication claims.

## Inputs Inherited

| Input | Source |
|---|---|
| Residual blocker backlog | `data/optimizer-residual-blocker-backlog.csv` |
| Optimizer claim-review docket | `data/optimizer-claim-review.csv` |
| T2 Beck diagnostics | `data/beck-t2-diagnostics.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and connector scope | done | this wave card and pulse plan |
| 02 - Long-connector review artifact | done | `data/t2-beck-long-connector-review.csv`; CLI regression test |
| 03 - Doctrine close | done | `CLOSE.md`; `panels/review/review.md`; manifest and index updates |

## Done Criteria

- The three backlog representative routes are represented exactly once.
- Review rows preserve all three long-connector claim blockers.
- Every row points to the long-connector policy artifact needed before relief.
- Final gates pass before close.

## Non-Goals

- Do not author the long-connector policy in this wave.
- Do not change Beck geometry, connector routing, or optimizer constraint ledger.
- Do not reduce residual claim blockers.
