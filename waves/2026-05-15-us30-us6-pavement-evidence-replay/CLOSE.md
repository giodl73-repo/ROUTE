---
wave: us30-us6-pavement-evidence-replay
date_closed: 2026-05-15
status: done
---

# Close: US30 US6 Pavement Evidence Replay

## Result

Fetched and replayed HPMS pavement evidence for the US30 evidence-state set. The
replay also cleared the remaining US6 source-needed pavement evidence row through
the refreshed HPMS join. Both evidence-debt rows are now absent from the pavement
debt budget.

Remaining T2 asset-condition debt is repair-only:

- I220 / TX
- I220 / LA
- I110 / CA
- I110 / AL
- I110 / NM
- I110 / LA

## Optimizer Effect

- T2 asset-condition budget-debt rows decreased from 8 to 6.
- T2 asset-condition debt cost decreased from $86.8M to $75.0M.
- Total optimizer rows decreased from 97 to 94.
- Total claim blockers remain 36: 35 T4 terminal-access upgrade blockers and 1 source snapshot evidence blocker.

## Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- build --all-roads`
- `cargo run -q -p route -- tier-pavement-docket --gate`
- `cargo run -q -p route -- tier-pavement-source-gaps --gate`
- `cargo run -q -p route -- tier-pavement-debt-budget --gate`
- `cargo run -q -p route -- tier-pavement-acquisition-plan --gate`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- map-publication-readiness --gate`
- `cargo run -q -p route -- map-publication-inventory --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-us30-us6-pavement-evidence-replay`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
