---
wave: t4-terminal-contact-evidence
pulse: 04
date: 2026-05-13
status: planned
depends_on: [pulse-03]
governing_roles:
  - traffic-engineer
  - freight-economist
  - scope-keeper
---

# Pulse 04 - Scenario Readiness Docket

## Mission

Turn any source-backed terminal contact rows into a bounded scenario-readiness
docket while keeping source-needed rows out of scenario and publication claims.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Contact queue | Pulse 01 artifact | Identify scenario-ready rows, if any. |
| Game/scenario spine | `data/game/t2-scenario-hooks.csv`; `data/game/campaign-spine.csv` | Name candidate scenario artifacts without creating unsupported scenarios. |
| Release surface | `data/release-manifest.csv` | Preserve publication holds. |
| Constraint budget | `data/optimizer-constraint-budget.csv` | Verify scenario readiness does not erase source holds. |

## Deliverables

- [ ] Create a terminal scenario-readiness docket or equivalent queue.
- [ ] Name scenario candidate(s) only for source-backed terminal contact rows.
- [ ] Require scenario-ready rows to carry contact proof source, operational
  contact basis, selected higher-tier attachment, and freight/access rationale.
- [ ] Keep source-needed rows release-held.
- [ ] Update review notes for any scenario candidate.

## Expected Gates

- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route release-manifest --gate`
- `cargo test -p route`

## Non-Goals

- Do not implement a new playable scenario in this pulse.
- Do not use scenario readiness as publication readiness.
