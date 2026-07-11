---
name: I-80 Flagship Gap Decision
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
---

# I-80 Flagship Gap Decision

## Decision

Select the **Des Moines independent-transfer-path package** as the I-80
flagship candidate for Parliament and editorial review.

This is a selection for design review, not a construction, funding, ROI, or
publication recommendation. The current evidence supports testing the
mechanism; it does not prove the capital intervention.

## Current Command Evidence

Commands run on 2026-07-11:

```text
cargo run -q -p route -- sim scenario des-moines-interchange
cargo run -q -p route -- sim scenario des-moines-interchange --intervention
cargo run -q -p route -- diamond I35xI80
```

The scenario commands found zero demand pairs and reported zero baseline,
incident, and intervention throughput. They therefore provide no current
benefit evidence. The topology command found the curated I-35/I-80 node,
reported current k-connectivity of 0, and reported three connectors needed to
reach the configured k>=3 target.

Older game documents record nonzero throughput outputs, while an older research
paper reports different locations and k-values for I-35/I-80. Those values are
not used as current evidence. The inconsistency is part of the validation gap.

## Evidence Classes

| Surface | Current evidence | Classification | Decision use |
|---|---|---|---|
| Chicago Southland I-80/I-94 | ATRI bottleneck seed; classified as a local topology chokepoint | observed seed | Confirms that corridor averages hide local stress; insufficient treatment model |
| Donner Pass | Bound weather scenario; current catalog records no throughput delta | executable heuristic | Strong resilience hypothesis, but not loaded enough to select a capital treatment |
| Des Moines I-35/I-80 | 29 freight-relevant Iowa 511 rows in a snapshot-only window | observed sample / not annualizable | Supports continued investigation, not annual failure probability |
| Des Moines scenario | Bound closure and intervention fixture; current run has zero demand pairs | executable but unloaded | No benefit claim allowed |
| Des Moines topology | Curated node is found; current analyzer reports k=0 and three connectors needed | model/geometry hypothesis | Defines a falsifiable independent-path treatment |
| Corridor-wide managed lanes | No corridor-wide segment diagnosis or calibrated demand distribution | unsupported generalization | Reject as the flagship treatment |

## Treatment Comparison

| Candidate | Mechanism | Evidence strength | Falsifier | Decision |
|---|---|---|---|---|
| Corridor-wide I-80 upgrade | Apply one program across all 2,917 miles | Weak; corridor measurements are incomplete and highly aggregated | Segment diagnosis shows different binding problems by region | Reject |
| Chicago Southland operations or lanes | Relieve a known local freight topology chokepoint | Observed seed, but no validated directional demand or treatment sensitivity | Segment-level flow shows the node is not structurally binding | Hold |
| Donner resilience package | Add egress, operations, alternate road, or intermodal capacity | Strong story and bound fixture; current model has no loaded throughput effect | Observed closures and alternate capacity show Donner is not binding | Hold |
| Des Moines independent transfer paths | Add physically independent transfer options with operations companions | Most falsifiable bounded mechanism; current demand and geometry evidence remain inadequate | Manual geometry shows adequate existing paths, or calibrated closure demand shows no binding transfer loss | Select as review hypothesis |
| Des Moines widening-only package | Add approach capacity without independent paths | Does not address the stated topology mechanism | Independent-path analysis shows capacity, not path count, is binding | Reject as primary treatment |

## Decisive Gap

The current flagship gap is:

> ROUTE has identified a potentially fragile I-35/I-80 transfer node, but its
> geometry, demand, incident history, and alternate freight capacity are not yet
> aligned well enough to prove either the failure magnitude or the benefit of a
> connector package.

This is narrower and more defensible than claiming that I-80 as a whole lacks
capacity or resilience.

## Selection Rationale

Des Moines is selected as the review hypothesis because:

1. It is a bounded node on I-80 rather than a corridor-wide generalization.
2. It has an existing source-acquisition path and explicit snapshot-history
   guard.
3. It has a named topology mechanism: independent transfer paths.
4. Its zero-demand scenario and inconsistent topology outputs make the current
   hypothesis directly falsifiable rather than benefit-ready.
5. A null result is acceptable: geometry or calibrated demand may reject the
   package.

## Required Promotion Evidence

- Reconcile the k=0 current analyzer result with older k=1 research claims.
- Record the graph version, node pair, zone boundary, route filters, and freight
  eligibility rules for every compared k-value before treating them as
  comparable.
- Manually validate existing and feasible freight-capable paths inside the
  analysis zone.
- Restore or rebuild a nonzero, source-labeled demand fixture.
- Obtain an archive or repeated observation window before annualizing failure.
- Join direct PTI or travel-time evidence for baseline, closure, and recovery.
- Demonstrate intervention sensitivity across demand and connector assumptions.

Until those steps close, the selected package remains `selected-for-review /
capital-held`.
