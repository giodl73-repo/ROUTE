---
wave: t2-stitched-member-selection-docket
pulse: 02
date: 2026-05-13
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Selection Docket Surface

## Mission

Add a gateable selection docket that classifies every split-plan row as
evidence-needed before any registry mutation.

## Scope Inventory

- `data/t2-stitched-member-split-plan.csv`

## Deliverables

- [x] Add `data/t2-stitched-member-selection-docket.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving selection rows do not select membership.

## Expected Gates

- `cargo test -p route`
- `route t2-stitched-member-selection-docket --gate`
- `route t2-stitched-member-split-plan --gate`

## Non-Goals

- Do not edit candidate, registry, or bundle rows.
