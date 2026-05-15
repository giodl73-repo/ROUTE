---
wave: us2-pavement-evidence-replay
date_closed: 2026-05-15
status: done
---

# Close: US2 Pavement Evidence Replay

## Result

Refreshed HPMS pavement evidence for the US2 evidence-debt slice and replayed
the pavement/optimizer artifacts. The US2 evidence-debt row is no longer present
in the pavement debt budget.

The Washington HPMS fetch reported a parse failure during the multi-state fetch,
but the generated pavement docket no longer emits a US2 source-needed or repair
debt row after the replay. Remaining pavement debt is still held.

## Optimizer Effect

- T2 asset-condition budget-debt rows decreased from 9 to 8.
- T2 asset-condition debt cost decreased from $87.2M to $86.8M.
- Total optimizer rows decreased from 99 to 97.
- Total claim blockers remain 36: 35 T4 terminal-access upgrade blockers and 1 source snapshot evidence blocker.

## Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- build --all-roads`
- `cargo run -q -p route -- tier-pavement-docket --gate`
- `cargo run -q -p route -- tier-pavement-source-gaps --gate`
- `cargo run -q -p route -- tier-pavement-debt-budget --gate`
- `cargo run -q -p route -- tier-pavement-acquisition-plan --gate`
- `cargo run -q -p route -- tier-pavement-source-fetch-attempt --gate`
- `cargo run -q -p route -- tier-pavement-source-fetch-review --gate`
- `cargo run -q -p route -- tier-pavement-unmatched-join-review --gate`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- map-publication-readiness --gate`
- `cargo run -q -p route -- map-publication-inventory --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-us2-pavement-evidence-replay`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
