---
wave: t2-stitched-member-split-plan
pulse: 02
date: 2026-05-13
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Split Plan Surface

## Mission

Add a gateable split-plan artifact that maps each split decision to
state-scoped candidate bundle rows.

## Scope Inventory

- `data/t2-stitched-member-decision-docket.csv`
- `data/tier-segment-candidates.csv`

## Deliverables

- [x] Add `data/t2-stitched-member-split-plan.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving split-plan rows do not mutate membership.

## Expected Gates

- `cargo test -p route`
- `route t2-stitched-member-split-plan --gate`
- `route t2-stitched-member-decision-docket --gate`

## Non-Goals

- Do not edit candidate, registry, or bundle rows.
