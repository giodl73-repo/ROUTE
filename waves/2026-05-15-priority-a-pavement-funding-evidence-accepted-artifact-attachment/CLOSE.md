---
wave: priority-a-pavement-funding-evidence-accepted-artifact-attachment
date_closed: 2026-05-15
status: done
---

# Priority A Pavement Funding Evidence Accepted Artifact Attachment Close

## Decision

Priority-A pavement funding evidence now has explicit accepted-artifact
attachment placeholders after metadata capture. No accepted artifact is attached
by this wave; all rows remain held, source-needed, not reviewed, not accepted,
and relief-ineligible.

## Evidence

| State | Route | Attachment status | Attached artifact | Source title | Evidence status | Relief eligibility |
|---|---|---|---|---|---|---|
| TX | I220 | source-needed | none | source-needed | not-accepted | not-eligible-for-relief |
| LA | I220 | source-needed | none | source-needed | not-accepted | not-eligible-for-relief |
| NM | I110 | source-needed | none | source-needed | not-accepted | not-eligible-for-relief |
| LA | I110 | source-needed | none | source-needed | not-accepted | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-accepted-artifact-attachment --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-15-priority-a-pavement-funding-evidence-accepted-artifact-attachment`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Funding evidence review must still reject or hold unattached accepted-artifact
  placeholders before any relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
