---
wave: seattle-terminal-proof-rejection
date_open: 2026-05-15
status: done
---

# Seattle Terminal Proof Rejection

## Mission

Close one additional T4 terminal-access blocker using a public source that names
the Seattle International Gateway rail yard and its interstate access.

## Opening Rule

Reject only the held route contradicted by a source-listed access pair. Leave
other western terminal rows held unless their terminal source names direct access
routes.

## Inputs Inherited

- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/t4-terminal-contact-evidence.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Reject Seattle unsupported pairing | done | One additional source-listed negative proof row and optimizer replay |

## Done Criteria

- Seattle BNSF rejection cites Port of Seattle SR 519 access to I-5/I-90 and Seattle International Gateway Rail Yard.
- T4 terminal-access residual blockers decrease from 38 to 37.

## Non-goals

- Do not infer access for Portland, Denver, Los Angeles/Long Beach, or Salt Lake rows from regional proximity alone.
- Do not accept route-to-terminal contact proof.
- Do not promote T1 source evidence or T2 asset-condition claims.
