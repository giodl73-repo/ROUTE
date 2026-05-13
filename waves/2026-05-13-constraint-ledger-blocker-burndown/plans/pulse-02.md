---
wave: constraint-ledger-blocker-burndown
pulse: 02
date: 2026-05-13
status: done
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

- [x] Add a zone-assignment action queue or extend the existing diagnostics with
  explicit decisions.
- [x] Classify each `zone_assignment_gap` row as zone assignment, local inset,
  demotion, source-needed, or held-known.
- [x] Regenerate T3/T4 diagnostics, render-board, ledger, and budget artifacts.
- [x] Update `docs/t3-t4-access-optimization.md` only if a new zone taxonomy or
  decision rule is introduced.

## Results

- The 63 opening `zone_assignment_gap` rows were resolved by extending the
  deterministic T3 zone map used by `data/t4-terminal-access-columns.csv`.
- `data/t4-terminal-access-columns.csv` now assigns all 69 T4 local-access rows
  to one of the five existing T3 zones; no new zone taxonomy was introduced.
- `data/t3-t4-access-gaps.csv` now carries those rows as explicit
  `terminal-evidence-needed` holds with zone ids and next artifacts.
- `data/optimizer-constraint-ledger.csv` and
  `data/optimizer-constraint-budget.csv` now contain 0 `zone_assignment_gap`
  rows and 69 `terminal_access_evidence_gap` rows.

## Gate Results

- `cargo test -p route`: pass
- `route t4-terminal-access-columns --gate`: pass
- `route t3-t4-access-gaps --gate`: pass
- `route t3-zone-map-diagnostics --gate`: pass
- `route t3-zone-render-board --gate`: pass
- `route optimizer-constraint-ledger --gate`: pass
- `route optimizer-constraint-budget --gate`: pass
- `route tier-optimize --all-tiers --gate`: pass
- `route optimizer-manifest --gate`: pass
- `route release-manifest --gate`: pass
- `scripts/check-mileposts.ps1 -SkipTests`: pass

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

