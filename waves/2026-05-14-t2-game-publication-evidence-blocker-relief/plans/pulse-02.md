---
wave: t2-game-publication-evidence-blocker-relief
pulse: 02
status: done
---

# Pulse 02 - Blocker-Relief Surface

## Deliverable

Add a CLI-generated relief artifact at
`data/t2-game-publication-evidence-blocker-relief.csv`.

## Gates

- One relief row exists for each accepted policy row.
- Relief rows reduce blockers from 1 to 0 with `claim_blocker_delta = -1`.
- Rows point to `data/optimizer-constraint-ledger.csv`.

## Result

Done in the `route t2-game-publication-evidence-blocker-relief` command and
CSV artifact.

