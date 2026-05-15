---
wave: priority-a-pavement-funding-evidence-source-access
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Funding Evidence Source Access Close

## Decision

Priority-A pavement funding evidence acquisition requires manual or cached
accepted full-cost programming or DOT commitment artifacts. No safe live funding
commitment fetcher is declared. All rows remain held, source-needed,
not-accepted, and relief-ineligible.

## Evidence

| State | Route | Access mode | Evidence artifact | Evidence status | Relief eligibility |
|---|---|---|---|---|---|
| TX | I220 | manual-or-cached-source-needed | source-needed | not-accepted | not-eligible-for-relief |
| LA | I220 | manual-or-cached-source-needed | source-needed | not-accepted | not-eligible-for-relief |
| NM | I110 | manual-or-cached-source-needed | source-needed | not-accepted | not-eligible-for-relief |
| LA | I110 | manual-or-cached-source-needed | source-needed | not-accepted | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-source-access --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-evidence-source-access`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Funding evidence intake requirements must still be defined before artifact
  collection or attachment.
- Non-priority pavement source debt remains open for US30, US2, and US6.
