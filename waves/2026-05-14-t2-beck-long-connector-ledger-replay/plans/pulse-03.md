---
wave: t2-beck-long-connector-ledger-replay
pulse: 03
status: done
---

# Pulse 03 - Budget/Backlog Close

## Deliverable

Regenerate optimizer budget and residual backlog from the replayed ledger, then
close the wave after gates pass.

## Gates

- `data/optimizer-constraint-budget.csv` reflects the replayed pass rows.
- `data/optimizer-residual-blocker-backlog.csv` no longer carries the
  `beck_long_connector` family.
- Final gate bundle passes.

## Result

Done in `CLOSE.md` and `panels/replay/review.md`.

