---
wave: t4-terminal-access-proof-intake
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 02 - Proof-Intake Surface

## Mission

Add a gateable proof-intake docket for the 69 source-needed terminal-access
source-access rows.

## Scope Inventory

- `data/t4-terminal-access-source-access.csv`

## Deliverables

- [x] Add `data/t4-terminal-access-proof-intake.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blockers and forbid proof acceptance.
- [x] Add tests proving one proof-intake row per source-needed source-access row.

## Expected Gates

- `cargo test -p route`
- `route t4-terminal-access-proof-intake --gate`
- `route t4-terminal-access-source-access --gate`

## Non-Goals

- Do not attach or accept proof sources.
