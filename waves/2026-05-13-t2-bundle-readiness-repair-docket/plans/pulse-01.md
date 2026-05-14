---
wave: t2-bundle-readiness-repair-docket
pulse: 01
date: 2026-05-13
status: done
depends_on: []
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 01 - Readiness Repair Docket

## Mission

Create a gateable repair docket for the four `repair-needed` T2
bundle-readiness rows.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Readiness disposition | `data/t2-bundle-readiness-disposition.csv` | Four repair-needed rows |
| Repair delta | `data/t2-bundle-overlay-repair-delta.csv` | blocked claim preservation |
| Bundle/segment surfaces | `data/national-segment-bundles.csv`; `data/tier-segment-candidates.csv`; `data/t2-service-selection.csv` | named next repair artifacts |

## Deliverables

- [x] Add `data/t2-bundle-readiness-repair-docket.csv`.
- [x] Gate that every repair-needed readiness row has a repair task.
- [x] Preserve game, incident, publication, and upgrade blockers.
- [x] Add tests for no readiness promotion from repair-task authoring.

## Evidence

- `cargo test -p route`
- `route t2-bundle-readiness-repair-docket --gate`
- `data/t2-bundle-readiness-repair-docket.csv`

## Expected Gates

- `route t2-bundle-readiness-repair-docket --gate`
- `route t2-bundle-readiness-disposition --gate`
- `cargo test -p route`

## Non-Goals

- Do not repair bundle geometry in this pulse.
