---
wave: milestone-10-t2-qualification-actions
pulse: 02
date: 2026-06-23
status: done
---

# Pulse 02 - Service Selection Consumes Qualification Actions

## Deliverables

- Join T2 Beck diagnostic action/basis pairs to the qualification-action rule
  ledger while generating `data/t2-service-selection.csv`.
- Add service-selection columns for qualification map treatment, gate policy, and
  game use so downstream selection consumers do not need to reinterpret raw
  diagnostic strings.
- Strengthen `route t2-service-selection --gate` so every diagnostic-backed row
  must have a covered qualification action/basis pair and populated rule
  treatment fields.
- Regenerate `data/t2-service-selection.csv`.

## Gates

- `route t2-service-selection --gate`
- `npm run check:l2`

## Non-goals

- Do not change the T2 service-selection decision policy.
- Do not change Beck geometry, stop placement, service classes, or game overlay
  behavior.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. T2 service selection now consumes the qualification-action contract and
exports the map/gate/game-use semantics needed by downstream T2 decision
surfaces.
