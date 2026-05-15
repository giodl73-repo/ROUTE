---
wave: priority-a-pavement-funding-evidence-source-capture
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Funding Evidence Source Capture Close

## Decision

Priority-A pavement funding evidence has no accepted artifact attached. The
source-capture ledger records placeholders only; all rows remain held and
relief-ineligible.

## Evidence

| State | Route | Minimum commitment | Captured artifact | Evidence status | Relief eligibility |
|---|---|---:|---|---|---|
| TX | I220 | $10.00M | none | not-accepted | not-eligible-for-relief |
| LA | I220 | $25.00M | none | not-accepted | not-eligible-for-relief |
| NM | I110 | $5.00M | none | not-accepted | not-eligible-for-relief |
| LA | I110 | $25.00M | none | not-accepted | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-source-capture --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-evidence-source-capture`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Accepted funding artifacts must still be attached and reviewed before any
  asset-condition relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
