---
wave: priority-a-pavement-downgrade-exclusion-decision
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Downgrade Exclusion Decision Close

## Decision

Priority-A pavement repair rows are not downgraded or excluded. They remain
held at current tier, unfunded, and not eligible for asset-condition relief.

## Evidence

| State | Route | Repair cost proxy | Downgrade | Exclusion | Service status |
|---|---|---:|---|---|---|
| TX | I220 | $10.00M | no-downgrade-selected | no-exclusion-selected | held-at-current-tier |
| LA | I220 | $25.00M | no-downgrade-selected | no-exclusion-selected | held-at-current-tier |
| NM | I110 | $5.00M | no-downgrade-selected | no-exclusion-selected | held-at-current-tier |
| LA | I110 | $25.00M | no-downgrade-selected | no-exclusion-selected | held-at-current-tier |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-downgrade-exclusion-decision --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-downgrade-exclusion-decision`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Accepted priority-A pavement funding evidence is still required before
  asset-condition relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
