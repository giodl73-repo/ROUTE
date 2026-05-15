---
wave: t2-game-ops-bundle-evidence-blocker-relief
date_open: 2026-05-14
status: done
source: data/t2-game-ops-bundle-evidence-policy-acceptance.csv
---

# T2 Game/Ops Bundle Evidence Blocker Relief

## Mission

Emit blocker relief rows from accepted T2 game/ops bundle evidence policy before
optimizer-ledger replay.

## Opening Rule

This wave may create relief rows with negative blocker deltas. It must not wire
those deltas into `data/optimizer-constraint-ledger.csv`, regenerate residual
backlog counts from relief, or refresh maps.

## Inputs Inherited

| Input | Source |
|---|---|
| T2 game/ops bundle evidence policy acceptance | `data/t2-game-ops-bundle-evidence-policy-acceptance.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and relief scope | done | this wave card |
| 02 - Blocker relief surface | done | `data/t2-game-ops-bundle-evidence-blocker-relief.csv`; CLI regression test |
| 03 - Review and close | done | `CLOSE.md`; `panels/relief/review.md`; final gates |

## Done Criteria

- Every accepted T2 game/ops bundle evidence policy row has one relief row.
- Relief rows reduce accepted blockers to zero locally and mark ledger replay as
  pending.
- Optimizer and release manifests register the relief artifact.
- Final gates pass before close.

## Non-Goals

- Do not replay relief into the optimizer ledger in this wave.
- Do not regenerate residual backlog counts from the relief artifact.
- Do not refresh final maps or selector publication images.

