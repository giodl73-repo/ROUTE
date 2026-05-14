---
wave: t1-shared-segment-map-policy
date_open: 2026-05-14
status: done
source: data/t1-schematic-geometry-claim-review.csv
---

# T1 Shared Segment Map Policy

## Mission

Author the pair-level shared-segment map-policy artifact required before T1
schematic geometry blocker relief.

## Opening Rule

This wave may write conservative map-policy rows for shared T1 promise-spine
segments. It may not accept the policy, reduce blockers, publish final Beck
replacement geometry, or mutate selector output.

## Inputs Inherited

| Input | Source |
|---|---|
| T1 schematic geometry claim review | `data/t1-schematic-geometry-claim-review.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Map-policy surface | done | `data/t1-shared-segment-map-policy.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/map-policy/review.md`; final gates |

## Done Criteria

- Shared routes are collapsed into pair-level policy rows for I40/I95 and
  I80/I90.
- Rows preserve `map;publication`, `blocker_count_after = 4`, and
  `claim_blocker_delta = 0` per pair.
- Rows remain `held-pending-policy-acceptance`.
- Optimizer and release manifests register the artifact.
- Final gates pass before close.

## Non-Goals

- Do not accept shared-segment policy.
- Do not reduce T1 schematic blockers.
- Do not publish final Beck replacement geometry.
