---
wave: indianapolis-avon-terminal-proof-acceptance
date_open: 2026-05-15
status: done
---

# Indianapolis Avon Terminal Proof Acceptance

## Mission

Accept one narrow non-seed T4 terminal-contact proof source for I-465 /
Indianapolis Avon and wire accepted proof into the optimizer so a real
terminal-access blocker is removed.

## Opening Rule

Only accept route-specific source proof where the public source names both the
terminal district and the interstate contact. District-only or proximity-only
sources remain held.

## Inputs Inherited

- `data/t4-terminal-contact-proof-docket.csv`
- `data/t4-terminal-contact-proof-source-registry.csv`
- `data/t4-terminal-contact-district-proof-import.csv`
- `data/t3-t4-access-gaps.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Indianapolis Avon terminal proof acceptance | done | `data/t4-terminal-contact-accepted-proof-sources.csv`; source-backed I-465 registry/import row; optimizer blocker reduction |

## Done Criteria

- A non-seed source-backed proof row exists for `T4CONTACT-T3GREATLAKES-I465`.
- The district proof import accepts that row while leaving unresolved Columbus
  South rows source-needed.
- The optimizer ledger no longer emits the I-465 terminal-access evidence gap.
- T4 terminal-access residual blockers decrease from 69 to 68.

## Non-goals

- Do not accept district-only proof for Columbus South.
- Do not clear terminal-access rows that lack route-specific source proof.
- Do not change map publication scope.
