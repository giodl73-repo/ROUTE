---
wave: priority-a-pavement-funding-evidence-artifact-attachment
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Funding Evidence Artifact Attachment Close

## Decision

Priority-A pavement funding evidence still has no accepted artifact attached.
The artifact-attachment ledger records placeholders only; all rows remain held,
not reviewed, not accepted, and relief-ineligible.

## Evidence

| State | Route | Minimum commitment | Attached artifact | Review status | Relief eligibility |
|---|---|---:|---|---|---|
| TX | I220 | $10.00M | none | not-reviewed | not-eligible-for-relief |
| LA | I220 | $25.00M | none | not-reviewed | not-eligible-for-relief |
| NM | I110 | $5.00M | none | not-reviewed | not-eligible-for-relief |
| LA | I110 | $25.00M | none | not-reviewed | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-artifact-attachment --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-evidence-artifact-attachment`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Accepted funding artifacts must still be reviewed before any asset-condition
  relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
