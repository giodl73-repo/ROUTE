---
wave: priority-a-pavement-funding-commitment-review
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Funding Commitment Review Close

## Decision

No priority-A pavement repair funding row has an accepted commitment artifact
attached. All four rows remain relief-ineligible and keep their SLA, transit,
upgrade, and publication blockers.

## Evidence

| State | Route | Repair cost proxy | Commitment status | Relief eligibility |
|---|---|---:|---|---|
| TX | I220 | $10.00M | no-accepted-commitment-attached | not-eligible-for-relief |
| LA | I220 | $25.00M | no-accepted-commitment-attached | not-eligible-for-relief |
| NM | I110 | $5.00M | no-accepted-commitment-attached | not-eligible-for-relief |
| LA | I110 | $25.00M | no-accepted-commitment-attached | not-eligible-for-relief |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-commitment-review --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-commitment-review`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Downgrade/exclusion or accepted funding evidence is still required before
  asset-condition relief replay.
- Non-priority pavement source debt remains open for US30, US2, and US6.
