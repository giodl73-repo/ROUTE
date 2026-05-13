---
wave: great-lakes-terminal-contact-sources
pulse: 03
date: 2026-05-13
status: done
depends_on: [pulse-02]
governing_roles:
  - traffic-engineer
  - freight-economist
  - numeracy-checker
---

# Pulse 03 - Route Contact Proof Docket

## Mission

Create route-level source-acquisition tasks for the 33 Great Lakes terminal
contact rows.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Great Lakes contact rows | `data/t4-terminal-contact-evidence.csv` | One proof task per row. |
| Source plan | Pulse 01 artifact | Attach district source families and proof fields. |
| Scenario docket | `data/t4-terminal-scenario-readiness.csv` | Keep empty unless proof exists. |

## Deliverables

- [x] Emit route-level proof docket rows for all 33 Great Lakes rows.
- [x] Each task names route, terminal district, proof field, source family,
  selected higher-tier attachment requirement, and next artifact.
- [x] Preserve all rows as source-needed unless proof is traceable.
- [x] Add tests for one source-needed row per Great Lakes contact row.

## Expected Gates

- `route t4-terminal-contact-source-plan --gate`
- `route t4-terminal-contact-evidence --gate`
- `route t4-terminal-scenario-readiness --gate`
- `cargo test -p route`

## Non-Goals

- Do not create a scenario.
- Do not modify non-Great-Lakes rows.
