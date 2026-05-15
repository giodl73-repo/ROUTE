---
wave: priority-a-pavement-repair-disposition
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Repair Disposition Close

## Decision

All four priority-A repair-debt rows are repair-funding-required and
not eligible for asset-condition relief. The wave preserves all blockers and
does not change T1 selection, maps, SLA, transit, upgrade, or publication
claims.

## Evidence

| State | Route | Repair members | Repair cost proxy | Disposition |
|---|---|---:|---:|---|
| TX | I220 | 4 | $10.00M | repair-funding-required |
| LA | I220 | 10 | $25.00M | repair-funding-required |
| NM | I110 | 2 | $5.00M | repair-funding-required |
| LA | I110 | 10 | $25.00M | repair-funding-required |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-repair-disposition --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-repair-disposition`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- A repair funding package, downgrade decision, or exclusion decision is still
  required before relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
