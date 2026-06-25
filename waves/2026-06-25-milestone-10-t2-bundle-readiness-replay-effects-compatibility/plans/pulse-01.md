---
wave: milestone-10-t2-bundle-readiness-replay-effects-compatibility
pulse: 01
date: 2026-06-25
status: done
---

# Pulse 01 - Readiness Replay Preserves Effects

## Deliverables

- Add `qualification_effects` to `T2BundleReadinessReplayDecisionRow`.
- Merge readiness evidence and repair-delta qualification effects into replay
  decision rows.
- Add focused readiness replay coverage.

## Gates

- `cargo test -q -p route --bin route t2_bundle_readiness_replay_decisions`
- `cargo test -q -p route --bin route t2_bundle_readiness_repair_evidence`
- `cargo test -q -p route --bin route t2_national_bundle_readiness_audit`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-bundle-readiness-replay-decisions.csv`; current data
  does not need a semantic row update for this compatibility assertion.
- Do not change readiness replay decisions.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Bundle readiness replay decision rows now preserve qualification effects
from readiness evidence and repair deltas.
