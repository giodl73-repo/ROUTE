---
wave: terminal-contact-source-acquisition-spine
pulse: 01
date: 2026-05-13
status: done
depends_on: []
governing_roles:
  - citation-auditor
  - scope-keeper
  - optimization-methodologist
---

# Pulse 01 - Proof Artifact Contract

## Mission

Define a gateable contract for manual or cached route-to-terminal contact proof
artifacts before any source row can be accepted.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Source policy | `docs/source-fetch-cache-policy.md` | Add terminal-contact manual/cache proof rules. |
| Terminal doctrine | `docs/t3-t4-access-optimization.md` | Name proof artifact contract and promotion rule. |
| Proof artifacts | none | `data/t4-terminal-contact-proof-artifact-contract.csv` with required fields and gate. |

## Deliverables

- [x] Add a proof artifact contract artifact with required fields.
- [x] Add a CLI gate that rejects source-backed status without non-seed proof.
- [x] Document manual/cached proof rules in source and T3/T4 doctrine.
- [x] Register the contract in spec index and manifests if needed.

## Expected Gates

- `route t4-terminal-contact-proof-artifact-contract --gate`
- `route t4-terminal-contact-source-plan --gate`
- `route t4-terminal-columbus-proof-attempts --gate`
- `cargo test -p route`
- targeted `proof check`

## Non-Goals

- Do not import live source evidence.
- Do not promote any Great Lakes or Columbus row.

