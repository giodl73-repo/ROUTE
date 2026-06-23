---
wave: milestone-10-t2-binding-qualification-semantics
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Binding Decisions Preserve Qualification Semantics

## Deliverables

- Add qualification gate policy and game-use fields to
  `T2GameOpsBindingDecisionRow`.
- Populate those fields from `data/game/t2-bundle-overlays.csv`.
- Strengthen `route t2-game-ops-binding-decisions --gate` so bound and
  bundle-bound-review decisions must preserve qualification semantics.
- Regenerate `data/t2-game-ops-binding-decisions.csv`.

## Gates

- `route t2-game-ops-binding-decisions --gate`
- `npm run check:l2`

## Non-goals

- Do not change game/ops binding decision policy or repair routing.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. T2 game/ops binding decisions now preserve qualification-action semantics
from bundle overlays.
