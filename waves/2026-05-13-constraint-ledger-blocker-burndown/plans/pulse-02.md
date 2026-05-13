---
wave: constraint-ledger-blocker-burndown
pulse: 02
date: 2026-05-13
status: planned
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - schematic-cartographer
  - state-dot
  - rural-advocate
  - scope-keeper
---

# Pulse 02 - T4 Zone-Assignment Queue

## Mission

Turn the 63 `zone_assignment_gap` rows into an actionable T4 zone-assignment
queue with explicit zone, demotion, local-inset, or hold decisions.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Constraint budget rows | `data/optimizer-constraint-budget.csv` | Identify every T4 zone-assignment blocker and its candidate next action. |
| Access diagnostics | `data/t3-zone-map-diagnostics.csv` | Carry zone assignment decisions rather than generic `zone-assignment-needed`. |
| Render board | `data/t3-zone-render-board.csv` | Reflect resolved or held local-access backlog rows. |
| T4 local access columns | `data/t4-terminal-access-columns.csv` | Preserve 1h access obligations and zone/local scope. |

## Deliverables

- [ ] Add a zone-assignment action queue or extend the existing diagnostics with
  explicit decisions.
- [ ] Classify each `zone_assignment_gap` row as zone assignment, local inset,
  demotion, source-needed, or held-known.
- [ ] Regenerate T3/T4 diagnostics, render-board, ledger, and budget artifacts.
- [ ] Update `docs/t3-t4-access-optimization.md` only if a new zone taxonomy or
  decision rule is introduced.

## Expected Gates

- `route t3-zone-map-diagnostics --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `cargo test -p route`

## Non-Goals

- Do not draw or publish new maps in this pulse.
- Do not promote T4 local access to T3/T2 without contact or obligation proof.

