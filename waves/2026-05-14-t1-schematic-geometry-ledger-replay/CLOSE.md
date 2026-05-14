---
wave: t1-schematic-geometry-ledger-replay
date_closed: 2026-05-14
status: done
---

# Close - T1 Schematic Geometry Ledger Replay

## Decision

Accepted T1 schematic geometry relief now reaches the optimizer constraint
ledger, constraint budget, and residual backlog.

## Evidence

- `data/optimizer-constraint-ledger.csv` now has 136 rows.
- `data/optimizer-constraint-budget.csv` now has 135 rows.
- `data/optimizer-residual-blocker-backlog.csv` now has nine rows and no
  T1 `beck_schematic_geometry|schematic_geometry` backlog family.
- Residual claim blockers dropped from 117 to 109.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route optimizer-residual-blocker-backlog --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t1-schematic-geometry-ledger-replay`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Resume residual optimizer blocker burn-down from the largest remaining
claim-blocker families before final Beck replacement publication.
