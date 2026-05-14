---
wave: t2-stitched-member-proof-intake
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Proof Intake Surface

## Mission

Add a gateable proof-intake docket for every source-needed stitched-member
source-access policy row.

## Scope Inventory

- `data/t2-stitched-member-source-access-policy.csv`

## Deliverables

- [x] Add `data/t2-stitched-member-proof-intake.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving intake rows do not attach or accept evidence.

## Expected Gates

- `cargo test -p route`
- `route t2-stitched-member-proof-intake --gate`
- `route t2-stitched-member-source-access-policy --gate`

## Non-Goals

- Do not edit candidate, registry, or bundle rows.
