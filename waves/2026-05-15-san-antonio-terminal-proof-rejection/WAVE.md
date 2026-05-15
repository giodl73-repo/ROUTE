---
wave: san-antonio-terminal-proof-rejection
date_open: 2026-05-15
status: done
---

# San Antonio Terminal Proof Rejection

## Mission

Close the San Antonio Kirby T4 terminal-access blockers using Union Pacific's
source-listed access for the adjacent San Antonio Intermodal Terminal.

## Opening Rule

Reject only rows contradicted by the terminal operator's direct access statement.
Do not infer from citywide freight geography or nearby interstate corridors.

## Inputs Inherited

- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/t4-terminal-contact-evidence.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Reject San Antonio unsupported pairings | done | Two additional source-listed negative proof rows and optimizer replay |

## Done Criteria

- San Antonio Kirby rejections cite Union Pacific's San Antonio Logistics Park / SAIT I-35 corridor access.
- T4 terminal-access residual blockers decrease from 37 to 35.

## Non-goals

- Do not accept route-to-terminal contact proof.
- Do not reject Houston, Miami, Denver, Portland, Los Angeles/Long Beach, or Salt Lake rows without precise direct-access evidence.
- Do not promote T1 source evidence or T2 asset-condition claims.
