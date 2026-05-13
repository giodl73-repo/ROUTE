---
wave: constraint-ledger-blocker-burndown
pulse: 05
date: 2026-05-13
status: planned
depends_on: [pulse-04]
governing_roles:
  - schematic-cartographer
  - traffic-engineer
  - scope-keeper
---

# Pulse 05 - Beck and Publication Blocker Cleanup

## Mission

Clean up Beck/map publication blockers that remain after hard, T4, and game
blocker passes, without letting schematic convenience alter optimizer truth.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| T1 map holds | `data/t1-design-policy-actions.csv`; `data/beck-t1-diagnostics.csv` | Resolve or carry schematic/publication holds for T1 selected lines. |
| T2 map holds | `data/beck-t2-diagnostics.csv` | Resolve transfer, label, long-connector, and geometry blockers where source/stop decisions now exist. |
| Map atlas | `data/map-atlas.csv` | Preserve release claims only for maps that pass diagnostics. |
| Constraint budget | `data/optimizer-constraint-budget.csv` | Verify map/publication blockers are visible and not duplicated. |

## Deliverables

- [ ] Audit remaining Beck/map claim blockers by route and map id.
- [ ] Regenerate Beck diagnostics and affected map/readiness artifacts.
- [ ] Keep unresolved publication blockers in release/manifest surfaces.
- [ ] Update `docs/beck-renderer-contract.md` only if a new map rule is needed.

## Expected Gates

- `route beck-t1-diagnostics --gate`
- `route beck-t2-diagnostics --gate`
- `route t1-beck-alignment --gate`
- `route map-atlas --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Non-Goals

- Do not hand-edit schematic geometry to hide a blocker.
- Do not claim publication readiness for held maps.

