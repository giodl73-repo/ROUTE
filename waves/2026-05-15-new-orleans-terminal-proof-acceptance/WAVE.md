---
wave: new-orleans-terminal-proof-acceptance
date_open: 2026-05-15
status: done
---

# New Orleans Terminal Proof Acceptance

## Mission

Use the generalized positive proof docket to accept only route-specific New
Orleans terminal-contact proof that is backed by Port NOLA's official truck
directions.

## Opening Rule

Accept proof only where the public source names the route and terminal access
path. Do not treat the New Orleans terminal seed row, generic metro proximity,
or unofficial terminal directories as proof.

## Inputs Inherited

- `data/t4-terminal-contact-accepted-proof-sources.csv`
- `data/t4-terminal-contact-proof-source-registry.csv`
- `data/t4-terminal-contact-district-proof-import.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Accept Port NOLA route proof | done | Two Port NOLA-backed accepted proof rows and optimizer replay |

## Done Criteria

- I-510 proof cites Port NOLA truck directions using I-510 South for terminal access.
- US90Z proof cites Port NOLA truck directions using Business 90 / U.S. 90 terminal access.
- Proof source registry marks three total source-backed rows, preserving source-needed status for unresolved rows.
- T4 terminal-access residual blockers decrease from 8 to 6.

## Non-goals

- Do not accept New York Fresh Pond proof without a source that names the route-to-terminal access path.
- Do not promote T1 source evidence or T2 repair-debt claims.
- Do not use seed terminal assignment as proof.
