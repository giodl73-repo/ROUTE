---
wave: optimizer-claim-review
date_open: 2026-05-14
status: done
source: data/optimizer-residual-blocker-backlog.csv
---

# Optimizer Claim Review

## Mission

Return from proof-chain bookkeeping to broad optimizer work by docketing the P1
residual claim-blocker families that are not terminal-access proof work and not
game bundle-binding work.

## Opening Rule

This wave may classify P1 claim blockers into a review docket and preserve
their owning artifacts. It may not reduce blockers, promote map/game/publication
claims, mutate selectors, or treat review rows as accepted proof.

## Inputs Inherited

| Input | Source |
|---|---|
| Residual blocker backlog | `data/optimizer-residual-blocker-backlog.csv` |
| Constraint budget | `data/optimizer-constraint-budget.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Claim-review surface | done | `data/optimizer-claim-review.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/claim-review/review.md`; final gates |

## Done Criteria

- Every `P1-claim-blocker` residual backlog row assigned to
  `optimizer-claim-review` has one review row.
- Rows preserve blocked claims and set `claim_blocker_delta = 0`.
- Rows point back to the evidence artifact that owns the original blocker.
- Optimizer and release manifests register the claim-review artifact.
- Final gates pass before close.

## Non-Goals

- Do not resolve P1 claim blockers in this wave.
- Do not fetch or attach new evidence.
- Do not change selector output or map/game publication status.
