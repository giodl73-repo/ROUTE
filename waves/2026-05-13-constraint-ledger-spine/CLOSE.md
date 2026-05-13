---
name: Constraint Ledger Spine Closeout
slug: constraint-ledger-spine-closeout
type: plan
status: validated
rubric_version: v1.0
author: route-pulse
created: 2026-05-13
updated: 2026-05-13
sources:
  - waves/2026-05-13-constraint-ledger-spine/WAVE.md
  - docs/optimizer-constraint-ledger-spec.md
  - data/optimizer-constraint-ledger.csv
  - data/optimizer-constraint-budget.csv
---

# Constraint Ledger Spine Closeout

## Decision

Close the wave as complete.

The optimizer constraint ledger is now the shared spine for migrated selector,
renderer, source-readiness, and game/publication blockers. The current ledger
has 143 normalized rows and the selector-facing budget has 138 rollup rows.
Held rows remain visible: 117 claim blockers, one hard blocker, and 12 review
rows are carried through the budget surface instead of being erased.

## Done Criteria Review

| Criterion | Closeout finding |
|---|---|
| `optimizer-constraint-ledger --gate` passes and names every migrated source family | Met. The gate names pavement debt, route budget, T1 promise/topology, T2 duplication, T3/T4 access, Beck diagnostics, source acquisition, and game ops classes. |
| `optimizer-constraint-budget --gate` passes and selectors consume the rollup | Met. T1/T2/T3/T4 selector artifacts carry generalized constraint budget fields while preserving compatibility fields where downstream tools still expect them. |
| T1/T2/T3/T4 selector artifacts carry generalized constraint-budget fields | Met. Route, bundle, regionalizer, service-selection, zone-route, terminal-access, and access-gap outputs carry blocker/debt/penalty/class/ledger fields. |
| Beck diagnostics enter the same ledger | Met. T1/T2 schematic rows normalize to Beck claim-blocking classes and roll up through the budget. |
| Game/source rows enter the same ledger | Met. Source-fetch policy rows, scenario hook holds, and bundle overlay binding holds normalize into source acquisition and game ops classes. |
| Final wave gates pass | Met for the pulse gate bundle at closeout. |

## Doctrine Result

The doctrine shift is durable enough for `data/significant-moments.csv`: future
optimizer, renderer, source, and game artifacts should not carry blocker logic
only in side reports. If a claim can block selection, promotion, SLA readiness,
map publication, game use, source acquisition, or payment, it belongs in
`data/optimizer-constraint-ledger.csv` and, when candidate-facing, in
`data/optimizer-constraint-budget.csv`.

## Residual Holds

- The ledger is not a mathematical optimality proof.
- Held claim blockers are valid outputs, not failures, when they name blocked
  claims, repair actions, and next artifacts.
- Future blocker families remain incomplete until they normalize into the same
  schema and budget surface.

