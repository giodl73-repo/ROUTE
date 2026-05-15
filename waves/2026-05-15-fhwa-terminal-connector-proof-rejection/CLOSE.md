---
wave: fhwa-terminal-connector-proof-rejection
date_closed: 2026-05-15
status: done
---

# Close: FHWA Terminal Connector Proof Rejection

## Result

Recorded eleven additional route-terminal pairings whose FHWA NHS intermodal
connector source lists a different direct access set than the held route:

- I-759 / Memphis Intermodal
- I-840 / Memphis Intermodal
- US167 / Memphis Intermodal
- US270 / Memphis Intermodal
- US14 / Salt Lake City
- US95 / Salt Lake City
- US26 / Portland Albina
- I-175 / Miami Hialeah
- US45E / New Orleans Gentilly
- US45W / New Orleans Gentilly
- US82 / New Orleans Gentilly

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from 20 to 9.
- Total claim blockers decreased from 21 to 10.
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
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-fhwa-terminal-connector-proof-rejection`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
