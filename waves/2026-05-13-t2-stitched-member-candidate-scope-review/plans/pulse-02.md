---
wave: t2-stitched-member-candidate-scope-review
pulse: 02
date: 2026-05-13
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - numeracy-checker
---

# Pulse 02 - Candidate Scope Review

## Mission

Add a gateable artifact comparing blocked stitched bundles to route-level
candidate bundle and state scope.

## Deliverables

- [x] Add `data/t2-stitched-member-candidate-scope-review.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving route-level evidence cannot promote blocked bundles.

## Expected Gates

- `route t2-stitched-member-candidate-scope-review --gate`
- `route t2-stitched-member-registry-handoff --gate`
- `cargo test -p route`

## Non-Goals

- Do not mutate candidate rows.
