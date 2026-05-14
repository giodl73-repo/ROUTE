---
wave: t2-beck-long-connector-ledger-replay
pulse: 02
status: done
---

# Pulse 02 - Constraint-Ledger Replay

## Deliverable

Wire long-connector relief into `data/optimizer-constraint-ledger.csv`.

## Gates

- Relieved routes suppress matching `beck_long_connector` blocker rows.
- Relieved routes emit `beck_long_connector_relief` pass rows.
- Replay has a regression test.

## Result

Done by `route optimizer-constraint-ledger --gate`.

