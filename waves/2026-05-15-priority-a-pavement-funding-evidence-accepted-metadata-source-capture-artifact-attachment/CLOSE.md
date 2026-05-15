---
wave: priority-a-pavement-funding-evidence-accepted-metadata-source-capture-artifact-attachment
date_closed: 2026-05-15
status: done
---

# Priority A Pavement Funding Evidence Accepted Metadata Source-Capture Artifact Attachment Close

## Decision

Priority-A pavement funding evidence accepted metadata source-capture rows now
have explicit artifact-attachment placeholders. No accepted artifact is attached,
reviewed, or accepted by this wave; all rows remain held and relief-ineligible.

## Evidence

| State | Route | Attachment status | Attached artifact | Source title | Source URL | Amount | Evidence status | Relief eligibility |
|---|---|---|---|---|---|---|---|---|
| TX | I220 | source-needed | none | source-needed | source-needed | source-needed | not-accepted | not-eligible-for-relief |
| LA | I220 | source-needed | none | source-needed | source-needed | source-needed | not-accepted | not-eligible-for-relief |
| NM | I110 | source-needed | none | source-needed | source-needed | source-needed | not-accepted | not-eligible-for-relief |
| LA | I110 | source-needed | none | source-needed | source-needed | source-needed | not-accepted | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-accepted-metadata-source-capture-artifact-attachment --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-15-priority-a-pavement-funding-evidence-accepted-metadata-source-capture-artifact-attachment`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Accepted metadata source-capture artifact-attachment placeholders must still be
  reviewed before evidence acceptance or asset-condition relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
