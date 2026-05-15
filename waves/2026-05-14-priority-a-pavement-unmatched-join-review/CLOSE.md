---
wave: priority-a-pavement-unmatched-join-review
date_closed: 2026-05-14
status: done
---

# Priority A Pavement Unmatched Join Review Close

## Decision

Priority-A unmatched pavement join review is closed without relief. TX, LA, and
NM all have populated HPMS caches, but their source-needed priority-A members
are on US-route bundles with zero HPMS IRI route records in the current cache
scope. Repair-required members are separated and remain repair debt, not source
acquisition debt.

## Evidence

| State | Cache records | Source-needed members | Repair-required members | HPMS records for source-needed routes | Decision |
|---|---:|---:|---:|---:|---|
| TX | 43,381 | 78 | 4 | 0 | Attach state DOT pavement evidence or broaden HPMS scope for `US287;US70;US80;US83` |
| LA | 10,892 | 10 | 20 | 0 | Attach state DOT pavement evidence or broaden HPMS scope for `US80` |
| NM | 12,020 | 14 | 2 | 0 | Attach state DOT pavement evidence or broaden HPMS scope for `US70;US80` |

## Artifacts

| Artifact | Role |
|---|---|
| `data/tier-pavement-unmatched-join-review.csv` | Gated unmatched-join review rows |
| `data/tier-optimizer-runs.csv` | Registers the held-known unmatched-join stage |
| `data/release-manifest.csv` | Registers the held-public review artifact |
| `panels/unmatched-join/review.md` | Role review preserving blockers |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-unmatched-join-review --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-unmatched-join-review`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- TX, LA, and NM source-needed US-route members need state DOT pavement
  condition evidence or a broader HPMS fetch/join contract before relief.
- Repair-required interstate members stay on the pavement repair debt rail.
- No selector, SLA, transit, publication, or upgrade claim is promoted by this
  wave.
