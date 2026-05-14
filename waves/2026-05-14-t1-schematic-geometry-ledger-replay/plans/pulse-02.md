---
wave: t1-schematic-geometry-ledger-replay
pulse: 02
status: done
---

# Pulse 02 - Constraint-Ledger Replay

## Deliverable

Teach `route optimizer-constraint-ledger` to load
`data/t1-schematic-geometry-blocker-relief.csv`, suppress relieved T1 schematic
blocker rows, and emit pass rows for relief lineage.

## Gates

- Regression test covers suppression plus pass-row emission.
- `route optimizer-constraint-ledger --gate`

## Result

Done in `crates/route-cli/src/main.rs` and
`data/optimizer-constraint-ledger.csv`.
