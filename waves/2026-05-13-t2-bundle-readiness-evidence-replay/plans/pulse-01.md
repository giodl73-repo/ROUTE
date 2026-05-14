---
wave: t2-bundle-readiness-evidence-replay
pulse: 01
date: 2026-05-13
status: done
depends_on: []
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 01 - Replay Decision Surface

## Mission

Create a gateable replay decision surface for the four readiness evidence rows.

## Deliverables

- [x] Add `data/t2-bundle-readiness-replay-decisions.csv`.
- [x] Gate that every evidence row has a replay decision.
- [x] Preserve game, incident, publication, and upgrade blockers.
- [x] Add tests for no bound status from evidence replay.

## Expected Gates

- `route t2-bundle-readiness-replay-decisions --gate`
- `route t2-bundle-readiness-repair-evidence --gate`
- `cargo test -p route`

## Non-Goals

- Do not update game/ops binding decisions.
