---
wave: t4-terminal-access-proof-review
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 02 - Proof Review Surface

## Mission

Add a gateable proof review docket for the 69 source-needed terminal-access
proof artifact placeholders.

## Scope Inventory

- `data/t4-terminal-access-proof-artifacts.csv`

## Deliverables

- [x] Add `data/t4-terminal-access-proof-review.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blockers and forbid proof acceptance.
- [x] Add tests proving one review row per source-needed proof artifact.

## Expected Gates

- `cargo test -p route`
- `route t4-terminal-access-proof-review --gate`
- `route t4-terminal-access-proof-artifacts --gate`

## Non-Goals

- Do not attach or accept proof sources.
