---
wave: t2-national-bundle-readiness-audit
pulse: 02
date: 2026-05-13
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - numeracy-checker
---

# Pulse 02 - Bundle Audit Surface

## Mission

Add a gateable audit artifact comparing readiness replay rows to the current
national bundle statuses.

## Deliverables

- [x] Add `data/t2-national-bundle-readiness-audit.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving audit rows cannot promote unresolved readiness claims.

## Expected Gates

- `route t2-national-bundle-readiness-audit --gate`
- `route national-segment-bundles --gate`
- `route t2-bundle-readiness-replay-decisions --gate`
- `cargo test -p route`

## Non-Goals

- Do not edit `data/national-segment-bundles.csv`.
