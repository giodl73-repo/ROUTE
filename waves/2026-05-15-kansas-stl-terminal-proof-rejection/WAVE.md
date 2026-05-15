---
wave: kansas-stl-terminal-proof-rejection
date_open: 2026-05-15
status: done
---

# Kansas St. Louis Terminal Proof Rejection

## Mission

Continue T4 terminal-access blocker burn-down using public terminal sources for
Kansas City Gateway and the remaining St. Louis Gateway row.

## Opening Rule

Reject held routes only when a public terminal source lists direct access routes
and the held route is absent. Leave broader metro terminal rows held when the
terminal-to-source mapping is not precise.

## Inputs Inherited

- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/t4-terminal-contact-evidence.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Reject Kansas City and St. Louis unsupported pairings | done | Seven additional source-listed negative proof rows and optimizer replay |

## Done Criteria

- Kansas City Gateway rejections cite BNSF Logistics Park Kansas City I-35 access.
- St. Louis Gateway rejection cites the existing St. Louis terminal access-source set.
- T4 terminal-access residual blockers decrease from 35 to 28.

## Non-goals

- Do not accept route-to-terminal contact proof.
- Do not reject New York, Memphis, New Orleans, Atlanta, or Philadelphia rows without precise direct-access evidence.
- Do not promote T1 source evidence or T2 repair-debt claims.
