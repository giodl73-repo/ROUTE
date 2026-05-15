---
wave: columbus-louisville-terminal-proof-rejection
date_open: 2026-05-15
status: done
---

# Columbus Louisville Terminal Proof Rejection

## Mission

Burn down the next defensible T4 terminal-access blocker tranche using only
public terminal sources that list direct interstate access.

## Opening Rule

Use negative proof only where the terminal seed maps to a specific public source
and the held route is absent from that source's listed interstate access. Leave
broader gateway seeds held when source-to-seed mapping is ambiguous.

## Inputs Inherited

- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/t4-terminal-contact-evidence.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Reject Columbus and Louisville unsupported pairings | done | Eleven additional source-listed negative proof rows and optimizer replay |

## Done Criteria

- Columbus South rejections cite CSX Columbus direct I-270 access.
- Louisville KentuckyOne rejections cite CSX Louisville I-65/I-264 access and co-location with UPS/airport.
- Kansas City, Memphis, New Orleans, and other broad gateway rows remain held unless source mapping is precise.
- T4 terminal-access residual blockers decrease from 53 to 42.

## Non-goals

- Do not accept route-to-terminal contact proof.
- Do not use broad gateway-region descriptions as negative proof.
- Do not promote T1 source evidence or T2 asset-condition claims.
