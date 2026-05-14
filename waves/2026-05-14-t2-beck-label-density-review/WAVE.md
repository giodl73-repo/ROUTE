---
wave: t2-beck-label-density-review
date_open: 2026-05-14
status: done
source: data/optimizer-residual-blocker-backlog.csv
---

# T2 Beck Label Density Review

## Mission

Expand the residual T2 `beck_label_density` claim-blocker family into
route-level review rows before any label policy, acceptance, relief, or ledger
replay.

## Opening Rule

This wave may classify dense T2 Beck rows and name the next label policy
artifact, but it must not reduce blockers or promote map, promotion, or
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
| 01 - Wave card and density scope | done | this wave card and pulse plan |
| 02 - Label-density review artifact | done | `data/t2-beck-label-density-review.csv`; CLI regression test |
| 03 - Doctrine close | done | `CLOSE.md`; `panels/review/review.md`; manifest and index updates |

## Done Criteria

- The five backlog representative routes are represented exactly once.
- Review rows preserve all five label-density claim blockers.
- Every row points to the label-density policy artifact needed before relief.
- Final gates pass before close.

## Non-Goals

- Do not author the label-density policy in this wave.
- Do not change Beck geometry, label placement, or optimizer constraint ledger.
- Do not reduce residual claim blockers.
