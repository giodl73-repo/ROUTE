---
wave: t4-terminal-access-proof-acquisition
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 02 - Acquisition Task Surface

## Mission

Add a gateable proof acquisition task docket for the 69 held terminal-access
review rows.

## Scope Inventory

- `data/t4-terminal-access-evidence-review.csv`

## Deliverables

- [x] Add `data/t4-terminal-access-proof-acquisition.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blockers and forbid proof acceptance.
- [x] Add tests proving one task per held review row.

## Expected Gates

- `cargo test -p route`
- `route t4-terminal-access-proof-acquisition --gate`
- `route t4-terminal-access-evidence-review --gate`

## Non-Goals

- Do not attach proof sources.
