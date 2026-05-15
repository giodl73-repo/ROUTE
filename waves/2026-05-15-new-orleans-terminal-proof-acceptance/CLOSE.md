---
wave: new-orleans-terminal-proof-acceptance
date_closed: 2026-05-15
status: done
---

# Close: New Orleans Terminal Proof Acceptance

## Result

Accepted two additional route-terminal contact proof rows:

- I-510 / New Orleans Gentilly
- US90Z / New Orleans Gentilly

Both cite Port NOLA's official road/truck directions rather than the terminal
seed assignment.

## Optimizer Effect

- T4 terminal-access upgrade blockers decreased from 8 to 6.
- Total claim blockers decreased from 9 to 7.
- Remaining T4 terminal-access rows are the six New York Fresh Pond rows.
- T2 asset-condition debt remains repair-only at six budget-debt rows / $75.0M.

## Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- t4-terminal-contact-proof-source-registry --gate`
- `cargo run -q -p route -- t4-terminal-contact-district-proof-import --gate`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- map-publication-readiness --gate`
- `cargo run -q -p route -- map-publication-inventory --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-new-orleans-terminal-proof-acceptance`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
