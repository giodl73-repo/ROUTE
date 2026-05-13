---
wave: terminal-contact-source-acquisition-spine
pulse: 02
date: 2026-05-13
status: planned
depends_on: [pulse-01]
governing_roles:
  - citation-auditor
  - scope-keeper
  - numeracy-checker
---

# Pulse 02 - Source Registry Intake

## Mission

Create a registry/import surface for manual or cached terminal-contact proof
artifacts without adding live-fetch side effects.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Proof contract | Pulse 01 artifact | Registry rows must satisfy contract fields. |
| Source policy | `docs/source-fetch-cache-policy.md` | Manual/cache import keeps cache provenance visible. |
| Great Lakes docket | `data/t4-terminal-contact-proof-docket.csv` | Candidate route rows can be matched to registry entries. |

## Deliverables

- [ ] Add terminal-contact proof source registry artifact.
- [ ] Gate route/district/source metadata completeness.
- [ ] Reject registry rows that cite terminal seed data as contact proof.
- [ ] Keep missing source rows as blockers.

## Expected Gates

- `route t4-terminal-contact-proof-artifact-contract --gate`
- terminal-contact source registry gate if added
- `route t4-terminal-contact-source-plan --gate`
- `cargo test -p route`

## Non-Goals

- Do not fetch remote pages.
- Do not propagate proof decisions yet.

