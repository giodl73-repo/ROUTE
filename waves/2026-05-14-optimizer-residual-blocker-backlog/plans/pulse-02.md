---
wave: optimizer-residual-blocker-backlog
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - numeracy-checker
  - scope-keeper
---

# Pulse 02 - Residual Backlog Surface

## Mission

Add a gateable backlog artifact grouping remaining constraint-budget blockers
into optimizer action families.

## Scope Inventory

- `data/optimizer-constraint-budget.csv`

## Deliverables

- [x] Add `data/optimizer-residual-blocker-backlog.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blocker counts; no row may claim relief.
- [x] Add tests for grouping and non-promotion.

## Expected Gates

- `cargo test -p route`
- `route optimizer-residual-blocker-backlog --gate`
- `route optimizer-constraint-budget --gate`

## Non-Goals

- Do not edit underlying budget, ledger, selector, or game artifacts.
