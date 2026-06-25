---
wave: milestone-10-t2-ledger-budget-effects-compatibility
pulse: 01
date: 2026-06-24
status: done
---

# Pulse 01 - Budget Rollup Extracts Qualification Effects

## Deliverables

- Extract qualification effect clauses from optimizer ledger `optimizer_effect`
  text instead of storing full semicolon-delimited effect strings.
- Emit budget and route-index `qualification_effects` as pipe-delimited text.
- Strengthen optimizer budget rollup coverage for pipe-delimited T2 effects.

## Gates

- `cargo test -q -p route --bin route optimizer_constraint_budget_rolls_up_ledger_subjects`
- `cargo test -q -p route --bin route optimizer_constraint_ledger_replays_t2_game_ops_bundle_relief`
- `cargo test -q -p route --bin route tier_candidate_column`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/optimizer-constraint-budget.csv`; current budget data
  does not need a semantic row update for this compatibility assertion.
- Do not change optimizer scoring or blocker rollup counts.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Optimizer budget rollup now preserves qualification effects as
pipe-delimited downstream-ready clauses.
