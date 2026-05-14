---
wave: t2-stitched-member-proof-source-capture
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Source Capture Surface

## Mission

Add a gateable source-capture docket for every source-needed stitched-member
proof-intake row.

## Scope Inventory

- `data/t2-stitched-member-proof-intake.csv`

## Deliverables

- [x] Add `data/t2-stitched-member-proof-source-capture.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving capture rows do not attach or accept evidence.

## Expected Gates

- `cargo test -p route`
- `route t2-stitched-member-proof-source-capture --gate`
- `route t2-stitched-member-proof-intake --gate`

## Non-Goals

- Do not edit candidate, registry, or bundle rows.
