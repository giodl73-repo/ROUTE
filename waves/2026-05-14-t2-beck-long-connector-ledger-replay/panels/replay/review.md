---
wave: t2-beck-long-connector-ledger-replay
type: review
status: done
---

# Replay Review

## Finding

The replay is scoped to the three accepted relief rows from
`data/t2-beck-long-connector-blocker-relief.csv`. It removes the matching T2
Beck long-connector blocker rows from the optimizer ledger and replaces them
with pass lineage rows.

## Doctrine Check

- Relief source: authored policy, accepted policy, then blocker-relief rows.
- Ledger behavior: suppress only relieved long-connector routes.
- Downstream behavior: budget and residual backlog are regenerated from the
  replayed ledger rather than hand-edited.

## Residual Holds

The replay does not resolve T4 terminal-access evidence, source-acquisition,
game/ops, or budget-debt families.

