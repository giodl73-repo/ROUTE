---
wave: milestone-10-t2-budget-qualification-semantics
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Budget Rollup Preserves Qualification Semantics

## Deliverables

- Add `qualification_effects` to `OptimizerConstraintBudgetRow`.
- Roll up qualification-bearing optimizer effects from
  `data/optimizer-constraint-ledger.csv`.
- Strengthen `route optimizer-constraint-budget --gate` so qualification-bearing
  ledger rows cannot be dropped in budget rollup.
- Regenerate `data/optimizer-constraint-budget.csv`.

## Gates

- `route optimizer-constraint-budget --gate`
- `npm run check:l2`

## Non-goals

- Do not change budget classification, selector scoring, or downstream routing.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Optimizer constraint budget now preserves qualification semantics from
ledger replay.
