---
wave: priority-a-pavement-funding-evidence-accepted-metadata-artifact-acquisition
date_closed: 2026-05-15
status: done
---

# Priority A Pavement Funding Evidence Accepted Metadata Artifact Acquisition Close

## Decision

Priority-A pavement funding evidence accepted metadata attachment-review rows now
have explicit source-needed acquisition/cache targets. No accepted artifact is
cached, attached, reviewed, or accepted by this wave; all rows remain
relief-ineligible.

## Evidence

| State | Route | Acquisition status | Cache status | Candidate source owner | Evidence status | Relief eligibility |
|---|---|---|---|---|---|---|
| TX | I220 | source-needed | not-cached | TX DOT or accepted programming authority | not-accepted | not-eligible-for-relief |
| LA | I220 | source-needed | not-cached | LA DOT or accepted programming authority | not-accepted | not-eligible-for-relief |
| NM | I110 | source-needed | not-cached | NM DOT or accepted programming authority | not-accepted | not-eligible-for-relief |
| LA | I110 | source-needed | not-cached | LA DOT or accepted programming authority | not-accepted | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-accepted-metadata-artifact-acquisition --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-15-priority-a-pavement-funding-evidence-accepted-metadata-artifact-acquisition`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Accepted artifact source/cache access must still be classified before evidence
  intake, acceptance, or asset-condition relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
