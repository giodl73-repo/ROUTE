---
wave: milestone-10-t2-binding-intake-qualification-compatibility
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Binding Intake Accepts Qualification Effects

## Deliverables

- Add `qualification_effects` to `T2GameOpsBindingIntakeRow`.
- Copy `qualification_effects` from `OptimizerConstraintBudgetRow` when intake
  rows are generated.
- Default the field during deserialization so existing intake CSVs remain
  readable.
- Strengthen the intake gate so future budget blockers with qualification effects
  cannot drop them in generated intake rows.

## Gates

- `cargo test -q -p route --bin route t2_game_ops_binding_intake_filters_constraint_budget`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-game-ops-binding-intake.csv`; the current relieved
  budget has no active game/ops binding blockers.
- Do not change binding intake selection policy.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future game/ops binding intake rows preserve qualification effects from
budget rollup, while existing intake CSVs remain compatible.
