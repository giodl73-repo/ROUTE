---
wave: t4-terminal-access-evidence-review
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 02 - Terminal Evidence Review Surface

## Mission

Add a gateable terminal-access evidence review docket for the 69 T4 terminal
contact rows.

## Scope Inventory

- `data/t4-terminal-contact-evidence.csv`

## Deliverables

- [x] Add `data/t4-terminal-access-evidence-review.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `map;publication;upgrade` blockers.
- [x] Add tests proving no source-needed row is promoted.

## Expected Gates

- `cargo test -p route`
- `route t4-terminal-access-evidence-review --gate`
- `route t4-terminal-contact-evidence --gate`

## Non-Goals

- Do not attach proof sources or change terminal contact evidence decisions.
