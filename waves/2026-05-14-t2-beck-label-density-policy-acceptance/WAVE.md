---
wave: t2-beck-label-density-policy-acceptance
date_open: 2026-05-14
status: done
source: data/t2-beck-label-density-policy.csv
---

# T2 Beck Label Density Policy Acceptance

## Mission

Accept authored T2 Beck label-density policy rows before any blocker relief or
final Beck replacement publication.

## Opening Rule

This wave may accept the authored label-density policy. It may not reduce
blockers, mutate Beck diagnostics, replay the optimizer ledger, or change
selector output.

## Inputs Inherited

| Input | Source |
|---|---|
| T2 Beck label-density policy | `data/t2-beck-label-density-policy.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Acceptance surface | done | `data/t2-beck-label-density-policy-acceptance.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/acceptance/review.md`; final gates |

## Done Criteria

- Every authored label-density policy row has one acceptance row.
- Acceptance rows preserve `map;promotion;publication` blockers with
  `claim_blocker_delta = 0`.
- Rows route to blocker relief before any optimizer-ledger replay.
- Optimizer and release manifests register the artifact.
- Final gates pass before close.

## Non-Goals

- Do not reduce T2 Beck label-density blockers.
- Do not replay the optimizer constraint ledger.
- Do not publish final Beck replacement geometry.
