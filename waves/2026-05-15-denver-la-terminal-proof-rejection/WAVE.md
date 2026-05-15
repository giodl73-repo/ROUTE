---
wave: denver-la-terminal-proof-rejection
date_open: 2026-05-15
status: done
---

# Denver LA Terminal Proof Rejection

## Mission

Close two remaining singleton T4 terminal-access blockers using official public
sources for direct terminal access routes.

## Opening Rule

Use only sources that identify a direct terminal/logistics access route. Do not
reject other singleton rows from geographic proximity or generic freight-market
descriptions.

## Inputs Inherited

- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/t4-terminal-contact-evidence.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Reject Denver and LA unsupported pairings | done | Two additional source-listed negative proof rows and optimizer replay |

## Done Criteria

- Denver Logistics Hub rejection cites BNSF Logistics Center Hudson I-76 access.
- Los Angeles/Long Beach rejection cites the Caltrans Route 710 port access corridor.
- T4 terminal-access residual blockers decrease from 28 to 26.

## Non-goals

- Do not accept route-to-terminal contact proof.
- Do not reject Salt Lake, Portland, Miami, Houston, Atlanta, Memphis, New Orleans, New York, or Philadelphia rows without precise direct-access evidence.
- Do not promote T1 source evidence or T2 repair-debt claims.
