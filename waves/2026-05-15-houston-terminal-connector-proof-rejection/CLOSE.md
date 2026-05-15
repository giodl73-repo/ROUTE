---
wave: houston-terminal-connector-proof-rejection
date_closed: 2026-05-15
status: done
---

# Close: Houston Terminal Connector Proof Rejection

## Result

Recorded one additional route-terminal pairing whose official terminal and FHWA
connector sources list a different access set than the held route:

- US96 / Houston Englewood

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from 9 to 8.
- Total claim blockers decreased from 10 to 9.
- T2 asset-condition debt remains repair-only at six budget-debt rows / $75.0M.
- Remaining T4 terminal-access rows stay held for route-specific proof.

## Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- map-publication-readiness --gate`
- `cargo run -q -p route -- map-publication-inventory --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-houston-terminal-connector-proof-rejection`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
