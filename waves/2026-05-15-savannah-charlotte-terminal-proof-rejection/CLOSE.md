---
wave: savannah-charlotte-terminal-proof-rejection
date_closed: 2026-05-15
status: done
---

# Close: Savannah Charlotte Terminal Proof Rejection

## Result

Recorded four additional route-terminal pairings whose assigned terminal source
lists a different direct interstate access set than the held route:

- I-140 / Savannah Garden City
- US301 / Savannah Garden City
- I-795 / Charlotte Intermodal
- US119 / Charlotte Intermodal

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from 42 to 38.
- Total claim blockers decreased from 43 to 39.
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
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-savannah-charlotte-terminal-proof-rejection`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
