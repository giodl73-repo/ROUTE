---
wave: terminal-contact-proof-docket-generalization
date_closed: 2026-05-15
status: done
---

# Close: Terminal Contact Proof Docket Generalization

## Result

Generalized the T4 terminal-contact positive proof source plan from the original
Great-Lakes-only slice to all source-needed rows. The generated proof surface now
contains:

- 69 terminal-contact source plan rows.
- 23 terminal-district source catalog rows.
- 69 terminal-contact proof docket rows.
- 69 proof source registry rows: one source-backed row and 68 source-needed rows.
- 9 district proof import rows: one accepted I-465 row and eight source-needed Columbus South rows.

## Optimizer Effect

- No T4 blocker relief was taken in this wave.
- Residual T4 terminal-access blockers remain at 8.
- The positive-proof path is now available to all districts when route-specific
  non-seed proof is strong enough.

## Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- t4-terminal-contact-source-plan --gate`
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
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-terminal-contact-proof-docket-generalization`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
