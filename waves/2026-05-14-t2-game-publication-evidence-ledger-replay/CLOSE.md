---
wave: t2-game-publication-evidence-ledger-replay
date_closed: 2026-05-14
status: done
---

# Close - T2 Game Publication Evidence Ledger Replay

## Decision

Accepted T2 game publication evidence relief now reaches the optimizer
constraint ledger, constraint budget, residual backlog, and optimizer run
manifest.

## Evidence

- `data/optimizer-constraint-ledger.csv` now has 136 rows, including three
  `game_ops_publication_readiness_relief` pass rows.
- `data/optimizer-constraint-budget.csv` now has 135 rows.
- `data/optimizer-residual-blocker-backlog.csv` now has four rows and no T2
  `game_ops_publication_readiness` backlog family.
- Residual claim blockers dropped from 89 to 86.
- `data/t1-line-selector.csv` remains a complete selection output after
  `route tier-optimize --all-tiers --gate`; map PNG refresh remains downstream.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route optimizer-residual-blocker-backlog --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t2-game-publication-evidence-ledger-replay`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Resume residual blocker burn-down from the largest remaining optimizer backlog
families before any map PNG refresh or scenario publication.

