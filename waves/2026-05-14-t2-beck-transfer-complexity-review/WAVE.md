---
wave: t2-beck-transfer-complexity-review
date_open: 2026-05-14
status: done
source: data/beck-t2-diagnostics.csv
---

# T2 Beck Transfer Complexity Review

## Mission

Expand the residual T2 `beck_transfer_complexity` optimizer claim family into
route-level review rows before any transfer policy or blocker relief.

## Opening Rule

This wave may classify transfer-complexity blockers by route. It may not reduce
blockers, mutate Beck diagnostics, or alter selector behavior.

## Inputs Inherited

| Input | Source |
|---|---|
| Optimizer claim review | `data/optimizer-claim-review.csv` |
| Beck T2 diagnostics | `data/beck-t2-diagnostics.csv` |
| Residual backlog | `data/optimizer-residual-blocker-backlog.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Transfer-complexity review surface | done | `data/t2-beck-transfer-complexity-review.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/review/review.md`; final gates |

## Done Criteria

- Every T2 `beck_transfer_complexity` claim route has one review row.
- Rows preserve `map;promotion;publication` blockers with
  `claim_blocker_delta = 0`.
- Rows point to the next policy artifact instead of granting relief.
- Optimizer and release manifests register the artifact.
- Final gates pass before close.

## Non-Goals

- Do not change `data/beck-t2-diagnostics.csv`.
- Do not reduce transfer-complexity blockers.
- Do not publish final Beck replacement geometry.
