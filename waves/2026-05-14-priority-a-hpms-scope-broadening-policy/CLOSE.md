---
wave: priority-a-hpms-scope-broadening-policy
date_closed: 2026-05-14
status: done
---

# Priority A HPMS Scope Broadening Policy Close

## Decision

The governed HPMS scope-broadening policy is closed. `route fetch-hpms` now
supports explicit functional systems while preserving the default system `1`
behavior. Non-default scope is allowed only with `--states`, keeping broader
cache mutation state-scoped. TX, LA, and NM have broadening rows for systems
`1,2,3`, but no broadened fetch has been executed or accepted.

## Evidence

| State | Source-needed routes | Source-needed members | Broadening command |
|---|---|---:|---|
| TX | `US287;US70;US80;US83` | 78 | `route fetch-hpms --states TX --functional-systems 1,2,3` |
| LA | `US80` | 10 | `route fetch-hpms --states LA --functional-systems 1,2,3` |
| NM | `US70;US80` | 14 | `route fetch-hpms --states NM --functional-systems 1,2,3` |

## Artifacts

| Artifact | Role |
|---|---|
| `data/tier-pavement-hpms-scope-broadening.csv` | Gated scope-broadening plan |
| `data/source-fetch-policy.csv` | Registers scoped functional-system fetch mutation policy |
| `data/tier-optimizer-runs.csv` | Registers the held-known broadening stage |
| `data/release-manifest.csv` | Registers the held-public broadening artifact |
| `panels/scope-broadening/review.md` | Role review preserving blockers |

## Gate Bundle

- `cargo fmt -p route-data -p route`
- `cargo test -p route-data`
- `cargo test -p route`
- `cargo run -q -p route -- source-fetch-policy --gate`
- `cargo run -q -p route -- tier-pavement-hpms-scope-broadening --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-priority-a-hpms-scope-broadening-policy`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- The broadened fetch has not been run.
- Cache population remains non-acceptance until a postfetch review verifies the
  affected pavement members and replays relief separately.
- All TX/LA/NM source-needed and repair-debt blockers remain held.
