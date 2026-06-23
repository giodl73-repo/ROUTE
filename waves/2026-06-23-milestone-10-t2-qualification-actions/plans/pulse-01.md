---
wave: milestone-10-t2-qualification-actions
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Qualification Action Basis Gate

## Deliverables

- Add explicit `covered_bases` to each T2 qualification-action rule.
- Regenerate `data/beck-t2-qualification-actions.csv` with the covered-basis
  column.
- Strengthen `route beck-t2-qualification-actions --gate` so it verifies every
  T2 diagnostic row has both a covered `service_action` and covered
  `qualification_basis`.
- Add route-map test coverage for action/basis pair coverage.

## Gates

- `route beck-t2-qualification-actions --gate`
- `npm run check:l2`

## Non-goals

- Do not change T2 geometry, stop placement, service class assignment, or game
  behavior in this pulse.
- Do not resolve unrelated T1 design-review data changes.

## Result

Done. The T2 qualification-action ledger now carries basis coverage and the CLI
gate rejects uncovered duplicate-service decision bases.
