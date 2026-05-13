---
wave: t4-terminal-contact-evidence
pulse: 02
date: 2026-05-13
status: done
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

- [x] Classify all `t3-great-lakes` terminal rows.
- [x] Separate source-backed, source-needed, demotion/local-only, and held-known
  decisions.
- [x] Name any scenario-ready candidate and next scenario artifact, if earned.
- [x] Regenerate affected T4/access-gap/ledger/budget artifacts.

## Expected Gates

- `route t4-terminal-access-columns --gate`
- `route t4-terminal-contact-evidence --gate`
- `route t3-t4-access-gaps --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `cargo test -p route`

## Evidence

- Classified all 33 `t3-great-lakes` rows in
  `data/t4-terminal-contact-evidence.csv` with candidate terminal districts:
  Chicago Intermodal Complex (4), Columbus South (8), Detroit Livernois (5),
  Indianapolis Avon (3), Minneapolis Twin Cities (1), New York Fresh Pond (6),
  Philadelphia Frankford (3), and St. Louis Gateway (3).
- No row earned source-backed or scenario-ready status because no
  route-to-terminal contact proof source exists yet; all 33 Great Lakes rows
  remain `source-needed` claim blockers with Pulse 02 evidence recorded.
- Gates passed: `cargo test -p route`, `route t4-terminal-access-columns
  --gate`, `route t4-terminal-contact-evidence --gate`, `route
  t3-t4-access-gaps --gate`, `route optimizer-constraint-ledger --gate`, and
  `route optimizer-constraint-budget --gate`.

## Non-Goals

- Do not process the other four zones in this pulse.
- Do not create a scenario unless a contact row earns scenario-ready status.
