---
wave: t2-stitched-member-evidence-acquisition
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Acquisition Docket Surface

## Mission

Add a gateable acquisition docket for every source-needed stitched-member
evidence contract row.

## Scope Inventory

- `data/t2-stitched-member-evidence-contract.csv`

## Deliverables

- [x] Add `data/t2-stitched-member-evidence-acquisition.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving acquisition rows do not satisfy evidence.

## Expected Gates

- `cargo test -p route`
- `route t2-stitched-member-evidence-acquisition --gate`
- `route t2-stitched-member-evidence-contract --gate`

## Non-Goals

- Do not edit candidate, registry, or bundle rows.
