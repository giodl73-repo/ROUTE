---
wave: t2-stitched-member-proof-review-docket
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Proof Review Surface

## Mission

Add a gateable proof-review docket for every source-needed stitched-member
artifact-attachment row.

## Scope Inventory

- `data/t2-stitched-member-proof-artifact-attachment.csv`

## Deliverables

- [x] Add `data/t2-stitched-member-proof-review-docket.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving review rows do not accept proof.

## Expected Gates

- `cargo test -p route`
- `route t2-stitched-member-proof-review-docket --gate`
- `route t2-stitched-member-proof-artifact-attachment --gate`

## Non-Goals

- Do not edit candidate, registry, or bundle rows.
