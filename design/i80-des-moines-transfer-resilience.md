---
name: I-80 Des Moines Transfer Resilience Package
slug: i80-des-moines-transfer-resilience
type: design-proposal
status: draft
rubric_version: v1.4
author: copilot
created: 2026-07-11
updated: 2026-07-11
sources:
  - gaps/i80-flagship.md
  - data/t1-intersection-failures.csv
  - data/t1-evidence-windows.csv
  - data/pressure-test-scenarios.csv
  - data/throughput-proof-matrix.csv
  - crates/route-sim/src/scenarios/des-moines-interchange.toml
corridor:
  termini: ["I-35 north/south approaches", "I-80 east/west approaches"]
  states: ["IA"]
  approx_miles: 50
  designation: "I-35/I-80 Des Moines analysis zone"
  classification: proposed
---

# I-80 Des Moines Transfer Resilience Package

## Status

**Selected for Parliament review; capital decision held.**

The proposal defines a mechanism and validation program. It does not select a
physical alignment, estimate cost, claim positive ROI, or assert agency support.

## Problem Statement

The I-35/I-80 node may concentrate national east-west and north-south transfer
through too few independent freight-capable paths. Current ROUTE artifacts do
not yet agree on the node's k-connectivity, and the current scenario run has no
loaded demand. The design question is therefore whether an independent-path
package can be shown to preserve transfer and recovery under a validated
closure, not how quickly construction should begin.

## Package

### 1. Geometry And Freight-Path Validation

- Inventory existing limited-access and freight-capable transfer paths inside
  the analysis zone.
- Reconcile current k=0 output with older k=1 documentation.
- Identify whether any apparent path is a TIGER snapping artifact, shares the
  same failure zone, or lacks truck-suitable geometry.

### 2. Independent Transfer Path Concept

- Develop conceptual connector options only after the geometry inventory.
- Require proposed paths to be physically independent of the interchange-zone
  failure.
- Treat the current `three connectors needed` output as a hypothesis to test,
  not an alignment or construction quantity.

### 3. Operations Companion

- Define early diversion and dynamic routing rules.
- Sequence work zones so construction does not create the failure being
  mitigated.
- Keep I-235 and local roads explicitly bounded by truck suitability, capacity,
  and community-impact review.

### 4. Evidence Package

- Extend Iowa 511 observations or obtain archive history.
- Add direct travel-time/PTI evidence for normal, closure, diversion, and
  recovery windows.
- Rebuild a nonzero, source-labeled demand fixture.
- Run sensitivity across demand, closure duration, connector capacity, and
  alternate availability.

## Acceptance Gates

| Gate | Required result |
|---|---|
| Topology provenance | Compared k-values use documented graph versions, node pairs, zone boundaries, route filters, and freight eligibility rules |
| Geometry | Existing and proposed paths are manually validated and freight-capable |
| Independence | At least one proposed transfer path does not share the modeled failure zone |
| Loaded stressor | Calibrated demand produces a bounded degradation under closure |
| Sensitivity | The package changes throughput, PTI, or recovery over stated assumptions |
| Recovery | The existing 80% within four hours standard is tested, not assumed |
| Evidence | Snapshot observations are not annualized without a stable window |
| Community | Local diversion and construction impacts receive stakeholder review |

## Falsifiers

Reject or redesign the package if:

- manual validation finds adequate independent freight paths already exist;
- current and historical k-values cannot be reproduced on a comparable topology definition;
- the k=0 result is primarily a graph construction artifact;
- calibrated demand does not produce a binding transfer failure;
- feasible connectors share the same hazard or closure zone;
- local diversion creates unacceptable safety, access, or community impacts;
- operations-only treatment performs as well as capital options across the
  accepted sensitivity range.

## Explicit Holds

- No cost or benefit-cost ratio.
- No positive NPV.
- No construction schedule or procurement plan.
- No guaranteed throughput, PTI, SLA, or recovery outcome.
- No Iowa DOT, FHWA, MPO, carrier, or community endorsement.
- No claim that three connectors are physically feasible or required.

## Parliament Question

Should ROUTE advance an independent-transfer-path concept at Des Moines for
geometry and demand validation, or does the current evidence require a
different mechanism, narrower scope, or rejection?
