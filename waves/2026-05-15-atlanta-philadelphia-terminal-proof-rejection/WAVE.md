---
wave: atlanta-philadelphia-terminal-proof-rejection
date_open: 2026-05-15
status: done
---

# Atlanta Philadelphia Terminal Proof Rejection

## Mission

Continue T4 terminal-access blocker burn-down using public terminal-access
sources for Atlanta Hulsey and Philadelphia Frankford rows.

## Opening Rule

Reject held routes only when a public source names direct terminal or terminal
facility access routes and the held route is absent. Do not infer access from
metro proximity alone.

## Inputs Inherited

- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/t4-terminal-contact-evidence.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Reject Atlanta and Philadelphia unsupported pairings | done | Six additional source-listed negative proof rows and optimizer replay |

## Done Criteria

- Atlanta Hulsey rejections cite the USDOT/Georgia intermodal connector source listing I-20/Boulevard SE access.
- Philadelphia Frankford rejections cite PhilaPort terminal truck access via I-95/I-76.
- T4 terminal-access residual blockers decrease from 26 to 20.

## Non-goals

- Do not accept route-to-terminal contact proof.
- Do not reject New York, Memphis, New Orleans, Salt Lake, Portland, Miami, or Houston rows without precise direct-access evidence.
- Do not promote T1 source evidence or T2 repair-debt claims.
