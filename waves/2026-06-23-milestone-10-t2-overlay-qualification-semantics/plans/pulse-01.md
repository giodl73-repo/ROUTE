---
wave: milestone-10-t2-overlay-qualification-semantics
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Bundle Overlays Consume Qualification Semantics

## Deliverables

- Add qualification map treatment, gate policy, and game-use fields to
  `T2BundleOverlayRow`.
- Populate those fields from `data/t2-service-selection.csv`.
- Strengthen `route t2-bundle-overlays --gate` so bound overlays must carry
  qualification semantics.
- Regenerate `data/game/t2-bundle-overlays.csv`.

## Gates

- `route t2-bundle-overlays --gate`
- `npm run check:l2`

## Non-goals

- Do not change service-selection decisions, Beck geometry, stop placement, or
  game scenario behavior.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. T2 bundle overlays now carry the qualification-action contract into the
game/ops binding surface.
