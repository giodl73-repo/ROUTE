---
wave: constraint-ledger-spine
pulse: 05
date: 2026-05-13
status: done
governing_roles:
  - optimization-methodologist
  - schematic-cartographer
  - scope-keeper
---

# Pulse 05 - Beck Diagnostics Enter the Ledger

## Mission

Normalize T1/T2 Beck diagnostic rows into the optimizer constraint ledger so map
and publication blockers affect the same selector-facing budget surface as
pavement, topology, and access pressure.

## Delivered

- T1 non-OK Beck diagnostics become `beck_schematic_geometry` claim blockers.
- T2 non-OK Beck diagnostics become typed `beck_*` schematic claim blockers.
- Constraint budget and selector artifacts regenerated.
- Specs and reviews updated to mark Beck diagnostic migration implemented.

## Evidence

Commit: `487eec2`.

## Gates

- [x] `route optimizer-constraint-ledger --gate`
- [x] `route optimizer-constraint-budget --gate`
- [x] `route tier-optimize --all-tiers --gate`
- [x] `route optimizer-manifest --gate`
- [x] `route release-manifest --gate`
- [x] `cargo test -p route`
- [x] `scripts/check-mileposts.ps1 -SkipTests`
