---
wave: t2-game-publication-evidence-policy
date_open: 2026-05-14
status: done
source: data/t2-game-publication-evidence-review.csv
---

# T2 Game Publication Evidence Policy

## Mission

Author evidence policy rows for the reviewed T2 game publication readiness
scenarios before any policy acceptance, blocker relief, or optimizer-ledger
replay.

## Opening Rule

This wave may author required evidence treatments for scenario publication. It
may not accept the policy, reduce blockers, mutate scenario hooks, or change
optimizer ledger output.

## Inputs Inherited

| Input | Source |
|---|---|
| T2 game publication evidence review | `data/t2-game-publication-evidence-review.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Evidence policy surface | done | `data/t2-game-publication-evidence-policy.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/policy/review.md`; final gates |

## Done Criteria

- Every game publication evidence review row has one policy row.
- Policy rows preserve `game;publication;upgrade` blockers with
  `claim_blocker_delta = 0`.
- Rows route to policy acceptance before any blocker relief.
- Optimizer and release manifests register the artifact.
- Final gates pass before close.

## Non-Goals

- Do not accept game publication evidence policy in this wave.
- Do not reduce T2 game publication readiness blockers.
- Do not publish scenario hooks or final game overlays.

