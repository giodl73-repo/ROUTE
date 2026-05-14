---
wave: t1-shared-segment-policy-acceptance
date_open: 2026-05-14
status: done
source: data/t1-shared-segment-map-policy.csv
---

# T1 Shared Segment Policy Acceptance

## Mission

Accept authored T1 shared-segment map policy as ready for blocker-relief replay
without directly reducing blockers.

## Opening Rule

This wave may accept the authored interline-or-transfer-split policy for the
two shared T1 route pairs. It may not reduce blockers, publish final Beck
replacement geometry, or mutate selector output.

## Inputs Inherited

| Input | Source |
|---|---|
| T1 shared-segment map policy | `data/t1-shared-segment-map-policy.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Acceptance surface | done | `data/t1-shared-segment-policy-acceptance.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/acceptance/review.md`; final gates |

## Done Criteria

- Every held shared-segment map-policy row has one acceptance row.
- Rows set `accepted-policy-ready-for-relief-replay`.
- Rows preserve `map;publication`, blocker counts, and `claim_blocker_delta = 0`.
- Optimizer and release manifests register the artifact.
- Final gates pass before close.

## Non-Goals

- Do not reduce T1 schematic blockers.
- Do not publish final Beck replacement geometry.
- Do not change selector output.
