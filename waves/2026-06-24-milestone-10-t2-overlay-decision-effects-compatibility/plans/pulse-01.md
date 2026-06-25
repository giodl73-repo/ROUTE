---
wave: milestone-10-t2-overlay-decision-effects-compatibility
pulse: 01
date: 2026-06-24
status: done
---

# Pulse 01 - Binding Decisions Merge Overlay Effects

## Deliverables

- Merge binding-intake and bundle-overlay `qualification_effects` in
  `T2GameOpsBindingDecisionRow` generation.
- Add positive test coverage for preserving both effect sources.
- Preserve existing decision status, blocker, and next-artifact behavior.

## Gates

- `cargo test -q -p route --bin route t2_game_ops_binding_decisions`
- `cargo test -q -p route --bin route t2_bundle_overlay`
- `cargo test -q -p route --bin route t2_route_family_splits`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-game-ops-binding-decisions.csv`; current decision
  data does not need a semantic row update for this compatibility field.
- Do not change game/ops binding decisions or blocker policy.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future game/ops binding decisions preserve qualification effects from both
intake and bundle overlay inputs.
