# Des Moines Diamond CLI Playtest 002

Date: 2026-05-09  
Scenario: `des-moines-diamond`  
Scenario version: G0 v0.2  
CLI slice: G1-A deterministic `run-season`  
Evidence level: Heuristic

## Purpose

Exercise the first deterministic season resolver with explicit events and projects. This playtest checks the parts that matter before scoring exists: accepted projects, rejected actions with reasons, state writing, log appending, and publication-gate separation.

## Commands Run

```powershell
cargo test -p route game::

cargo run -q -p route -- game run-season des-moines-diamond `
  --season 1 `
  --event full-interchange-zone-closure `
  --project general-purpose-widening `
  --project intelligent-routing

cargo run -q -p route -- game run-season des-moines-diamond `
  --season 2 `
  --event source-challenge `
  --project source-request `
  --project validated-evidence

cargo run -q -p route -- game run-season des-moines-diamond `
  --season 1 `
  --event night-work-zone-closure `
  --project work-zone-sequencing `
  --write-state $env:TEMP\route-des-moines-state.json `
  --append-log $env:TEMP\route-des-moines-session.csv

cargo run -q -p route -- game run-season des-moines-diamond `
  --season 2 `
  --event relay-hub-surge `
  --project relay-hub-reserve-staffing `
  --state $env:TEMP\route-des-moines-state.json `
  --write-state $env:TEMP\route-des-moines-state-2.json `
  --append-log $env:TEMP\route-des-moines-session.csv
```

## Results

| Check | Result |
|---|---|
| Game unit tests | Pass; 8 tests |
| Capacity-first run accepts widening/routing | Pass |
| Capacity-first run still names transfer fragility | Pass |
| Evidence-first run accepts `source-request` | Pass |
| Evidence-first run rejects `validated-evidence` with reason | Pass |
| Publication gate remains locked after operational actions | Pass |
| `--write-state` writes JSON state | Pass |
| `--append-log` writes append-only CSV rows | Pass |
| `--state` resumes from prior JSON | Pass |

## Playtest Runs

### Capacity-First Closure

Projects:

- `general-purpose-widening`
- `intelligent-routing`

Outcome:

- Accepted both projects.
- Budget moved from 12 to 6.
- Crews moved from 3 to 0.
- Event result explicitly says the transfer remains fragile because no independent transfer path is complete.
- Publication gate stayed locked.

Reading: this preserves the intended warning. The player can buy capacity and still not solve topology.

### Evidence-First Review

Projects:

- `source-request`
- `validated-evidence`

Outcome:

- Accepted `source-request`.
- Rejected `validated-evidence` with: validated evidence unavailable; no observed artifact exists yet.
- Evidence confidence moved from 2 to 3.
- Source challenge still held publication locked because evidence confidence is below 4.

Reading: the v0.2 evidence split works better than the old single evidence card. It feels like a review gate rather than a tax.

### Resume And Log

Season 1 wrote JSON state and CSV session log after `work-zone-sequencing`.

Season 2 resumed from that state, accepted `relay-hub-reserve-staffing`, preserved completed `work-zone-sequencing`, and appended a second CSV row.

Reading: QUEST continuity is viable. The session log is simple but already useful.

## Findings

| Finding | Severity | Action |
|---|---|---|
| Multi-season construction is not modeled yet; projects with `time > 1` become active but do not tick down | Medium | Add active-project countdown before scoring |
| Throughput/recovery values are fixed heuristic outputs | Expected | G1-B should summarize live ROUTE hooks directly |
| No score command exists yet | Expected | Implement after run-season countdown and fixture stabilize |

## Decision

G1-A deterministic run-season is useful enough for continued playtesting, with known limits.

Next implementation step:

1. Add active-project countdown and completion.
2. Add score bands and Budget Discipline cap behavior.
3. Add fixture tests for a two-season session log.
