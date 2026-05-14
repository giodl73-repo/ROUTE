---
wave: t2-beck-long-connector-blocker-relief
type: review
status: done
---

# Long-Connector Relief Review

## Finding

The accepted T2 long-connector policy now has relief rows for I44, US83, and
US90. Each row reduces one `map;promotion;publication` blocker inside the
relief artifact.

## Doctrine Check

- Relief follows accepted policy.
- Ledger replay is not performed in this wave.
- The artifact records `pending-optimizer-constraint-ledger-replay`.

## Residual Holds

The optimizer ledger and residual backlog still carry the T2 long-connector
blockers until `data/optimizer-constraint-ledger.csv` consumes this relief
artifact.

