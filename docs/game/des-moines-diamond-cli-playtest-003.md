# Des Moines Diamond CLI Playtest 003

Date: 2026-05-09  
Scenario: `des-moines-diamond`  
Scenario version: G0 v0.2  
CLI slice: G1-A active-project countdown  
Evidence level: Heuristic

## Purpose

Verify that multi-season projects can move from active to complete before event resolution. This closes the biggest implementation gap found in CLI playtest 002.

## Commands Run

```powershell
cargo test -p route game::

cargo run -q -p route -- game run-season des-moines-diamond `
  --season 1 `
  --event night-work-zone-closure `
  --project diamond-connector-package `
  --write-state $env:TEMP\route-des-moines-countdown-1.json

cargo run -q -p route -- game run-season des-moines-diamond `
  --season 2 `
  --event night-work-zone-closure `
  --state $env:TEMP\route-des-moines-countdown-1.json `
  --write-state $env:TEMP\route-des-moines-countdown-2.json

cargo run -q -p route -- game run-season des-moines-diamond `
  --season 3 `
  --event full-interchange-zone-closure `
  --state $env:TEMP\route-des-moines-countdown-2.json `
  --write-state $env:TEMP\route-des-moines-countdown-3.json
```

## Results

| Check | Result |
|---|---|
| Game unit tests | Pass; 10 tests |
| Season 1 starts connector package | Pass |
| Season 1 stores connector as active with 2 seasons remaining | Pass |
| Season 2 ticks connector to 1 season remaining | Pass |
| Season 3 completes connector before closure event | Pass |
| Closure event sees redundant transfer paths | Pass |
| Publication gate remains locked despite operational success | Pass |

## Observed Run

Season 1:

- Accepted `diamond-connector-package`.
- Budget moved from 12 to 7.
- Active projects: `diamond-connector-package:2 seasons`.
- Closure was not tested yet.

Season 2:

- No new project selected.
- Active projects: `diamond-connector-package:1 seasons`.

Season 3:

- Connector completed before `full-interchange-zone-closure`.
- Event result: redundant transfer paths are available.
- Completed projects: `diamond-connector-package`.
- Throughput retention: `1.000 heuristic`.
- Publication gate remained locked because observed closure evidence is still missing.

## Reading

The resolver now distinguishes three states that matter for the game:

- Bought but not yet complete.
- Complete and operationally useful.
- Operationally useful but still not publication-grade evidence.

That is the right structure for the topology lesson.

## Decision

Active-project countdown is ready for the first score command.

Next implementation step:

1. Add `route game score`.
2. Score a fixture session log.
3. Preserve the publication gate as a separate result from the operational win band.
