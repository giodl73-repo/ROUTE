---
wave: terminal-access-negative-proof-expansion
date_closed: 2026-05-15
status: done
---

# Close: Terminal Access Negative Proof Expansion

## Result

Recorded ten additional route-terminal pairings whose assigned terminal source
lists a different terminal access set than the held route:

- I-115 / Chicago Intermodal Complex
- I-176 / Chicago Intermodal Complex
- I-196 / Detroit Livernois
- I-496 / Detroit Livernois
- I-696 / Detroit Livernois
- US223 / Detroit Livernois
- I-180 / St. Louis Gateway
- I-72 / St. Louis Gateway
- US42 / St. Louis Gateway
- I-235 / Minneapolis Twin Cities

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from 63 to 53.
- Total claim blockers decreased from 64 to 54.
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
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-terminal-access-negative-proof-expansion`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
