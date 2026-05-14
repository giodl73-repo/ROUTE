---
wave: t2-service-overlay-diagnostic-binding
pulse: 01
date: 2026-05-13
status: done
depends_on: []
governing_roles:
  - optimization-methodologist
  - schematic-cartographer
  - scope-keeper
---

# Pulse 01 - Diagnostic Decision Surface

## Mission

Create a gateable decision surface for the seven `service-overlay` T2 repair
rows.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Service repair docket | `data/t2-service-class-repair-docket.csv` | Seven service-overlay rows |
| Repair targets | `data/t2-bundle-overlay-repair-targets.csv` | Blocked claim preservation |
| Service diagnostics | `data/t2-service-diagnostic-queue.csv` | Beck diagnostic requirement |

## Deliverables

- [x] Add `data/t2-service-overlay-diagnostic-decisions.csv`.
- [x] Gate that every service-overlay row has a diagnostic decision.
- [x] Preserve game, incident, publication, and upgrade blockers.
- [x] Add tests for no premature service-class promotion.

## Evidence

- `cargo test -p route`
- `route t2-service-overlay-diagnostic-decisions --gate`
- `data/t2-service-overlay-diagnostic-decisions.csv`

## Expected Gates

- `route t2-service-overlay-diagnostic-decisions --gate`
- `route t2-service-class-repair-docket --gate`
- `cargo test -p route`

## Non-Goals

- Do not assign a service class without Beck diagnostic evidence.
