---
wave: atlanta-philadelphia-terminal-proof-rejection
date_closed: 2026-05-15
status: done
---

# Close: Atlanta Philadelphia Terminal Proof Rejection

## Result

Recorded six additional route-terminal pairings whose assigned terminal source
lists a different direct access set than the held route:

- I-185 / Atlanta Hulsey
- US278 / Atlanta Hulsey
- US84 / Atlanta Hulsey
- I-276 / Philadelphia Frankford
- I-93 / Philadelphia Frankford
- US15 / Philadelphia Frankford

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from 26 to 20.
- Total claim blockers decreased from 27 to 21.
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
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-atlanta-philadelphia-terminal-proof-rejection`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
