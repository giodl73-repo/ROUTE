---
wave: t2-game-publication-evidence-blocker-relief
date_open: 2026-05-14
status: done
source: data/t2-game-publication-evidence-policy-acceptance.csv
---

# T2 Game Publication Evidence Blocker Relief

## Mission

Replay accepted T2 game publication evidence policy into explicit
blocker-relief rows before any optimizer-ledger replay or scenario publication.

## Opening Rule

This wave may reduce blockers inside the relief artifact because policy
acceptance already exists. It may not mutate the optimizer constraint ledger,
scenario hooks, game overlays, or map artifacts.

## Inputs Inherited

| Input | Source |
|---|---|
| T2 game publication evidence policy acceptance | `data/t2-game-publication-evidence-policy-acceptance.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Blocker-relief surface | done | `data/t2-game-publication-evidence-blocker-relief.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/relief/review.md`; final gates |

## Done Criteria

- Every accepted game publication evidence policy row has one blocker-relief
  row.
- Rows reduce blocker count from 1 to 0 per scenario with
  `claim_blocker_delta = -1`.
- Rows remain pending optimizer constraint-ledger replay.
- Optimizer and release manifests register the artifact.
- Final gates pass before close.

## Non-Goals

- Do not mutate `data/optimizer-constraint-ledger.csv` in this wave.
- Do not publish scenario hooks or final game overlays.
- Do not regenerate map PNGs.

