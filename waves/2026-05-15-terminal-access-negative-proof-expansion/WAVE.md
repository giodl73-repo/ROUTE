---
wave: terminal-access-negative-proof-expansion
date_open: 2026-05-15
status: done
---

# Terminal Access Negative Proof Expansion

## Mission

Continue T4 terminal-access blocker burn-down by rejecting only route-terminal
pairings contradicted by source-listed terminal access routes.

## Opening Rule

Negative proof may remove an upgrade blocker only when a public terminal source
lists access routes and the held route is absent from that set. Broad district
membership, nearby geography, and seed terminal assignment are not proof.

## Inputs Inherited

- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/t4-terminal-contact-evidence.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Expand terminal-access negative proof | done | Ten additional rejected route-terminal pairings and optimizer replay |

## Done Criteria

- Rejected rows cite non-seed public terminal access sources.
- Rejected rows are limited to held routes absent from the source-listed access set.
- Optimizer ledger omits rejected route-terminal upgrade blockers.
- T4 terminal-access residual blockers decrease from 63 to 53.

## Non-goals

- Do not accept route-to-terminal contact proof.
- Do not reject rows where the terminal source match is ambiguous.
- Do not promote T1 source evidence or T2 asset-condition claims.
