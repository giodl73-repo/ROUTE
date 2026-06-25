---
wave: milestone-10-t2-parallel-budget-effects-compatibility
pulse: 01
date: 2026-06-25
status: done
---

# Pulse 01 - Parallel Budget Preserves Ledger Effects

## Deliverables

- Add optimizer budget coverage for qualification-bearing parallel-service
  ledger rows.
- Confirm pipe-delimited qualification effects survive into route-scoped budget
  rows.
- Preserve budget rollup counts, blocker claims, and validation status behavior.

## Gates

- `cargo test -q -p route --bin route optimizer_constraint_budget_extracts_parallel_qualification_effects`
- `cargo test -q -p route --bin route optimizer_constraint_ledger_preserves_parallel_service_qualification_effects`
- `cargo test -q -p route --bin route t2_regionalizer`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/optimizer-constraint-budget.csv`; current data does not
  need a semantic row update for this compatibility assertion.
- Do not change optimizer budget rollup scoring.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Optimizer budget coverage now locks in preservation of close-parallel
service qualification effects.
