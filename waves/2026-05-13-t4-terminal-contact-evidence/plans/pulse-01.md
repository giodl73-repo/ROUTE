---
wave: t4-terminal-contact-evidence
pulse: 01
date: 2026-05-13
status: done
depends_on: []
governing_roles:
  - freight-economist
  - state-dot
  - citation-auditor
  - scope-keeper
---

# Pulse 01 - Terminal Contact Evidence Schema

## Mission

Create the terminal contact evidence queue that will own T4 route-to-terminal
proof decisions for the 69 `terminal_access_evidence_gap` rows.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| T4 terminal holds | `data/t4-terminal-access-columns.csv` | Define the per-route terminal contact decision contract. |
| Access gaps | `data/t3-t4-access-gaps.csv` | Preserve held claims while pointing to the evidence queue. |
| Terminal source seed | `data/intermodal_terminals.csv` | Use as district seed, not proof of route contact. |
| Constraint ledger | `data/optimizer-constraint-ledger.csv` | Keep unresolved claims normalized. |

## Deliverables

- [x] Add a terminal contact evidence artifact or schema extension.
- [x] Gate required fields: route, zone, terminal district, contact basis,
  evidence status, selected higher-tier attachment, decision, next artifact.
- [x] Separate terminal-district seed source fields from route-to-terminal
  contact-proof source fields so `data/intermodal_terminals.csv` cannot be
  mistaken for contact evidence.
- [x] Enumerate allowed decision states and legal status transitions for
  source-needed, source-backed, demotion/local-only, held-known, and
  scenario-ready rows.
- [x] Regenerate affected T4/access-gap/ledger artifacts if producer logic
  changes.
- [x] Add tests for source-needed versus scenario-ready terminal contact rows,
  including a proximity-only row that must stay held.

## Expected Gates

- `route t4-terminal-access-columns --gate`
- `route t4-terminal-contact-evidence --gate`
- `route t3-t4-access-gaps --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `cargo test -p route`

## Evidence

- Added `data/t4-terminal-contact-evidence.csv` with 69 source-needed rows.
- Terminal-evidence access gaps now point to the contact queue as their next
  artifact while remaining normalized claim blockers.
- Gates passed: `cargo test -p route`, `route t4-terminal-access-columns
  --gate`, `route t4-terminal-contact-evidence --gate`, `route
  t3-t4-access-gaps --gate`, `route optimizer-constraint-ledger --gate`, and
  `route optimizer-constraint-budget --gate`.

## Non-Goals

- Do not classify all 69 rows in this pulse.
- Do not claim a terminal district seed proves route contact.
