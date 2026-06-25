---
wave: milestone-10-t2-budget-regionalizer-effects-compatibility
pulse: 01
date: 2026-06-24
status: done
---

# Pulse 01 - Regionalizer and Selection Preserve Budget Effects

## Deliverables

- Add positive regionalizer coverage for pipe-delimited candidate qualification
  effects.
- Add positive service-selection coverage for regionalizer qualification effects.
- Preserve regionalizer treatment and service-selection decisions.

## Gates

- `cargo test -q -p route --bin route t2_regionalizer`
- `cargo test -q -p route --bin route t2_service_selection_joins_regionalizer_to_beck_diagnostics`
- `cargo test -q -p route --bin route optimizer_constraint_budget_rolls_up_ledger_subjects`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-regionalizer.csv` or
  `data/t2-service-selection.csv`; current data does not need a semantic row
  update for this compatibility assertion.
- Do not change selection decisions or Beck diagnostic joins.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Regionalizer and service-selection coverage now locks in preservation of
normalized budget qualification effects.
