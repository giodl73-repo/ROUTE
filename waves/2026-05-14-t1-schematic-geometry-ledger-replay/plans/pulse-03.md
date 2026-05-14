---
wave: t1-schematic-geometry-ledger-replay
pulse: 03
status: done
---

# Pulse 03 - Budget and Backlog Close

## Deliverable

Regenerate the optimizer constraint budget and residual blocker backlog from the
replayed ledger, then close the wave.

## Gates

- `route optimizer-constraint-budget --gate`
- `route optimizer-residual-blocker-backlog --gate`
- Full ROUTE gate bundle before commit.

## Result

Done in `data/optimizer-constraint-budget.csv`,
`data/optimizer-residual-blocker-backlog.csv`, `CLOSE.md`, and
`panels/replay/review.md`.
