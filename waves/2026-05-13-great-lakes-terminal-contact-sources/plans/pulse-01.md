---
wave: great-lakes-terminal-contact-sources
pulse: 01
date: 2026-05-13
status: planned
depends_on: []
governing_roles:
  - citation-auditor
  - scope-keeper
  - optimization-methodologist
---

# Pulse 01 - Source Plan Contract

## Mission

Create the Great Lakes terminal-contact source plan contract and gate.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Contact queue | `data/t4-terminal-contact-evidence.csv` | Filter 33 Great Lakes source-needed rows. |
| Source policy | `docs/source-fetch-cache-policy.md`; `data/source-fetch-policy.csv` | Preserve no-live-fetch rule unless policy is added. |
| New source plan | planned | Define source families, required fields, status vocabulary, and next artifacts. |
| Doctrine | `docs/t3-t4-access-optimization.md` | Document source-plan ownership. |

## Deliverables

- [ ] Add `data/t4-terminal-contact-source-plan.csv` or equivalent.
- [ ] Gate required fields: route, zone, terminal district, source family,
  required proof fields, acquisition status, proof blocker, and next artifact.
- [ ] Enforce seed-source versus contact-proof-source separation.
- [ ] Add tests for complete source-needed rows and invalid seed-as-proof rows.

## Expected Gates

- `route t4-terminal-contact-source-plan --gate`
- `route t4-terminal-contact-evidence --gate`
- `cargo test -p route`

## Non-Goals

- Do not fetch sources.
- Do not promote any terminal-contact row.
