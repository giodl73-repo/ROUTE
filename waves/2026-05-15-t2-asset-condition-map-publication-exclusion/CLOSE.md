---
wave: t2-asset-condition-map-publication-exclusion
date_closed: 2026-05-15
status: done
---

# T2 Asset-Condition Map Publication Exclusion Close

## Decision

T2 asset-condition debt is now scoped out of current map publication claims. The
rows still exist in the optimizer ledger and still block `sla`, `transit`, and
`upgrade`, but no longer block `publication`.

## Evidence

| Rows | Claims before | Claims after | Debt cleared |
|---:|---|---|---|
| 9 | sla;transit;upgrade;publication | sla;transit;upgrade | no |

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
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `cargo run -q -p route -- map-atlas --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md docs\map-publication-scope.md waves\PHASES.md waves\2026-05-15-t2-asset-condition-map-publication-exclusion`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Use current T1-T4 structural maps as map-publication-valid held-claim surfaces,
or continue reducing non-publication holds for evidence, upgrade, SLA, transit,
and repair readiness.
