---
wave: priority-a-pavement-funding-evidence-accepted-intake
date_closed: 2026-05-15
status: done
---

# Priority A Pavement Funding Evidence Accepted Intake Close

## Decision

Priority-A pavement funding evidence accepted source-access rows now have an
explicit intake contract for accepted full-cost programming or DOT commitment
artifacts. No artifact is cached, attached, reviewed, or accepted by this wave;
all rows remain held and relief-ineligible.

## Evidence

| State | Route | Intake status | Cache status | Evidence artifact | Review status | Evidence status | Relief eligibility |
|---|---|---|---|---|---|---|---|
| TX | I220 | artifact-required | not-cached | source-needed | not-reviewed | not-accepted | not-eligible-for-relief |
| LA | I220 | artifact-required | not-cached | source-needed | not-reviewed | not-accepted | not-eligible-for-relief |
| NM | I110 | artifact-required | not-cached | source-needed | not-reviewed | not-accepted | not-eligible-for-relief |
| LA | I110 | artifact-required | not-cached | source-needed | not-reviewed | not-accepted | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-accepted-intake --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-15-priority-a-pavement-funding-evidence-accepted-intake`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Accepted artifact metadata-capture placeholders must still be recorded before
  evidence attachment, review, acceptance, or relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
