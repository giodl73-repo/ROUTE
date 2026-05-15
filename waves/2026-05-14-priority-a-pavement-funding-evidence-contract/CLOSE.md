---
wave: priority-a-pavement-funding-evidence-contract
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Funding Evidence Contract Close

## Decision

Priority-A pavement repair rows require accepted full-cost funding evidence
before asset-condition relief replay. No funding evidence is accepted by this
wave; all rows remain source-needed and relief-ineligible.

## Evidence

| State | Route | Minimum commitment | Evidence status | Relief eligibility |
|---|---|---:|---|---|
| TX | I220 | $10.00M | source-needed | not-eligible-for-relief |
| LA | I220 | $25.00M | source-needed | not-eligible-for-relief |
| NM | I110 | $5.00M | source-needed | not-eligible-for-relief |
| LA | I110 | $25.00M | source-needed | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-contract --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-evidence-contract`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Accepted funding evidence must still be captured or attached before
  asset-condition relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
