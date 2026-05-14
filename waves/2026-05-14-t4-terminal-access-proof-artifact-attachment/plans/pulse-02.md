---
wave: t4-terminal-access-proof-artifact-attachment
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 02 - Artifact-Attachment Surface

## Mission

Add a gateable artifact-attachment docket for the 69 source-needed
terminal-access proof source-capture rows.

## Scope Inventory

- `data/t4-terminal-access-proof-source-capture.csv`

## Deliverables

- [x] Add `data/t4-terminal-access-proof-artifact-attachment.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blockers and forbid proof acceptance.
- [x] Add tests proving one attachment row per source-needed capture row.

## Expected Gates

- `cargo test -p route`
- `route t4-terminal-access-proof-artifact-attachment --gate`
- `route t4-terminal-access-proof-source-capture --gate`

## Non-Goals

- Do not attach or accept proof sources.
