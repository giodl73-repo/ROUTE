---
wave: priority-a-nm-pavement-fetch-repair
date_closed: 2026-05-14
status: done
---

# Priority A NM Pavement Fetch Repair Close

## Decision

The New Mexico fetch blocker is repaired. The HPMS source table now uses
`NewMexico` for the FHWA hosted service, matching `NewMexico_2018_PR`.
The scoped NM fetch produced 12,020 cache records, but NM remains unaccepted
because the current pavement source-gap surface still reports 23 unresolved
priority-A members.

## Evidence

| State | Cache records before | Cache records after | Review status | Unresolved members |
|---|---:|---:|---|---:|
| NM | 0 | 12,020 | `cache-populated-source-gap-still-open` | 23 |

## Artifacts

| Artifact | Role |
|---|---|
| `crates/route-data/src/hpms_fetch.rs` | Fixes FHWA service names for multi-word states |
| `data/tier-pavement-source-fetch-attempt.csv` | Reclassifies NM as populated but unreviewed |
| `data/tier-pavement-source-fetch-review.csv` | Preserves NM blockers pending join/evidence review |
| `data/tier-optimizer-runs.csv` | Updates held-known fetch-review summary |
| `panels/nm-fetch-repair/review.md` | Role review preserving blockers |

## Gate Bundle

- `cargo fmt -p route-data -p route`
- `cargo test -p route-data`
- `cargo test -p route`
- `cargo run -q -p route -- fetch-hpms --states NM`
- `cargo run -q -p route -- build --all-roads`
- `cargo run -q -p route -- tier-pavement-docket --gate`
- `cargo run -q -p route -- tier-pavement-source-gaps --gate`
- `cargo run -q -p route -- tier-pavement-debt-budget --gate`
- `cargo run -q -p route -- tier-pavement-source-fetch-attempt --gate`
- `cargo run -q -p route -- tier-pavement-source-fetch-review --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-priority-a-nm-pavement-fetch-repair`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- TX, LA, and NM now all have populated priority-A HPMS caches, but all three
  remain source-gap open after the current join/build.
- Next work is unmatched-join review or state DOT pavement evidence attachment
  for TX/LA/NM before any debt relief replay.
