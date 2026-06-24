---
wave: milestone-10-t2-binding-decision-qualification-compatibility
date_open: 2026-06-23
date_close: 2026-06-23
status: done
source: goal-resume
---

# Milestone 10 T2 Binding Decision Qualification Compatibility

## Mission

Prepare the game/ops binding decision surface to preserve selector-facing
qualification effects when future active intake rows carry them, without
breaking existing binding decision CSVs that predate the column.

## Pulses

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Binding decisions accept qualification effects | done | `T2GameOpsBindingDecisionRow`; `cargo test -q -p route --bin route t2_game_ops_binding_decisions_preserve_residual_blockers`; `npm run check:l2` |

## Close Evidence

`T2GameOpsBindingDecisionRow` now has a defaulted `qualification_effects` field,
and generated future decision rows copy it from `T2GameOpsBindingIntakeRow`.
The field is defaulted so historical decision CSVs remain readable.
