---
wave: t4-terminal-access-map-exclusion
date_closed: 2026-05-15
status: done
---

# T4 Terminal Access Map Exclusion Close

## Decision

The unresolved T4 terminal-access evidence gap is now scoped out of current map
publication claims. The rows still exist in the optimizer ledger and still block
`upgrade`, but no longer block `map` or `publication`.

## Evidence

| Rows | Claims before | Claims after | Proof accepted |
|---:|---|---|---|
| 69 | upgrade;map;publication | upgrade | no |

## Residual Holds

- Source snapshot guard still blocks `evidence` and `publication`.
- T2 asset-condition debt remains publication-relevant debt.
- T4 terminal-access proof remains required for upgrade/evidence claims.

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `cargo run -q -p route -- map-atlas --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md docs\map-publication-scope.md waves\PHASES.md waves\2026-05-15-t4-terminal-access-map-exclusion`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Clear the source snapshot guard and T2 asset-condition publication debt, or keep
full T1-T4 publication blocked.
