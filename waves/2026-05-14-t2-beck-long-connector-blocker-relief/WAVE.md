---
wave: t2-beck-long-connector-blocker-relief
date_open: 2026-05-14
status: done
source: data/t2-beck-long-connector-policy-acceptance.csv
---

# T2 Beck Long Connector Blocker Relief

## Mission

Replay accepted T2 Beck long-connector policy into explicit blocker-relief rows
before any optimizer-ledger replay or final Beck replacement publication.

## Opening Rule

This wave may reduce blockers inside the relief artifact because policy
acceptance already exists. It may not mutate the optimizer constraint ledger,
Beck diagnostics, or selector output.

## Inputs Inherited

| Input | Source |
|---|---|
| T2 Beck long-connector policy acceptance | `data/t2-beck-long-connector-policy-acceptance.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Blocker-relief surface | done | `data/t2-beck-long-connector-blocker-relief.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/relief/review.md`; final gates |

## Done Criteria

- Every accepted long-connector policy row has one blocker-relief row.
- Rows reduce blocker count from 1 to 0 per route with
  `claim_blocker_delta = -1`.
- Rows remain pending optimizer constraint-ledger replay.
- Optimizer and release manifests register the artifact.
- Final gates pass before close.

## Non-Goals

- Do not mutate `data/optimizer-constraint-ledger.csv` in this wave.
- Do not change `data/beck-t2-diagnostics.csv`.
- Do not publish final Beck replacement geometry.

