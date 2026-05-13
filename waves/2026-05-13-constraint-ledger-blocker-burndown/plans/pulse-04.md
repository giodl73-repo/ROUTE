---
wave: constraint-ledger-blocker-burndown
pulse: 04
date: 2026-05-13
status: done
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

- [x] Audit every T2 game bundle-binding blocker.
- [x] Repair stale bundle ids where the registry already has the accepted bundle.
- [x] Carry unresolved game/publication holds with named next artifacts.
- [x] Regenerate game/source artifacts, ledger, budget, and manifests.

## Results

- The audit found no stale bundle ids: game overlays already resolve through
  current `data/national-segment-bundles.csv` identities.
- `data/game/t2-bundle-overlays.csv` now separates 15 unresolved service-class
  rows as `service-class-held-known` rather than treating them as missing overlay
  rows.
- One row remains `bundle-bound-review` (`I37`) because the bundle is present but
  still has stop-chain validation work in `data/national-segment-bundles.csv`.
- Three scenario hooks remain explicit `game_ops_publication_readiness` holds;
  no new game scenario was created.
- Review notes are recorded in
  `waves/2026-05-13-constraint-ledger-blocker-burndown/panels/pulse-04-game-bundle-holds.md`.

## Gate Results

- `cargo test -p route`: pass
- `route game t2-overlays --gate`: pass
- `route game t2-hooks --gate`: pass
- `route t2-bundle-overlays --gate`: pass
- `route optimizer-constraint-ledger --gate`: pass
- `route optimizer-constraint-budget --gate`: pass
- `route tier-optimize --all-tiers --gate`: pass
- `route optimizer-manifest --gate`: pass
- `route release-manifest --gate`: pass
- `scripts/check-mileposts.ps1 -SkipTests`: pass

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

