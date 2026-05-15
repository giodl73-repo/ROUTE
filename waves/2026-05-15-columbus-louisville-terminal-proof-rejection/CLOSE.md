---
wave: columbus-louisville-terminal-proof-rejection
date_closed: 2026-05-15
status: done
---

# Close: Columbus Louisville Terminal Proof Rejection

## Result

Recorded eleven additional route-terminal pairings whose assigned terminal source
lists a different direct interstate access set than the held route:

- I-271 / Columbus South
- I-279 / Columbus South
- I-471 / Columbus South
- US22 / Columbus South
- US224 / Columbus South
- US250 / Columbus South
- US35 / Columbus South
- US74 / Columbus South
- I-181 / Louisville KentuckyOne
- I-277 / Louisville KentuckyOne
- US421 / Louisville KentuckyOne

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from 53 to 42.
- Total claim blockers decreased from 54 to 43.
- T2 asset-condition debt remains unresolved at nine budget-debt rows.
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
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-columbus-louisville-terminal-proof-rejection`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
