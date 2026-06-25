---
wave: milestone-10-t2-relief-ledger-effects-compatibility
pulse: 01
date: 2026-06-24
status: done
---

# Pulse 01 - Optimizer Ledger Preserves Relief Effects

## Deliverables

- Strengthen optimizer ledger replay coverage for T2 game/ops blocker-relief
  qualification effects.
- Confirm pipe-delimited relief effects survive in ledger `optimizer_effect`.
- Preserve game/ops bundle relief replay behavior and residual blocker removal.

## Gates

- `cargo test -q -p route --bin route optimizer_constraint_ledger_replays_t2_game_ops_bundle_relief`
- `cargo test -q -p route --bin route t2_game_ops_bundle_evidence_blocker_relief`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/optimizer-constraint-ledger.csv`; current ledger data
  does not need a semantic row update for this compatibility assertion.
- Do not change optimizer ledger replay behavior.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Optimizer ledger coverage now locks in preservation of blocker-relief
qualification effects.
