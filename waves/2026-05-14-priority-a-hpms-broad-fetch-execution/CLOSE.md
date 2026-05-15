---
wave: priority-a-hpms-broad-fetch-execution
date_closed: 2026-05-14
status: done
---

# Priority A HPMS Broad Fetch Execution Close

## Decision

The governed broadened HPMS fetch executed successfully. Priority-A
source-needed pavement holds for TX, LA, and NM are no longer source-acquisition
holds after rebuild. They are now classified as repair debt and remain blocked
until a separate repair-debt review or relief replay.

## Evidence

| State | Broad cache records | Priority-A source-needed members | Priority-A repair members | Review status |
|---|---:|---:|---:|---|
| TX | 208,285 | 0 | 4 | `repair-debt-not-source-join` |
| LA | 33,967 | 0 | 20 | `repair-debt-not-source-join` |
| NM | 39,115 | 0 | 2 | `repair-debt-not-source-join` |

Systemwide pavement debt changed from 13 rows / $95.95M to 9 rows / $87.20M.
The reduction is evidence classification from broader HPMS coverage, not relief
replay or debt payment.

## Artifacts

| Artifact | Role |
|---|---|
| `data/tier-pavement-docket.csv` | Rebuilt pavement member classifications |
| `data/tier-pavement-source-gaps.csv` | Current pavement source-gap and repair-debt surface |
| `data/tier-pavement-debt-budget.csv` | Current debt budget after broad HPMS coverage |
| `data/tier-pavement-source-fetch-attempt.csv` | Broad cache record counts |
| `data/tier-pavement-unmatched-join-review.csv` | Priority-A source-needed closure and repair-debt split |
| `data/tier-pavement-hpms-scope-broadening.csv` | Broadening status after fetch execution |
| `panels/broad-fetch/review.md` | Role review preserving relief boundary |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- source-fetch-policy --gate`
- `cargo run -q -p route -- fetch-hpms --states TX,LA,NM --functional-systems 1,2,3`
- `cargo run -q -p route -- build --all-roads`
- `cargo run -q -p route -- tier-pavement-docket --gate`
- `cargo run -q -p route -- tier-pavement-source-gaps --gate`
- `cargo run -q -p route -- tier-pavement-debt-budget --gate`
- `cargo run -q -p route -- tier-pavement-source-fetch-attempt --gate`
- `cargo run -q -p route -- tier-pavement-source-fetch-review --gate`
- `cargo run -q -p route -- tier-pavement-unmatched-join-review --gate`
- `cargo run -q -p route -- tier-pavement-hpms-scope-broadening --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-priority-a-hpms-broad-fetch-execution`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- Priority-A TX/LA/NM pavement blockers are now repair-debt blockers.
- Non-priority source-needed pavement debt remains for US30, US2, and US6.
- No selector, SLA, transit, publication, or upgrade claim is promoted by this
  wave.
