---
wave: t2-bundle-overlay-repair-spine
pulse: 01
date: 2026-05-13
status: done
depends_on: []
governing_roles:
  - optimization-methodologist
  - scope-keeper
  - traffic-engineer
---

# Pulse 01 - Repair Target Intake

## Mission

Create a gateable repair-target intake from the 15 residual rows in
`data/t2-game-ops-binding-decisions.csv`.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Residual decisions | `data/t2-game-ops-binding-decisions.csv` | Target rows classified by repair class |
| Bundle overlays | `data/game/t2-bundle-overlays.csv` | Joined blocker status and service class |
| Optimizer budget | `data/optimizer-constraint-budget.csv` | Preserve visible blockers |

## Deliverables

- [x] Add `data/t2-bundle-overlay-repair-targets.csv`.
- [x] Gate that all 15 held/repair-needed decisions are represented.
- [x] Classify targets into service-class, stop-chain, stitched-member,
  terminal-stop, pavement-debt, local-zone, or manual-review repair classes.
- [x] Add tests for the intake classifier.

## Expected Gates

- `route t2-bundle-overlay-repair-targets --gate`
- `route t2-game-ops-binding-decisions --gate`
- `cargo test -p route`
- targeted `proof check`

## Non-Goals

- Do not change overlay pass/fail status in this pulse.
