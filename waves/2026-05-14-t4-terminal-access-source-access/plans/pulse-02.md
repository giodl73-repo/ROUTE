---
wave: t4-terminal-access-source-access
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 02 - Source-Access Policy Surface

## Mission

Add a gateable source-access policy docket for the 69 held terminal-access proof
review rows.

## Scope Inventory

- `data/t4-terminal-access-proof-review.csv`

## Deliverables

- [x] Add `data/t4-terminal-access-source-access.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blockers and forbid proof acceptance.
- [x] Add tests proving one policy row per held proof review row.

## Expected Gates

- `cargo test -p route`
- `route t4-terminal-access-source-access --gate`
- `route t4-terminal-access-proof-review --gate`

## Non-Goals

- Do not attach or accept proof sources.
