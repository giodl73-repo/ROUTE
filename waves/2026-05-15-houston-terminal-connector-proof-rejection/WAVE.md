---
wave: houston-terminal-connector-proof-rejection
date_open: 2026-05-15
status: done
---

# Houston Terminal Connector Proof Rejection

## Mission

Continue T4 terminal-access blocker burn-down by resolving the Houston
Englewood / US96 held pairing against official UP terminal and FHWA intermodal
connector evidence.

## Opening Rule

Reject the pairing only if public sources identify the Houston UP intermodal
terminal and an official connector route that excludes US96. Do not infer from
metro proximity or seed terminal assignment.

## Inputs Inherited

- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/t4-terminal-contact-evidence.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Reject Houston unsupported pairing | done | One UP/FHWA-backed negative proof row and optimizer replay |

## Done Criteria

- Houston rejection cites Union Pacific's Houston intermodal terminal page.
- Houston rejection cites the FHWA Texas connector listing for U.P. Settegast Yard via Kirkpatrick Boulevard to I-610.
- T4 terminal-access residual blockers decrease from 9 to 8.

## Non-goals

- Do not accept route-to-terminal contact proof.
- Do not reject New York Fresh Pond or remaining New Orleans Gentilly rows without more precise direct-access evidence.
- Do not promote T1 source evidence or T2 repair-debt claims.
