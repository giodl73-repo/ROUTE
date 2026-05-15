---
wave: tx-i220-pavement-route-state-exclusion
date_closed: 2026-05-15
status: done
---

# Close: TX I-220 Pavement Route-State Exclusion

## Result

Added `data/tier-pavement-route-state-exclusions.csv` with one accepted
route-state exclusion:

- TX / I-220 / `US.HWYBUNDLE.5F57C12B12BEE8A8`

The exclusion cites FHWA's Interstate Route Log and Finders List, Table 2, which
lists I-220 in Louisiana and Mississippi but not Texas. The pavement debt
generator now reads that exclusion before pricing repair debt, and the repair
debt review applies the same excluded member count so the downstream funding
chain no longer emits a missing TX repair-review row.

## Optimizer Effect

- T2 asset-condition budget-debt rows decreased from six to five.
- T2 asset-condition repair debt decreased from $75.0M to $65.0M.
- The excluded TX / I-220 bundle is removed from residual asset-condition debt.
- Remaining T2 repair debt is LA I-220 plus CA, AL, NM, and LA I-110.

## Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-debt-budget --gate`
- `cargo run -q -p route -- tier-pavement-repair-debt-review --gate`
- `cargo run -q -p route -- tier-pavement-repair-disposition --gate`
- `cargo run -q -p route -- tier-pavement-repair-funding-package --gate`
- `cargo run -q -p route -- tier-pavement-funding-commitment-review --gate`
- `cargo run -q -p route -- tier-pavement-downgrade-exclusion-decision --gate`
- `cargo run -q -p route -- tier-pavement-funding-evidence-contract --gate`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- map-publication-readiness --gate`
- `cargo run -q -p route -- map-publication-inventory --gate`
- `cargo run -q -p route -- release-manifest --gate`
