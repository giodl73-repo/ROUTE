---
wave: t2-overlay-optimizer-action-docket
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Optimizer Action Surface

## Mission

Add a gateable action docket for every residual T2 overlay repair delta row.

## Scope Inventory

- `data/t2-bundle-overlay-repair-delta.csv`

## Deliverables

- [x] Add `data/t2-overlay-optimizer-action-docket.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving action rows do not reduce blockers.

## Expected Gates

- `cargo test -p route`
- `route t2-overlay-optimizer-action-docket --gate`
- `route t2-bundle-overlay-repair-delta --gate`

## Non-Goals

- Do not edit game overlay, registry, or bundle rows.
