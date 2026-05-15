---
wave: map-selector-readiness-checkpoint
date_closed: 2026-05-14
status: done
---

# Close - Map Selector Readiness Checkpoint

## Decision

The current selector set is full again for T1 map work: `data/t1-line-selector.csv`
selects 11 T1 routes with 90 selected stop references, and the 17-row map atlas
passes. No map PNG refresh is warranted in this checkpoint because the render
contracts and gated map inputs did not change.

## Evidence

- `data/t1-line-selector.csv` has 386 rows and 11 selected routes.
- Selected routes consume 90 route-budget units and feed 90 rows in
  `data/t1-stop-selector.csv`.
- `data/map-atlas.csv` has 17 map contracts: one national-tier map, three
  schematic maps, eight T1 regional maps, and five T3 zone maps.
- `data/optimizer-map-hooks.csv` has seven passing hook rows linking optimizer
  outputs to map and game consumers.
- Residual optimizer readiness is not complete: `data/optimizer-residual-blocker-backlog.csv`
  still has 70 claim blockers, including 69 T4 `terminal_access_evidence_gap`
  blockers that require non-seed proof attachments before relief.

## Gate Record

- `route map-atlas --gate`
- `route t1-line-selector --gate`
- `route t1-stop-selector --gate`
- `route t1-beck-alignment --gate`
- `route optimizer-map-hooks --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-map-selector-readiness-checkpoint`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

If the goal is blocker burn-down, resume with T4 terminal-access proof
attachment using real non-seed evidence. If the goal is publication, refresh map
PNGs only after a changed render input or an explicit publication checkpoint.

