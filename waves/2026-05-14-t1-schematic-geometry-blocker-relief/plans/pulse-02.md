---
wave: t1-schematic-geometry-blocker-relief
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - map-standards-reviewer
  - numeracy-checker
---

# Pulse 02 - Blocker-Relief Surface

## Mission

Add a gateable blocker-relief docket from accepted T1 shared-segment policy.

## Scope Inventory

- `data/t1-shared-segment-policy-acceptance.csv`

## Deliverables

- [x] Add `data/t1-schematic-geometry-blocker-relief.csv`.
- [x] Add a CLI command and gate.
- [x] Reduce accepted pair blockers to zero in the relief artifact.
- [x] Add tests proving relief rows reduce accepted blockers.

## Expected Gates

- `cargo test -p route`
- `route t1-schematic-geometry-blocker-relief --gate`
- `route t1-shared-segment-policy-acceptance --gate`

## Non-Goals

- Do not replay the optimizer constraint ledger.
