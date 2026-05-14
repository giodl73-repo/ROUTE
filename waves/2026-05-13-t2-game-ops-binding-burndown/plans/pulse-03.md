---
wave: t2-game-ops-binding-burndown
pulse: 03
date: 2026-05-13
status: done
depends_on: [pulse-02]
governing_roles:
  - optimization-methodologist
  - scope-keeper
  - numeracy-checker
---

# Pulse 03 - Overlay Propagation

## Mission

Propagate binding decisions into game/scenario readiness without promoting held
or repair-needed rows.

## Deliverables

- [x] Add propagation note or artifact if decisions change game surfaces.
- [x] Keep publication/game readiness held for non-bound rows.
- [x] Run game overlay and scenario hook gates.

## Expected Gates

- `route game t2-overlays --gate`
- `route game t2-scenario-hooks --gate`
- `route t2-game-ops-binding-decisions --gate`
- `cargo test -p route`

## Non-Goals

- Do not create a new playable scenario.

