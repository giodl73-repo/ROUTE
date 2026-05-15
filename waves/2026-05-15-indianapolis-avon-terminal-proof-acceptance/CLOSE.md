---
wave: indianapolis-avon-terminal-proof-acceptance
date_closed: 2026-05-15
status: done
---

# Close: Indianapolis Avon Terminal Proof Acceptance

## Result

Accepted a route-specific non-seed public source for `I-465` / `Indianapolis
Avon` terminal contact. The accepted proof is recorded in
`data/t4-terminal-contact-accepted-proof-sources.csv`, replayed into
`data/t4-terminal-contact-proof-source-registry.csv`, imported through
`data/t4-terminal-contact-district-proof-import.csv`, and consumed by
`optimizer-constraint-ledger`.

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from 69 to 68.
- Total claim blockers decreased from 70 to 69.
- Columbus South rows remain `source-needed`; the MAFC Columbus district page
  is not enough route-specific proof for those rows.

## Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `route t4-terminal-contact-proof-source-registry --gate`
- `route t4-terminal-contact-district-proof-import --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route optimizer-residual-blocker-backlog --gate`
