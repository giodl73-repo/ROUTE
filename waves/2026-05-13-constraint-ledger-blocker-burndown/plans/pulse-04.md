---
wave: constraint-ledger-blocker-burndown
pulse: 04
date: 2026-05-13
status: planned
depends_on: [pulse-03]
governing_roles:
  - optimization-methodologist
  - schematic-cartographer
  - freight-industry
  - scope-keeper
---

# Pulse 04 - T2 Game and Bundle-Binding Holds

## Mission

Repair or explicitly carry T2 `game_ops_bundle_binding` and publication-readiness
holds so game overlays target valid bundle identities and visible evidence holds.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Game holds | `data/game/t2-service-overlays.csv`; `data/game/t2-bundle-overlays.csv` | Bind overlays to accepted bundles or carry source/game holds. |
| Bundle identity | `data/national-segment-bundles.csv` | Verify missing bundle references are real blockers, not stale ids. |
| Scenario hooks | `data/game/t2-scenario-hooks.csv` | Keep scenario claims blocked unless evidence/map prerequisites pass. |
| Constraint ledger | `data/optimizer-constraint-ledger.csv` | Reflect repaired or held game ops rows. |

## Deliverables

- [ ] Audit every T2 game bundle-binding blocker.
- [ ] Repair stale bundle ids where the registry already has the accepted bundle.
- [ ] Carry unresolved game/publication holds with named next artifacts.
- [ ] Regenerate game/source artifacts, ledger, budget, and manifests.

## Expected Gates

- `route game t2-hooks --gate`
- `route game t2-overlays --gate`
- `route t2-bundle-overlays --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route release-manifest --gate`
- `cargo test -p route`

## Non-Goals

- Do not create new game scenarios to absorb unresolved blockers.
- Do not let game overlay convenience redefine optimizer bundle identity.

