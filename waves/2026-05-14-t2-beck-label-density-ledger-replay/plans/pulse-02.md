---
wave: t2-beck-label-density-ledger-replay
pulse: 02
status: done
---

# Pulse 02 - Constraint-Ledger Replay

## Deliverable

Wire label-density relief into `route optimizer-constraint-ledger` so accepted
relief suppresses matching `beck_label_density` rows and emits pass lineage rows.

## Gates

- Regression test proves `beck_label_density` rows are suppressed.
- `route optimizer-constraint-ledger --gate`

## Result

Done in `crates/route-cli/src/main.rs` and
`data/optimizer-constraint-ledger.csv`.
