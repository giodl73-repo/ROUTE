---
wave: map-publication-inventory-gate
date_closed: 2026-05-15
status: done
---

# Map Publication Inventory Gate Close

## Decision

The structural map publication inventory is now gateable. The gate compares
`data/map-publication-inventory.csv` against `data/map-atlas.csv` and
`data/map-publication-readiness.csv`.

## Evidence

| Command | Result |
|---|---|
| `cargo run -q -p route -- map-publication-inventory --gate` | pass |

## Residual Holds

- Source snapshot repeat-window or archive-history proof remains required before
  evidence claims.
- T4 terminal-access proof remains required for upgrade/evidence claims.
- T2 asset-condition debt remains payable before SLA, transit, or upgrade claims.

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- map-publication-readiness --gate`
- `cargo run -q -p route -- map-publication-inventory --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `cargo run -q -p route -- map-atlas --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md docs\map-publication-scope.md waves\PHASES.md waves\2026-05-15-map-publication-inventory-gate`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Use `data/map-publication-inventory.csv` as the exact current publishable map
set, guarded by `route map-publication-inventory --gate`.
