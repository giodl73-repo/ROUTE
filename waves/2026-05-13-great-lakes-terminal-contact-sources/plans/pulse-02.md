---
wave: great-lakes-terminal-contact-sources
pulse: 02
date: 2026-05-13
status: done
depends_on: [pulse-01]
governing_roles:
  - freight-economist
  - state-dot
  - citation-auditor
---

# Pulse 02 - District Source Catalog

## Mission

Classify the eight Great Lakes candidate terminal districts into source families
and proof requirements without claiming route contact.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Terminal seeds | `data/intermodal_terminals.csv` | Use only as district seed list. |
| Source plan | Pulse 01 artifact | Add district-level source-family rows. |
| Contact queue | `data/t4-terminal-contact-evidence.csv` | Keep 33 rows source-needed. |

## Deliverables

- [x] Add district-level source family rows for all eight Great Lakes candidate
  terminal districts.
- [x] Name proof fields required to show route-to-terminal operational contact.
- [x] Mark source families as planned/source-needed unless a safe cache policy
  already exists.
- [x] Document unsupported source families as blockers, not failures.

## Expected Gates

- `route t4-terminal-contact-source-plan --gate`
- `route t4-terminal-contact-evidence --gate`
- `cargo test -p route`

## Non-Goals

- Do not classify individual route contacts in this pulse.
- Do not add live fetchers.
