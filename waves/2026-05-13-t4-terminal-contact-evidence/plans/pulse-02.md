---
wave: t4-terminal-contact-evidence
pulse: 02
date: 2026-05-13
status: planned
depends_on: [pulse-01]
governing_roles:
  - freight-economist
  - state-dot
  - rural-advocate
  - numeracy-checker
---

# Pulse 02 - Great Lakes Contact Sample

## Mission

Classify the 33-row Great Lakes / Ohio Valley terminal contact sample before
scaling the decision rule to other zones.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Great Lakes T4 holds | `data/t4-terminal-access-columns.csv` | Assign contact decisions for `t3-great-lakes` rows. |
| Terminal districts | `data/intermodal_terminals.csv` | Name candidate districts without overstating proof. |
| Contact queue | Pulse 01 artifact | Populate Great Lakes rows. |
| Constraint budget | `data/optimizer-constraint-budget.csv` | Reflect any source-backed or carried decisions. |

## Deliverables

- [ ] Classify all `t3-great-lakes` terminal rows.
- [ ] Separate source-backed, source-needed, demotion/local-only, and held-known
  decisions.
- [ ] Name any scenario-ready candidate and next scenario artifact, if earned.
- [ ] Regenerate affected T4/access-gap/ledger/budget artifacts.

## Expected Gates

- `route t4-terminal-access-columns --gate`
- `route t3-t4-access-gaps --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `cargo test -p route`

## Non-Goals

- Do not process the other four zones in this pulse.
- Do not create a scenario unless a contact row earns scenario-ready status.
