---
wave: t2-stitched-member-source-access-policy
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Source Access Policy Surface

## Mission

Add a gateable source-access policy row for every source-needed stitched-member
acquisition docket row.

## Scope Inventory

- `data/t2-stitched-member-evidence-acquisition.csv`

## Deliverables

- [x] Add `data/t2-stitched-member-source-access-policy.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving policy rows do not fetch or accept evidence.

## Expected Gates

- `cargo test -p route`
- `route t2-stitched-member-source-access-policy --gate`
- `route t2-stitched-member-evidence-acquisition --gate`

## Non-Goals

- Do not edit candidate, registry, or bundle rows.
