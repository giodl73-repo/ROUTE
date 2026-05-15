---
wave: 2026-05-15-la-i110-repair-funding-acceptance
date_closed: 2026-05-15
status: done
---

# Close: LA I-110 Repair Funding Acceptance

## Result

Louisiana DOTD STIP project H.010319 is now a governed full-cost repair funding
acceptance for LA / I-110 / `US.HWYBUNDLE.D6C11122CB1414ED`. The accepted
coverage removes that row from priced T2 asset-condition debt.

## Residual State

- T2 asset-condition repair debt: 2 rows / $30.0M.
- Remaining repair holds: LA I-220 / $25.0M and CA I-110 / $5.0M.
- T1 source snapshot evidence guard remains held.

## Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-debt-budget --gate`
- `cargo run -q -p route -- tier-pavement-repair-debt-review --gate`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
