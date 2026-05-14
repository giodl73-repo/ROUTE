---
wave: t4-terminal-access-proof-artifacts
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 02 - Proof Artifact Placeholder Surface

## Mission

Add a gateable proof artifact placeholder docket for the 69 source-needed
terminal-access acquisition tasks.

## Scope Inventory

- `data/t4-terminal-access-proof-acquisition.csv`

## Deliverables

- [x] Add `data/t4-terminal-access-proof-artifacts.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blockers and forbid proof acceptance.
- [x] Add tests proving one placeholder per not-attached acquisition task.

## Expected Gates

- `cargo test -p route`
- `route t4-terminal-access-proof-artifacts --gate`
- `route t4-terminal-access-proof-acquisition --gate`

## Non-Goals

- Do not attach proof sources.
