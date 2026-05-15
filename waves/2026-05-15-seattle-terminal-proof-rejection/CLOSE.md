---
wave: seattle-terminal-proof-rejection
date_closed: 2026-05-15
status: done
---

# Close: Seattle Terminal Proof Rejection

## Result

Recorded one additional route-terminal pairing whose assigned terminal source
lists a different direct interstate access set than the held route:

- I-705 / Seattle BNSF

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from 38 to 37.
- Total claim blockers decreased from 39 to 38.
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
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-seattle-terminal-proof-rejection`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
