# Des Moines Diamond CLI Playtest 001

Date: 2026-05-09  
Scenario: `des-moines-diamond`  
Scenario version: G0 v0.2  
CLI slice: G1-A list/inspect  
Evidence level: Heuristic

## Purpose

Verify that the first executable `route game` slice represents the paper scenario faithfully enough to support the next run-season implementation.

This is not a full game playthrough. It is a terminal inspection playtest.

## Commands Run

```powershell
cargo test -p route game::
cargo run -q -p route -- game scenarios
cargo run -q -p route -- game inspect des-moines-diamond
cargo run -q -p route -- sim scenario des-moines-interchange
cargo run -q -p route -- sim scenario des-moines-interchange --intervention
cargo run -q -p route -- diamond I35xI80
```

## Results

| Check | Result |
|---|---|
| Game unit tests | Pass; 5 tests |
| Scenario list includes Des Moines | Pass |
| Scenario list separates evidence level and publication gate | Pass |
| Inspect output includes v0.2 project cards | Pass |
| Inspect output rejects old `evidence-acquisition` card | Pass via test |
| Inspect output uses independent-transfer-path language | Pass |
| Inspect output prints publication gate separately | Pass |
| Unknown scenario is rejected with available ids | Pass |
| Baseline scenario hook runs | Pass |
| Intervention scenario hook runs | Pass |
| Diamond analyzer recognizes `I35xI80` | Pass |

## Live Engine Values Observed

Baseline scenario:

| Metric | Value |
|---|---:|
| Baseline throughput | 86,671 vph |
| Incident throughput | 83,423 vph |
| Incident PTI | 1.17 |
| T90 | 0.9h |
| LOS-F edges | 7 |

Intervention scenario:

| Metric | Value |
|---|---:|
| Baseline throughput | 86,671 vph |
| Incident throughput | 83,423 vph |
| Intervention throughput | 86,671 vph |
| Incident PTI | 1.37 |
| Intervention PTI | 1.36 |
| T90 | 0.9h |

Diamond analyzer:

| Metric | Value |
|---|---:|
| Anchor | I-35/I-80, 41.66 N, 93.57 W |
| Current k-connectivity | 0 |
| Single point of failure | yes |
| Connectors needed for k >= 3 | 3 |
| Estimated cost | $0.75B |

## Player/Reviewer Reading

The CLI succeeds as an inspection surface. A reviewer can see the scenario id, phase, evidence level, standards, engine hook, publication lock, project cards, event cards, evidence cards, win bands, and live ROUTE hooks without opening the paper document.

The output also keeps the core lesson intact: general-purpose widening improves capacity but does not add independent transfer paths. The first executable slice therefore preserves the G0 topology aha.

## Findings

| Finding | Severity | Action |
|---|---|---|
| `route game inspect` is long but readable for a reviewer | Low | Keep stable for now; add JSON only when browser/fixtures need it |
| Engine hook values still make PTI an unsafe headline claim | Expected | Keep publication gate locked and teaching claim focused on topology/throughput |
| CLI playtest is not a full season mutation test | Expected | Implement `run-season` next |

## Decision

G1-A list/inspect passes this terminal playtest.

Next implementation step:

1. Add deterministic default state.
2. Add `route game run-season des-moines-diamond`.
3. Reject unaffordable projects with a reason sentence.
4. Preserve append-only session-log shape for QUEST continuity.
