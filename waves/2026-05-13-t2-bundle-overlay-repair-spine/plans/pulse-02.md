---
wave: t2-bundle-overlay-repair-spine
pulse: 02
date: 2026-05-13
status: planned
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - schematic-cartographer
  - scope-keeper
---

# Pulse 02 - Service-Class Repair Docket

## Mission

Route service-class-held T2 rows to the correct repair surface without treating
missing Beck/service metadata as a pass.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Repair targets | `data/t2-bundle-overlay-repair-targets.csv` | Service-class subset |
| Beck diagnostics | `data/beck-t2-diagnostics.csv` | Missing or held service-class reason |
| Service overlays | `data/game/t2-service-overlays.csv` | Required overlay repair path |
| Local/zone treatment | `data/t3-t4-pressure-intake.csv` | Rows that should stay below national T2 game overlay |

## Deliverables

- [ ] Add `data/t2-service-class-repair-docket.csv`.
- [ ] Split repair actions between Beck diagnostic authoring, service overlay
  correction, local-zone handoff, and explicit hold.
- [ ] Ensure any local-relief rows remain below national game overlay until a
  local/zone role is explicit.
- [ ] Add tests for service-class repair routing.

## Expected Gates

- `route t2-service-class-repair-docket --gate`
- `route game t2-overlays --gate`
- `route t2-game-ops-binding-decisions --gate`
- `cargo test -p route`

## Non-Goals

- Do not author new map geometry or new playable scenarios.
