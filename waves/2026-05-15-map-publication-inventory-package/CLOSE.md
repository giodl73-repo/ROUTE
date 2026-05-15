---
wave: map-publication-inventory-package
date_closed: 2026-05-15
status: done
---

# Map Publication Inventory Package Close

## Decision

The current structural T1-T4 map set is now packaged as a 17-row publication
inventory. Each map is publishable only with the held-claim label defined by
`docs/map-publication-scope.md`.

## Evidence

| Artifact | Rows | Publication status | Held claims |
|---|---:|---|---|
| `data/map-publication-inventory.csv` | 17 | publication-ready-held-claims | evidence;sla;transit;upgrade |

## Residual Holds

- Source snapshot repeat-window or archive-history proof remains required before
  evidence claims.
- T4 terminal-access proof remains required for upgrade/evidence claims.
- T2 asset-condition debt remains payable before SLA, transit, or upgrade claims.

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- map-publication-readiness --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `cargo run -q -p route -- map-atlas --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md docs\map-publication-scope.md waves\PHASES.md waves\2026-05-15-map-publication-inventory-package`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Use `data/map-publication-inventory.csv` as the exact current publishable map
set, or continue reducing non-publication evidence, upgrade, SLA, transit, and
repair holds.
