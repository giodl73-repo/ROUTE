---
wave: t1-schematic-geometry-claim-review
date_open: 2026-05-14
status: done
source: data/optimizer-claim-review.csv
---

# T1 Schematic Geometry Claim Review

## Mission

Burn down the largest optimizer claim-review family by expanding the T1
schematic-geometry aggregate blocker into route-level shared-segment map-policy
review rows.

## Opening Rule

This wave may bind T1 schematic geometry claim blockers to their owning design
policy action. It may not accept shared-segment geometry, publish final Beck
replacement geometry, mutate T1 selectors, or reduce blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Optimizer claim review | `data/optimizer-claim-review.csv` |
| T1 design review | `data/t1-design-review.csv` |
| T1 design policy actions | `data/t1-design-policy-actions.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Schematic claim surface | done | `data/t1-schematic-geometry-claim-review.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/claim-review/review.md`; final gates |

## Done Criteria

- The T1 schematic-geometry claim-review row expands to route-level rows for
  I40, I80, I90, and I95.
- Rows preserve `map;publication`, `blocker_count_after = 2`, and
  `claim_blocker_delta = 0`.
- Rows bind to `resolve-shared-segment-map-policy`.
- Optimizer and release manifests register the artifact.
- Final gates pass before close.

## Non-Goals

- Do not resolve shared-segment map policy.
- Do not replace final Beck geometry.
- Do not reduce T1 schematic blockers.
