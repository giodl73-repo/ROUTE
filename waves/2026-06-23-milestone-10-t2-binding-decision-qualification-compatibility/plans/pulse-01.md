---
wave: milestone-10-t2-binding-decision-qualification-compatibility
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Binding Decisions Accept Qualification Effects

## Deliverables

- Add `qualification_effects` to `T2GameOpsBindingDecisionRow`.
- Copy `qualification_effects` from `T2GameOpsBindingIntakeRow` when decision
  rows are generated.
- Default the field during deserialization so existing decision CSVs remain
  readable.
- Add gate coverage so future bound decisions cannot drop qualification effects
  without a qualification gate/game-use contract.

## Gates

- `cargo test -q -p route --bin route t2_game_ops_binding_decisions_preserve_residual_blockers`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-game-ops-binding-decisions.csv`; the current
  selector budget has no active unrelieved intake rows.
- Do not change binding decision policy.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future game/ops binding decisions preserve qualification effects from
binding intake, while existing decision CSVs remain compatible.
