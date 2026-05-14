---
wave: t3-lower-tier-feeder-gap-ledger-replay
pulse: 02
status: done
---

# Pulse 02 - Constraint-Ledger Replay

## Deliverables

- Load T3 feeder relief in `route optimizer-constraint-ledger`.
- Suppress only matching `lower_tier_feeder_gap` rows.
- Emit `lower_tier_feeder_gap_relief` pass rows.

## Gates

- `route optimizer-constraint-ledger --gate`
- Regression test for T3 feeder replay.

## Roles

- Numeracy Checker
- Scope Keeper
