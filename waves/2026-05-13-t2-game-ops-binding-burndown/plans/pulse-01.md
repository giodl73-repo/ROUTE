---
wave: t2-game-ops-binding-burndown
pulse: 01
date: 2026-05-13
status: done
depends_on: []
governing_roles:
  - optimization-methodologist
  - scope-keeper
  - traffic-engineer
---

# Pulse 01 - Binding Blocker Intake

## Mission

Create a gateable intake of T2 `game_ops_bundle_binding` blockers from the
constraint budget.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Constraint budget | `data/optimizer-constraint-budget.csv` | T2 game/ops blocker intake rows. |
| Bundle overlays | `data/game/t2-bundle-overlays.csv` | Cross-reference bundle binding status. |
| Wave card | active wave | Mark pulse complete after gates pass. |

## Deliverables

- [x] Add `data/t2-game-ops-binding-intake.csv`.
- [x] Gate that every row is T2 and includes `game_ops_bundle_binding`.
- [x] Preserve route/bundle ids, blocked claims, next artifacts, and validation status.
- [x] Add tests for filtering only game/ops binding blockers.

## Expected Gates

- `route t2-game-ops-binding-intake --gate`
- `route optimizer-constraint-budget --gate`
- `cargo test -p route`
- targeted `proof check`

## Non-Goals

- Do not resolve blockers in this pulse.

