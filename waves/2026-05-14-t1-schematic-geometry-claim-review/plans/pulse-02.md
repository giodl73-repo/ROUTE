---
wave: t1-schematic-geometry-claim-review
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - map-standards-reviewer
  - scope-keeper
---

# Pulse 02 - Schematic Claim Surface

## Mission

Add a gateable route-level review docket for T1 shared schematic geometry
claim blockers.

## Scope Inventory

- `data/optimizer-claim-review.csv`
- `data/t1-design-review.csv`
- `data/t1-design-policy-actions.csv`

## Deliverables

- [x] Add `data/t1-schematic-geometry-claim-review.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blockers and forbid map/publication relief.
- [x] Add tests proving route-level shared-segment rows preserve blockers.

## Expected Gates

- `cargo test -p route`
- `route t1-schematic-geometry-claim-review --gate`
- `route optimizer-claim-review --gate`

## Non-Goals

- Do not accept shared-segment map policy.
