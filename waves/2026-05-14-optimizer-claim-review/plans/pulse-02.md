---
wave: optimizer-claim-review
pulse: 02
date: 2026-05-14
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - scope-keeper
  - citation-auditor
---

# Pulse 02 - Claim-Review Surface

## Mission

Add a gateable review docket for the P1 residual claim-blocker backlog rows.

## Scope Inventory

- `data/optimizer-residual-blocker-backlog.csv`

## Deliverables

- [x] Add `data/optimizer-claim-review.csv`.
- [x] Add a CLI command and gate.
- [x] Preserve blocked claims and forbid blocker relief.
- [x] Add tests proving only `P1-claim-blocker` rows enter the docket.

## Expected Gates

- `cargo test -p route`
- `route optimizer-claim-review --gate`
- `route optimizer-residual-blocker-backlog --gate`

## Non-Goals

- Do not accept or resolve any claim blocker.
