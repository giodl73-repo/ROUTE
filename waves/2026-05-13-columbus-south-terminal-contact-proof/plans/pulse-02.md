---
wave: columbus-south-terminal-contact-proof
pulse: 02
date: 2026-05-13
status: done
depends_on: [pulse-01]
governing_roles:
  - citation-auditor
  - state-dot
  - scope-keeper
---

# Pulse 02 - Source Access Contract

## Mission

Decide and gate the source-access path for Columbus South terminal-contact proof.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Source fetch policy | `docs/source-fetch-cache-policy.md`; `data/source-fetch-policy.csv` | Confirm whether cached/manual or live fetch is allowed. |
| Columbus intake | Pulse 01 artifact | Add source-access decision fields. |
| Release surface | `data/release-manifest.csv` | Keep source artifacts held-public. |

## Deliverables

- [x] Add a Columbus source-access contract or extend the intake artifact.
- [x] Mark live fetch unsupported unless a safe cache command exists.
- [x] Name required source metadata: title, URL/cache path, capture date, route,
  terminal district, and contact statement.
- [x] Record unsupported source access as blockers, not gate failures.

## Expected Gates

- `route source-fetch-policy --gate`
- `route t4-terminal-contact-source-plan --gate`
- `route t4-terminal-columbus-proof-intake --gate`
- `route t4-terminal-columbus-source-access --gate`
- `cargo test -p route`

## Non-Goals

- Do not add an unsafe live fetcher.
- Do not promote any proof row.
