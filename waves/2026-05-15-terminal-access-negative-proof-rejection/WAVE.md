---
wave: terminal-access-negative-proof-rejection
date_open: 2026-05-15
status: done
---

# Terminal Access Negative Proof Rejection

## Mission

Use source-listed terminal access routes to reject held T4 route-terminal
pairings that are contradicted by public terminal access evidence.

## Opening Rule

Negative proof may remove an upgrade blocker only when the source lists terminal
access routes and the held route is absent from that access set. Do not treat
district membership, nearby corridors, or generalized freight-region relevance
as proof.

## Inputs Inherited

- `data/t4-terminal-contact-proof-source-registry.csv`
- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/t3-t4-access-gaps.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Terminal access negative proof rejection | done | `data/t4-terminal-contact-rejected-proof-sources.csv`; optimizer blocker reduction |

## Done Criteria

- Rejected rows cite source-listed terminal access routes.
- Rejected rows do not cite `data/intermodal_terminals.csv` as proof.
- Optimizer ledger omits rejected route-terminal upgrade blockers.
- T4 terminal-access residual blockers decrease from 68 to 63.

## Non-goals

- Do not accept route-to-terminal contact proof.
- Do not reject rows where the source does not list terminal access routes.
- Do not change map publication scope.
