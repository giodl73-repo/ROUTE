---
wave: t2-local-zone-overlay-handoff
pulse: 01
date: 2026-05-13
status: done
depends_on: []
governing_roles:
  - optimization-methodologist
  - schematic-cartographer
  - scope-keeper
---

# Pulse 01 - Local-Zone Handoff Surface

## Mission

Create a gateable handoff surface for the seven `local-zone` T2 repair rows.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Service repair docket | `data/t2-service-class-repair-docket.csv` | Seven local-zone rows |
| Zone route columns | `data/t3-zone-route-columns.csv` | zone role and decision context |
| Zone render board | `data/t3-zone-render-board.csv` | map treatment context |

## Deliverables

- [x] Add `data/t2-local-zone-overlay-handoff.csv`.
- [x] Gate that every local-zone row has a handoff decision.
- [x] Preserve game, incident, publication, and upgrade blockers.
- [x] Add tests for no national T2 promotion from local-zone handoff.

## Evidence

- `cargo test -p route`
- `route t2-local-zone-overlay-handoff --gate`
- `data/t2-local-zone-overlay-handoff.csv`

## Expected Gates

- `route t2-local-zone-overlay-handoff --gate`
- `route t2-service-class-repair-docket --gate`
- `cargo test -p route`

## Non-Goals

- Do not promote local-zone rows to T2 game overlays.
