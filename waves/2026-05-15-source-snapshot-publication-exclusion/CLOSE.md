---
wave: source-snapshot-publication-exclusion
date_closed: 2026-05-15
status: done
---

# Source Snapshot Publication Exclusion Close

## Decision

The live source snapshot guard is now scoped out of current map publication
claims. The row still exists in the optimizer ledger and still blocks
`evidence`, but no longer blocks `publication`.

## Evidence

| Rows | Claims before | Claims after | Evidence accepted |
|---:|---|---|---|
| 1 | evidence;publication | evidence | no |

## Residual Holds

- T2 asset-condition debt remains publication-relevant debt.
- Source snapshot repeat-window or archive-history proof remains required before
  evidence claims.
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
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md docs\map-publication-scope.md waves\PHASES.md waves\2026-05-15-source-snapshot-publication-exclusion`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Resolve or explicitly scope T2 asset-condition publication debt, or keep full
T1-T4 publication blocked.
