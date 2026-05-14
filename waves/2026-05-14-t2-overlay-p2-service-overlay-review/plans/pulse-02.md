---
wave: t2-overlay-p2-service-overlay-review
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - P2 Service-Overlay Surface

## Mission

Add a gateable P2 service-overlay review docket for the six T2 overlay optimizer
actions.

## Scope Inventory

- `data/t2-overlay-optimizer-action-docket.csv`

## Deliverables

- [x] Add `data/t2-overlay-p2-service-overlay-review.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving review rows do not reduce blockers.

## Expected Gates

- `cargo test -p route`
- `route t2-overlay-p2-service-overlay-review --gate`
- `route t2-overlay-optimizer-action-docket --gate`

## Non-Goals

- Do not edit game overlay, registry, or bundle rows.
