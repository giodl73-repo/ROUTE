---
wave: fresh-pond-truck-route-rejection
date_closed: 2026-05-15
status: done
---

# Close: Fresh Pond Truck Route Rejection

## Result

Added six rejected terminal-contact proof rows for New York Fresh Pond:

- I-190
- I-390
- I-478
- I-691
- I-990
- US7

The rejection source is NYC DOT's official New York City Truck Routes dataset
and truck-routing doctrine. A direct query of the dataset returns Fresh Pond
Road and Metropolitan Avenue as Queens local truck routes, and Long Island
Expressway and Brooklyn Queens Expressway as Queens through truck routes. The
six held routes are not supported by that Fresh Pond-area terminal-access
source.

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from six to zero.
- The residual blocker backlog no longer contains a T4 terminal-access family.
- Remaining residual holds are T2 asset-condition repair debt and the T1 source
  snapshot evidence guard.

## Gates

- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- map-publication-readiness --gate`
- `cargo run -q -p route -- map-publication-inventory --gate`
- `cargo run -q -p route -- release-manifest --gate`
