---
wave: map-publication-scope-decision
date_closed: 2026-05-15
status: done
---

# Map Publication Scope Decision Close

## Decision

ROUTE now treats current maps as render-valid but not fully publication-valid.
Full T1-T4 map publication remains blocked until real evidence relief or an
explicit downgrade/exclusion decision removes the residual optimizer blockers.

## Evidence

| Surface | Render gate | Evidence/publication gate | Status |
|---|---|---|---|
| Map atlas | pass | blocked | render-valid only |
| Beck T1 diagnostics | pass | scoped | structural WIP |
| Beck T2 diagnostics | pass | scoped | structural WIP |
| T3 zone diagnostics | pass | scoped | structural WIP |
| Full T1-T4 publication | pass | blocked | not publication-valid |

## Residual Holds

- 69 T4 terminal-access evidence gaps.
- 1 source snapshot guard.
- 9 T2 asset-condition debt rows.

## Gate Bundle

- `cargo run -q -p route -- map-atlas --gate`
- `cargo run -q -p route -- beck-t1-diagnostics --gate`
- `cargo run -q -p route -- beck-t2-diagnostics --gate`
- `cargo run -q -p route -- t3-zone-map-diagnostics --gate`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md docs\map-publication-scope.md waves\PHASES.md waves\2026-05-15-map-publication-scope-decision`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Do not add another source-needed placeholder artifact. The next pulse must either
attach accepted non-seed evidence, explicitly downgrade/exclude unresolved rows,
or leave full T1-T4 publication blocked.
