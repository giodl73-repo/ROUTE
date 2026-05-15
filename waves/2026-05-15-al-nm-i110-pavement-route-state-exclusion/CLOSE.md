---
wave: al-nm-i110-pavement-route-state-exclusion
date_closed: 2026-05-15
status: done
---

# Close: AL/NM I-110 Pavement Route-State Exclusion

## Result

Added two accepted route-state exclusions to
`data/tier-pavement-route-state-exclusions.csv`:

- AL / I-110 / `US.HWYBUNDLE.D6B6E222CB0B708B`
- NM / I-110 / `US.HWYBUNDLE.D6BA4122CB0E47CF`

Both cite FHWA's Interstate Route Log and Finders List, Table 2. The route log
lists I-110 in California, Texas, Louisiana, Mississippi, and Florida, but not
Alabama or New Mexico. The two excluded pavement bundles are therefore
route-state scope artifacts rather than repair obligations.

## Optimizer Effect

- T2 asset-condition budget-debt rows decreased from five to three.
- T2 asset-condition repair debt decreased from $65.0M to $55.0M.
- Remaining T2 repair debt is LA I-220, CA I-110, and LA I-110.

## Gates

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
