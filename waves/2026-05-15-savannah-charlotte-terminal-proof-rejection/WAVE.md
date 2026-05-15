---
wave: savannah-charlotte-terminal-proof-rejection
date_open: 2026-05-15
status: done
---

# Savannah Charlotte Terminal Proof Rejection

## Mission

Continue T4 terminal-access blocker burn-down with source-specific public access
routes for Savannah Garden City and Charlotte Intermodal rows.

## Opening Rule

Reject a held route only when the terminal operator or public port source lists a
different direct interstate access set. Do not use general regional freight
importance as negative proof.

## Inputs Inherited

- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/t4-terminal-contact-evidence.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Reject Savannah and Charlotte unsupported pairings | done | Four additional source-listed negative proof rows and optimizer replay |

## Done Criteria

- Savannah Garden City rejections cite Georgia Ports direct I-95/I-516/I-16 truck access.
- Charlotte Intermodal rejections cite North Carolina Ports I-85/I-77 corridor access.
- T4 terminal-access residual blockers decrease from 42 to 38.

## Non-goals

- Do not accept route-to-terminal contact proof.
- Do not reject Atlanta, Kansas City, Memphis, New Orleans, or New York rows without precise access evidence.
- Do not promote T1 source evidence or T2 asset-condition claims.
