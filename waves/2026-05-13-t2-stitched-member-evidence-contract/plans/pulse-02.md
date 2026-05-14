---
wave: t2-stitched-member-evidence-contract
pulse: 02
date: 2026-05-13
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Evidence Contract Surface

## Mission

Add a gateable evidence contract for every stitched-member selection docket row.

## Scope Inventory

- `data/t2-stitched-member-selection-docket.csv`

## Deliverables

- [x] Add `data/t2-stitched-member-evidence-contract.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving evidence contracts do not satisfy evidence.

## Expected Gates

- `cargo test -p route`
- `route t2-stitched-member-evidence-contract --gate`
- `route t2-stitched-member-selection-docket --gate`

## Non-Goals

- Do not edit candidate, registry, or bundle rows.
