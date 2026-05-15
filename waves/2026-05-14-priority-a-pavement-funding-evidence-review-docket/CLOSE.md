---
wave: priority-a-pavement-funding-evidence-review-docket
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Funding Evidence Review Docket Close

## Decision

Priority-A pavement funding evidence remains unavailable for review because no
accepted full-cost funding artifact is attached. All rows remain held, not
reviewed, not accepted, and relief-ineligible.

## Evidence

| State | Route | Minimum commitment | Review decision | Evidence status | Relief eligibility |
|---|---|---:|---|---|---|
| TX | I220 | $10.00M | held-no-attached-artifact | not-accepted | not-eligible-for-relief |
| LA | I220 | $25.00M | held-no-attached-artifact | not-accepted | not-eligible-for-relief |
| NM | I110 | $5.00M | held-no-attached-artifact | not-accepted | not-eligible-for-relief |
| LA | I110 | $25.00M | held-no-attached-artifact | not-accepted | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-review-docket --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-evidence-review-docket`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Accepted funding artifacts must still be acquired and attached before any
  asset-condition relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
