---
wave: t1-shared-segment-map-policy
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - map-standards-reviewer
  - scope-keeper
---

# Pulse 02 - Map-Policy Surface

## Mission

Add a gateable pair-level map-policy docket for T1 shared schematic segments.

## Scope Inventory

- `data/t1-schematic-geometry-claim-review.csv`

## Deliverables

- [x] Add `data/t1-shared-segment-map-policy.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blockers and forbid publication relief.
- [x] Add tests proving pair-level rows preserve blockers.

## Expected Gates

- `cargo test -p route`
- `route t1-shared-segment-map-policy --gate`
- `route t1-schematic-geometry-claim-review --gate`

## Non-Goals

- Do not accept or apply the policy.
