---
wave: priority-a-pavement-funding-evidence-metadata-capture
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Funding Evidence Metadata Capture Close

## Decision

Priority-A pavement funding evidence now has explicit metadata-capture
placeholders. No accepted artifact title, URL, cached artifact, or committed
amount is captured by this wave; all rows remain held, source-needed, not
reviewed, not accepted, and relief-ineligible.

## Evidence

| State | Route | Metadata status | Captured artifact | Source title | Evidence status | Relief eligibility |
|---|---|---|---|---|---|---|
| TX | I220 | source-needed | none | source-needed | not-accepted | not-eligible-for-relief |
| LA | I220 | source-needed | none | source-needed | not-accepted | not-eligible-for-relief |
| NM | I110 | source-needed | none | source-needed | not-accepted | not-eligible-for-relief |
| LA | I110 | source-needed | none | source-needed | not-accepted | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-metadata-capture --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-evidence-metadata-capture`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Funding evidence artifact attachment must still attach a real accepted artifact
  before review.
- Non-priority pavement source debt remains open for US30, US2, and US6.
