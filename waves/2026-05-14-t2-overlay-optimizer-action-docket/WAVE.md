---
wave: t2-overlay-optimizer-action-docket
date_open: 2026-05-14
status: closed
source: waves/2026-05-14-t2-stitched-member-proof-review-docket/CLOSE.md
---

# T2 Overlay Optimizer Action Docket

## Mission

Route the 15 residual T2 bundle-overlay repair deltas back into concrete
optimizer action families after the stitched-member source chain returned to
held-known status.

## Opening Rule

This wave may classify optimizer action families and next artifacts. It may not
promote game/incident/publication/upgrade claims, bind service overlays, or edit
registry/bundle membership.

## Inputs Inherited

| Input | Source |
|---|---|
| Overlay repair delta | `data/t2-bundle-overlay-repair-delta.csv` |
| Proof review return | `data/t2-stitched-member-proof-review-docket.csv` |
| Optimizer manifest | `data/tier-optimizer-runs.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Optimizer action surface | done | `data/t2-overlay-optimizer-action-docket.csv` and CLI gate |
| 03 - Review and close | done | manifests, review, gates, closeout |

## Done Criteria

- Every T2 overlay repair delta row has an optimizer action row.
- Rows classify action family without reducing blockers.
- Rows preserve `game;incident;publication;upgrade` and remain review-only.
- Optimizer and release manifests register the action docket.
- Final gates pass before close.

## Non-Goals

- Do not accept source evidence.
- Do not bind game/ops overlays.
- Do not classify candidates as repaired.
- Do not edit registry or bundle membership.
