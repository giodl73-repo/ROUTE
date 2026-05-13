---
wave: constraint-ledger-spine
pulse: 03
date: 2026-05-13
status: done
governing_roles:
  - optimization-methodologist
  - freight-economist
  - scope-keeper
---

# Pulse 03 - Constraint Ledger and Budget Commands

## Mission

Specify, review, implement, and gate the normalized optimizer constraint ledger
plus the selector-facing budget rollup.

## Delivered

- `docs/optimizer-constraint-ledger-spec.md`
- `docs/reviews/optimizer-constraint-ledger-review.md`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`

## Evidence

Commits: `838e64d`, `49c9988`, `317b501`, `b74fd4c`.

## Gates

- [x] `route optimizer-constraint-ledger --gate`
- [x] `route optimizer-constraint-budget --gate`
- [x] `cargo test -p route`
