---
wave: priority-a-pavement-repair-debt-review
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Repair Debt Review Close

## Decision

Priority-A TX/LA/NM pavement blockers are confirmed repair debt and remain
optimizer-held. The review does not pay debt, accept evidence, or reduce SLA,
transit, upgrade, or publication blockers.

## Evidence

| State | Route | Bundle rows | Repair members | Repair cost proxy |
|---|---|---:|---:|---:|
| TX | I220 | 1 | 4 | $10.00M |
| LA | I220; I110 | 2 | 20 | $50.00M |
| NM | I110 | 1 | 2 | $5.00M |

## Artifacts

| Artifact | Role |
|---|---|
| `data/tier-pavement-repair-debt-review.csv` | Priority-A repair debt review |
| `panels/repair-debt/review.md` | Role review preserving blocker boundary |
| `docs/optimizer-constraint-ledger-spec.md` | Source-ledger doctrine reference |
| `docs/SPEC_INDEX.md` | Claim/artifact index |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-repair-debt-review --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-repair-debt-review`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Priority-A repair debt requires a disposition wave before relief replay.
- Non-priority source-needed pavement debt remains for US30, US2, and US6.
- No map publication or T1 selector claim is promoted by this wave.
