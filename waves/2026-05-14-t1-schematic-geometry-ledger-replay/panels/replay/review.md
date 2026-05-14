---
wave: t1-schematic-geometry-ledger-replay
type: review
status: done
---

# Replay Review

## Finding

The replay is scoped to the two accepted relief pairs from
`data/t1-schematic-geometry-blocker-relief.csv`. It removes the matching
I40/I95 and I80/I90 T1 schematic blocker rows from the optimizer ledger and
replaces them with pass lineage rows.

## Doctrine Check

- Relief source: accepted T1 shared-segment policy, then blocker-relief rows.
- Ledger behavior: suppress only relieved affected routes.
- Downstream behavior: budget and residual backlog are regenerated from the
  replayed ledger rather than hand-edited.

## Residual Holds

The replay does not resolve terminal-access evidence, source-acquisition,
game/ops, T2 Beck, T3 feeder, or budget-debt families.
