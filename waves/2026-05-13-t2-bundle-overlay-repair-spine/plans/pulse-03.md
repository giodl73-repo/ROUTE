---
wave: t2-bundle-overlay-repair-spine
pulse: 03
date: 2026-05-13
status: done
depends_on: [pulse-02]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - numeracy-checker
---

# Pulse 03 - Bundle Readiness Disposition

## Mission

Decide rows whose bundle status is `needs-stop-chain`, `needs-stitched-members`,
or `needs-terminal-stop`, including the I37 `bundle-bound-review` repair row.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Repair targets | `data/t2-bundle-overlay-repair-targets.csv` | Bundle-readiness subset |
| Segment registry | `data/national-segment-registry.csv` | Member and stop-chain evidence |
| Segment bundles | `data/national-segment-bundles.csv` | Bundle status and readiness action |
| Pavement debt | `data/tier-pavement-debt-budget.csv` | Debt preserved as blocker or budget penalty |

## Deliverables

- [x] Add `data/t2-bundle-readiness-disposition.csv`.
- [x] Decide each readiness blocker as repair-needed, demote, held, or pass
  candidate.
- [x] Keep I37 blocked unless its stop-chain need is resolved.
- [x] Add tests for readiness disposition and no premature pass.

## Expected Gates

- `route t2-bundle-readiness-disposition --gate`
- `route national-segment-bundles --gate`
- `route t2-game-ops-binding-decisions --gate`
- `cargo test -p route`

## Non-Goals

- Do not change pavement-debt pricing or source acquisition policy.
