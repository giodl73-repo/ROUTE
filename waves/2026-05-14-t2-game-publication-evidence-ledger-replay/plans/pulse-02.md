---
wave: t2-game-publication-evidence-ledger-replay
pulse: 02
status: done
---

# Pulse 02 - Constraint-Ledger Replay

## Deliverable

Teach `route optimizer-constraint-ledger` to consume
`data/t2-game-publication-evidence-blocker-relief.csv`.

## Gates

- Replayed scenarios emit `game_ops_publication_readiness_relief` pass rows.
- Replayed scenarios no longer emit `game_ops_publication_readiness` blocker
  rows.
- Regression test covers the replay rule.

## Result

Done in `crates/route-cli/src/main.rs` and
`data/optimizer-constraint-ledger.csv`.

