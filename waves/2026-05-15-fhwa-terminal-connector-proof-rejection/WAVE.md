---
wave: fhwa-terminal-connector-proof-rejection
date_open: 2026-05-15
status: done
---

# FHWA Terminal Connector Proof Rejection

## Mission

Continue T4 terminal-access blocker burn-down using FHWA NHS intermodal
connector listings where they name direct truck/rail connector routes and the
held route is absent.

## Opening Rule

Reject held routes only when a public connector listing names direct
terminal-access routes for the terminal district or a matching truck/rail
facility. Do not use seed terminal assignment or generic metro proximity as
proof.

## Inputs Inherited

- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/t4-terminal-contact-evidence.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Reject FHWA connector-contradicted pairings | done | Eleven FHWA-backed negative proof rows and optimizer replay |

## Done Criteria

- Memphis rejections cite FHWA Tennessee truck/rail connector listings.
- Salt Lake City rejections cite FHWA Utah Salt Lake City Intermodal Terminal connector listing.
- Portland Albina rejection cites FHWA Oregon Albina Yards connector listing.
- Miami Hialeah rejection cites FHWA Florida Miami truck/rail connector listings.
- New Orleans Gentilly rejections cite FHWA Louisiana CSX New Orleans truck/rail connector listing.
- T4 terminal-access residual blockers decrease from 20 to 9.

## Non-goals

- Do not accept route-to-terminal contact proof.
- Do not reject New York Fresh Pond, Houston Englewood, New Orleans I-510, or New Orleans US90Z rows without more precise direct-access evidence.
- Do not promote T1 source evidence or T2 repair-debt claims.
