---
wave: t2-beck-long-connector-blocker-relief
pulse: 02
status: done
---

# Pulse 02 - Blocker-Relief Surface

## Deliverable

Generate `data/t2-beck-long-connector-blocker-relief.csv` from the accepted
policy rows.

## Gates

- Every accepted policy row has one relief row.
- Each row reduces blocker count from 1 to 0.
- Each row points to `data/optimizer-constraint-ledger.csv` for replay.

## Result

Done by `route t2-beck-long-connector-blocker-relief --gate`.

