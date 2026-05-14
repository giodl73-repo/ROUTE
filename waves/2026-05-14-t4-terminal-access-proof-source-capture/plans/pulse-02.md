---
wave: t4-terminal-access-proof-source-capture
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 02 - Source-Capture Surface

## Mission

Add a gateable source-capture docket for the 69 source-needed terminal-access
proof-intake rows.

## Scope Inventory

- `data/t4-terminal-access-proof-intake.csv`

## Deliverables

- [x] Add `data/t4-terminal-access-proof-source-capture.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blockers and forbid proof acceptance.
- [x] Add tests proving one source-capture row per source-needed proof-intake row.

## Expected Gates

- `cargo test -p route`
- `route t4-terminal-access-proof-source-capture --gate`
- `route t4-terminal-access-proof-intake --gate`

## Non-Goals

- Do not attach or accept proof sources.
