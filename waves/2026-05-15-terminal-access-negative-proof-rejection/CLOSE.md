---
wave: terminal-access-negative-proof-rejection
date_closed: 2026-05-15
status: done
---

# Close: Terminal Access Negative Proof Rejection

## Result

Recorded five route-terminal pairings whose assigned terminal source lists a
different access set than the held route:

- I-129 / Indianapolis Avon
- US31 / Indianapolis Avon
- I-294 / Chicago Intermodal Complex
- US41 / Chicago Intermodal Complex
- US10 / Detroit Livernois

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from 68 to 63.
- Total claim blockers decreased from 69 to 64.
- Remaining T4 terminal-access rows stay held for route-specific proof.

## Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route optimizer-residual-blocker-backlog --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route map-publication-readiness --gate`
- `route map-publication-inventory --gate`
- `route release-manifest --gate`
