---
wave: map-publication-readiness-certification
date_closed: 2026-05-15
status: done
---

# Map Publication Readiness Certification Close

## Decision

Current T1-T4 structural maps are now gate-certified for map publication as
held-claim surfaces. The readiness artifact reports zero residual
`publication` blockers while preserving non-publication holds.

## Evidence

| Artifact | Maps | Publication blockers | Held claims | Status |
|---|---:|---:|---|---|
| `data/map-publication-readiness.csv` | 17 | 0 | evidence;sla;transit;upgrade | pass |

## Residual Holds

- Source snapshot repeat-window or archive-history proof remains required before
  evidence claims.
- T4 terminal-access proof remains required for upgrade/evidence claims.
- T2 asset-condition debt remains payable before SLA, transit, or upgrade claims.

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `cargo run -q -p route -- map-publication-readiness --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `cargo run -q -p route -- map-atlas --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md docs\map-publication-scope.md waves\PHASES.md waves\2026-05-15-map-publication-readiness-certification`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Use the current structural maps as publication-ready held-claim T1-T4 maps, or
continue reducing non-publication evidence, upgrade, SLA, transit, and repair
holds.
