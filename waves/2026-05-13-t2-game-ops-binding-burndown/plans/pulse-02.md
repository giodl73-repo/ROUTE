---
wave: t2-game-ops-binding-burndown
pulse: 02
date: 2026-05-13
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Binding Decision Docket

## Mission

Classify each T2 game/ops binding intake row as bound, repair-needed, demote, or
held.

## Deliverables

- [x] Add `data/t2-game-ops-binding-decisions.csv`.
- [x] Require bound rows to have bundle id, service class, and pass validation.
- [x] Keep review rows visible as repair-needed, demote, or held.
- [x] Gate residual blocker decisions and next artifacts.

## Expected Gates

- `route t2-game-ops-binding-intake --gate`
- `route t2-game-ops-binding-decisions --gate`
- `cargo test -p route`

## Non-Goals

- Do not edit bundle geometry.

