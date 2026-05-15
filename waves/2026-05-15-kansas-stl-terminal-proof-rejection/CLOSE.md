---
wave: kansas-stl-terminal-proof-rejection
date_closed: 2026-05-15
status: done
---

# Close: Kansas St. Louis Terminal Proof Rejection

## Result

Recorded seven additional route-terminal pairings whose assigned terminal source
lists a different direct interstate access set than the held route:

- I-169 / Kansas City Gateway
- US24 / Kansas City Gateway
- US66 / Kansas City Gateway
- I-135 / Kansas City Gateway
- I-335 / Kansas City Gateway
- US76 / Kansas City Gateway
- I-255 / St. Louis Gateway

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from 35 to 28.
- Total claim blockers decreased from 36 to 29.
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
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-kansas-stl-terminal-proof-rejection`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
