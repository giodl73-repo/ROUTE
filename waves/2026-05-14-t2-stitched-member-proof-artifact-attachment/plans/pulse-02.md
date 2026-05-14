---
wave: t2-stitched-member-proof-artifact-attachment
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 02 - Attachment Surface

## Mission

Add a gateable artifact-attachment docket for every source-needed
stitched-member source-capture row.

## Scope Inventory

- `data/t2-stitched-member-proof-source-capture.csv`

## Deliverables

- [x] Add `data/t2-stitched-member-proof-artifact-attachment.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve `game;incident;publication;upgrade` blockers.
- [x] Add tests proving attachment rows do not attach or accept evidence.

## Expected Gates

- `cargo test -p route`
- `route t2-stitched-member-proof-artifact-attachment --gate`
- `route t2-stitched-member-proof-source-capture --gate`

## Non-Goals

- Do not edit candidate, registry, or bundle rows.
