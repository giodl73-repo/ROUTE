---
wave: t2-overlay-p3-local-zone-overlay-review
date_open: 2026-05-14
status: done
source: waves/2026-05-14-t2-overlay-p2-service-overlay-review/CLOSE.md
---

# T2 Overlay P3 Local Zone Overlay Review

## Mission

Decide the seven P3 local-zone overlay optimizer actions before returning to
broader optimizer work.

## Opening Rule

This wave may classify P3 local-zone overlay decisions and next artifacts. It
may not bind national game/ops overlays, reduce blockers, or mutate registry or
bundle membership.

## Inputs Inherited

| Input | Source |
|---|---|
| Optimizer action docket | `data/t2-overlay-optimizer-action-docket.csv` |
| P2 service-overlay review | `data/t2-overlay-p2-service-overlay-review.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - P3 local-zone surface | done | `data/t2-overlay-p3-local-zone-overlay-review.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/p3-local-zone-overlay/review.md`; final gates |

## Done Criteria

- Every `P3-local-zone-overlay` action row has a review row.
- Rows decide whether the local-zone action can advance or remains held.
- Rows preserve `game;incident;publication;upgrade` and remain review-only.
- Optimizer and release manifests register the P3 review artifact.
- Final gates pass before close.

## Non-Goals

- Do not bind national game/ops overlays.
- Do not edit registry or bundle membership.
- Do not start a new non-overlay optimizer family inside this wave.
