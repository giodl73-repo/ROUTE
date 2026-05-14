---
wave: t1-shared-segment-policy-acceptance
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - map-standards-reviewer
  - scope-keeper
---

# Pulse 02 - Acceptance Surface

## Mission

Add a gateable acceptance docket for T1 shared-segment map policy.

## Scope Inventory

- `data/t1-shared-segment-map-policy.csv`

## Deliverables

- [x] Add `data/t1-shared-segment-policy-acceptance.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blockers and forbid publication relief.
- [x] Add tests proving policy acceptance does not reduce blockers.

## Expected Gates

- `cargo test -p route`
- `route t1-shared-segment-policy-acceptance --gate`
- `route t1-shared-segment-map-policy --gate`

## Non-Goals

- Do not replay blocker relief.
