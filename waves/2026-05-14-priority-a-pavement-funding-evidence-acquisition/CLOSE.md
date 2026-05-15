---
wave: priority-a-pavement-funding-evidence-acquisition
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Funding Evidence Acquisition Close

## Decision

Priority-A pavement funding evidence acquisition now has explicit source-needed
targets for accepted full-cost programming or DOT commitment artifacts. No
artifact is attached or accepted by this wave; all rows remain held and
relief-ineligible.

## Evidence

| State | Route | Minimum commitment | Acquisition status | Evidence status | Relief eligibility |
|---|---|---:|---|---|---|
| TX | I220 | $10.00M | source-needed | not-accepted | not-eligible-for-relief |
| LA | I220 | $25.00M | source-needed | not-accepted | not-eligible-for-relief |
| NM | I110 | $5.00M | source-needed | not-accepted | not-eligible-for-relief |
| LA | I110 | $25.00M | source-needed | not-accepted | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-acquisition --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-evidence-acquisition`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Source access for the accepted funding artifacts must still be classified
  before artifact collection or attachment.
- Non-priority pavement source debt remains open for US30, US2, and US6.
