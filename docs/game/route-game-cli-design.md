# `route game` CLI Design

`route game` is the reproducible command surface for Interstate Tycoon. It should turn paper scenarios into deterministic state transitions without hiding the ROUTE evidence labels underneath.

Phase target: G1-A CLI Encoded  
First scenario: `des-moines-diamond`  
Primary paper source: `docs/game/des-moines-diamond-g0.md`  
Primary playtest source: `docs/game/des-moines-diamond-playtest.md`

## Design Goals

- Print the same setup as the paper artifact.
- Resolve one season from explicit inputs.
- Score a session log without browser/UI code.
- Keep every output evidence-labeled.
- Keep the simulation layer separate from the narrative/game layer.
- Make checkpoint/resume natural by using append-only logs.

## Non-Goals

- No AI opponent yet.
- No hidden randomness in G1-A.
- No browser state format yet.
- No balancing tournament yet.
- No claim should become publication-grade because it appears in the game CLI.

## Command Surface

### `route game scenarios`

Lists playable scenarios.

Output fields:

| Field | Example |
|---|---|
| `scenario_id` | `des-moines-diamond` |
| `phase` | `G0-B` |
| `evidence_level` | `Heuristic` |
| `standards` | `T1-DIAMOND-K; T1-FLYOVER; T1-RECOVERY` |
| `engine_hook` | `route sim scenario des-moines-interchange --intervention` |
| `publication_gate` | `locked: empirical closure evidence missing` |

### `route game inspect des-moines-diamond`

Prints the scenario setup in a stable, parseable order.

Sections:

- Hook.
- One aha.
- Tracks and starting values.
- Project cards.
- Event cards.
- Evidence cards.
- Win bands.
- Publication gate.
- ROUTE engine hooks.

### `route game run-season des-moines-diamond`

Resolves one deterministic season.

Inputs:

| Option | Meaning |
|---|---|
| `--season <n>` | Season number |
| `--event <slug>` | Event card slug |
| `--project <slug>` | Project card; repeatable |
| `--state <path>` | Optional prior JSON state |
| `--write-state <path>` | Optional output JSON state |
| `--append-log <path>` | Optional append-only session log |

Example:

```powershell
route game run-season des-moines-diamond `
  --season 1 `
  --event full-interchange-zone-closure `
  --project general-purpose-widening `
  --project intelligent-routing `
  --write-state data/game/des-moines-season-1.json `
  --append-log data/game/des-moines-session.csv
```

G1-A rule: if no `--state` is passed, the season starts from the paper default state. There is no hidden draw deck yet.

### `route game score des-moines-diamond`

Scores a session log and prints operational result plus publication gate.

Inputs:

| Option | Meaning |
|---|---|
| `--log <path>` | CSV or JSONL session log |
| `--details` | Print dimension-by-dimension scoring |
| `--gate-promotion` | Fail if G0-C/G1-A promotion gates are not met |

Output sections:

- Operational score.
- Win band.
- Aha status, if present in log metadata.
- Surprise count.
- Publication gate.
- Promotion readiness.

## State Model

G1-A state is intentionally small.

```text
scenario_id
season
tracks:
  budget
  crews
  political_capital
  public_patience
  operations_capacity
  evidence_confidence
projects:
  active
  completed
flags:
  first_closure_seen
  connector_package_complete
  source_requested
  validated_evidence_available
  fiscal_crisis
  analyzer_anchor_recognized
  empirical_closure_evidence
last_result:
  throughput_retention
  recovery_hours
  sla_status
  publication_gate
```

State must serialize cleanly to JSON. Future browser code can consume the same shape.

## Card Slugs

Project cards:

| Slug | Paper card |
|---|---|
| `diamond-connector-package` | Diamond connector package |
| `express-freight-flyovers` | Express freight flyovers |
| `work-zone-sequencing` | Work-zone sequencing |
| `intelligent-routing` | Intelligent routing |
| `relay-hub-reserve-staffing` | Relay hub reserve staffing |
| `ev-rest-hardening` | EV/rest hardening |
| `general-purpose-widening` | General-purpose widening |
| `source-request` | Source request |
| `validated-evidence` | Validated evidence |

Event cards:

| Slug | Paper card |
|---|---|
| `full-interchange-zone-closure` | Full interchange-zone closure |
| `night-work-zone-closure` | Night work-zone closure |
| `relay-hub-surge` | Relay hub surge |
| `ev-rest-queue` | EV/rest queue |
| `political-lane-mile-pressure` | Political lane-mile pressure |
| `source-challenge` | Source challenge |

## Scoring Contract

The CLI score must match the paper dimensions.

| Dimension | Max | G1-A source |
|---|---:|---|
| Throughput retention | 25 | Scenario/intervention proxy or explicit session field |
| Recovery | 20 | T90/recovery proxy or explicit session field |
| SLA | 15 | Bounded heuristic status |
| Budget discipline | 10 | Final budget non-negative |
| Public support | 10 | Political capital and public patience non-negative |
| Evidence honesty | 20 | Publication gate and evidence labels present |

G1-A can use fixed heuristic values from the current Des Moines engine facts. G1-B should call/summarize ROUTE outputs more directly.

## Publication Gate

For Des Moines G1-A:

| Gate | Current status |
|---|---|
| Diamond analyzer recognizes Des Moines node | pass |
| Empirical closure probability and duration exist | locked |
| Direct PTI/NPMRDS validation exists | locked |
| Observed versus modeled labels are present | required |

Operational win and publication gate must remain separate in output.

## Test Plan

L0 tests:

- Scenario slugs parse.
- Project/event slugs reject unknown values.
- Default state matches paper tracks.
- Season resolution is deterministic.
- Score bands match paper thresholds.
- Publication gate remains locked when empirical evidence is missing.

L1 tests:

- `route game inspect des-moines-diamond` includes all paper cards.
- `route game run-season ...` writes stable JSON state.
- `route game score ...` scores a fixture session log.
- `--gate-promotion` rejects missing aha/surprise metadata.

## Implementation Sequence

1. Add static Des Moines game definitions in Rust.
2. Add `Game` subcommand enum under `route-cli`.
3. Implement `scenarios` and `inspect`. Done in G1-A slice 1.
4. Implement deterministic `run-season` with JSON state. Done through G1-A slice 3; scoring still pending.
5. Implement `score` against a fixture log.
6. Add L0/L1 tests.
7. Only then consider moving scenario data to TOML/JSON.

## Open Questions

| Question | Current answer |
|---|---|
| Should season logs be CSV, JSONL, or both? | Accept CSV first; JSON state for checkpoints |
| Should G1-A call `route sim scenario` internally? | Not yet; use fixed engine facts, call directly in G1-B |
| Should budget below zero fail instantly? | Yes; v0.2 uses fiscal crisis, Budget Discipline 0, and Partial Win cap |
| Should `source-challenge` affect evidence confidence? | Yes, but only through the Source request / Validated evidence split |
