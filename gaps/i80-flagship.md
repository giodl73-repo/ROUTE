---
name: I-80 Flagship Gap Diagnosis
slug: i80-flagship-gap
type: gap-analysis
status: reviewed
rubric_version: v1.4
author: copilot
created: 2026-07-11
updated: 2026-07-11
sources:
  - corpus/existing/i80.md
  - data/pressure-test-scenarios.csv
  - data/throughput-proof-matrix.csv
  - data/t1-intersection-failures.csv
  - data/t1-evidence-windows.csv
  - gaps/bottleneck.md
  - docs/game/des-moines-diamond-g0.md
  - docs/game/donner-weather-closure-g0.md
  - research/publications/B.4+t1-intersection-resilience/sections/03-k-connectivity.tex
---

# I-80 Flagship Gap Diagnosis

## Scope

This artifact identifies and bounds I-80 gap candidates. It does not select a
physical treatment. Parliament review and the treatment hold are recorded in
`waves/2026-07-11-i80-flagship-stabilization/panels/i80-treatment-review/`.

## Current Command Evidence

Commands run on 2026-07-11:

```text
cargo run -q -p route -- sim scenario des-moines-interchange
cargo run -q -p route -- sim scenario des-moines-interchange --intervention
cargo run -q -p route -- diamond I35xI80
```

The scenario commands found zero demand pairs and reported zero baseline,
incident, and intervention throughput. They provide no current benefit,
retention, PTI, or recovery evidence.

The topology command found the curated I-35/I-80 node and reported k=0 with
three connectors needed to reach the configured k>=3 target. A physically open
interchange cannot be treated as k=0 without resolving graph definition and
snapping errors. The connector count is therefore not design evidence.

Older game and research artifacts contain nonzero throughput and different
locations or k-values. Those values are not comparable until graph version,
node pair, zone boundary, route filters, and freight eligibility rules are
recorded.

## Gap Candidates

| Location | Gap type | Current evidence | Blocking condition |
|---|---|---|---|
| Chicago Southland I-80/I-94 | Local freight topology/congestion | ATRI observed bottleneck seed | Directional demand and treatment sensitivity are not validated |
| Donner Pass | Weather resilience and alternate capacity | Bound scenario and documented no-delta limitation | Mountain demand, truck-capable alternates, rail diversion, and observed closures are missing |
| Des Moines I-35/I-80 | Potential transfer-path concentration | 29 snapshot rows, bound scenario, contradictory topology outputs | Evidence is not annualizable; scenario demand is zero; topology is not physically comparable |
| I-80 corridor-wide | Multiple nonuniform problems | Aggregated corridor measurements | No evidence supports one uniform treatment across the corridor |

## Diagnosed Flagship Gap

The bounded Des Moines gap is:

> ROUTE cannot yet determine whether the I-35/I-80 transfer node has too few
> physically independent freight-capable paths because its topology, demand,
> incident history, alternate capacity, and community impacts are not aligned
> on a common evidence definition.

This is an evidence and model-alignment gap around a plausible topology
hypothesis. It is not yet a proven infrastructure deficiency.

## Required Closure Evidence

- Comparable and manually validated k-connectivity.
- A physically correct baseline of k>=1.
- Nonzero source-labeled freight demand.
- Stable incident history or an explicit archive blocker.
- Direct travel-time/PTI evidence.
- Truck-suitable alternate capacity.
- Climate and shared-failure analysis.
- Equity, displacement, rural-access, and non-driving impact analysis.
- Operations-only and intermodal comparisons.

## Null Result

The gap closes as a null result if corrected topology shows adequate independent
paths, calibrated demand does not produce a binding transfer failure, or
operations-only measures perform as well as a feasible capital concept.
