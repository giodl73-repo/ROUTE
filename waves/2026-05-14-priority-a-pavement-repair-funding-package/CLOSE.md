---
wave: priority-a-pavement-repair-funding-package
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Repair Funding Package Close

## Decision

The priority-A pavement repair package is explicitly unfunded and not eligible
for asset-condition relief. All SLA, transit, upgrade, and publication blockers
remain held.

## Evidence

| State | Route | Repair members | Repair cost proxy | Funding status |
|---|---|---:|---:|---|
| TX | I220 | 4 | $10.00M | unfunded |
| LA | I220 | 10 | $25.00M | unfunded |
| NM | I110 | 2 | $5.00M | unfunded |
| LA | I110 | 10 | $25.00M | unfunded |

Total unfunded repair package: $65.00M.

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-repair-funding-package --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-repair-funding-package`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- An accepted funding commitment, downgrade, or exclusion decision is still
  required before relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
