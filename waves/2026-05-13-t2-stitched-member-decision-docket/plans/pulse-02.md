---
wave: t2-stitched-member-decision-docket
pulse: 02
date: 2026-05-13
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Decision Docket Surface

## Mission

Add a gateable docket that turns scope findings into explicit split, merge, or
expand review decisions.

## Deliverables

- [x] Add `data/t2-stitched-member-decision-docket.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving docket rows do not repair bundle membership.

## Expected Gates

- `route t2-stitched-member-decision-docket --gate`
- `route t2-stitched-member-candidate-scope-review --gate`
- `cargo test -p route`

## Non-Goals

- Do not mutate candidate, registry, or bundle rows.
