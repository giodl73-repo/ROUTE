---
name: I-80 Flagship Review Packet
slug: i80-flagship-review-packet
type: report
status: reviewed
rubric_version: v1.4
author: route-packet-builder
created: 2026-07-11
updated: 2026-07-11
sources:
  - corpus/existing/i80.md
  - gaps/i80-flagship.md
  - waves/2026-07-11-i80-flagship-stabilization/panels/i80-treatment-review/R1-consolidated.md
  - docs/plans/i80-des-moines-transfer-resilience-validation.md
  - docs/reviews/i80-external-review-docket.md
---

# I-80 Flagship Review Packet

## Review Posture

**Hold and narrow.**

ROUTE has a reviewed I-80 corridor record and a bounded Des Moines topology
hypothesis. It does not have an approved design, capital recommendation,
positive ROI, guaranteed SLA, agency endorsement, or publication-ready
intervention benefit.

## Ten-Minute Review

| Time | Topic | Decision focus |
|---:|---|---|
| 1 minute | Mission and claim boundary | Is the question narrow enough? |
| 2 minutes | I-80 corridor baseline | Which measurements are usable or held? |
| 2 minutes | Des Moines gap diagnosis | Is the topology hypothesis physically credible? |
| 2 minutes | Parliament decision | Were the right blockers applied? |
| 2 minutes | Validation plan | Are the promotion gates sufficient? |
| 1 minute | External review ask | Advance validation, narrow it, or reject it? |

## Corridor Baseline

I-80 is a transcontinental Interstate from the San Francisco Bay Area to
Teaneck, New Jersey, crossing eleven states and linking western ports and
metropolitan regions with the central plains, the Chicago freight complex, and
the New York region. As an Interstate it is part of the National Highway System,
the Strategic Highway Network, and the National Highway Freight Network. Those
federal classifications establish national mobility, defense, and freight
importance; they do not by themselves prove that every segment needs the same
upgrade or that any proposed treatment has positive ROI.

| Fact | Value | Source |
|---|---|---|
| Total miles | 2917 | NHS shapefile |
| Mean AADT across matched segments | 11344 | HPMS 2023; unweighted segment mean |
| Mean truck % across matched segments | 24% | HPMS 2023; unweighted segment mean |
| Bridges | 3327 | NBI 2023 |
| Bridges poor | 2% | NBI 2023 |
| States | CA, IA, IL, IN, NE, NJ, NV, OH, PA, UT, WY | NHS shapefile |

**Band totals**: A: 17.4/50 · B: 35.0/40 · C: 25.8/40 · D: 11.6/30 · **Total: 89.8/160**

**Confidence**: mean 0.76 (Medium) · score-weighted 0.80 (Medium)

### Score boundary

- A4=9.0 remains a heuristic designation score; I-80 has no direct international border terminus and no route-level border-flow calculation is attached.
- B4=8.5 remains heuristic until STRAHNET and installation proximity are represented by source-level rows rather than a single designation note.
- C4=9.0 supports an agricultural-production-access hypothesis, not a proven long-haul export-mode share.
- The 11,344 AADT and 24% truck values are unweighted means across matched HPMS segments, not uniform corridor-wide conditions.
- A3 is a BPR-derived reliability proxy rather than observed NPMRDS PTI.
- A5 FARS, D1 FEMA, D2 DCFC, and D3 NBI values are historical and excluded
  from clean regeneration until reviewed adapters exist.
- No official-plan, construction-readiness, guaranteed-SLA, positive-ROI, or agency-endorsement claim is made.

## Current Gap Diagnosis

The bounded Des Moines gap is:

> ROUTE cannot yet determine whether the I-35/I-80 transfer node has too few
> physically independent freight-capable paths because its topology, demand,
> incident history, alternate capacity, and community impacts are not aligned
> on a common evidence definition.

This is an evidence and model-alignment gap around a plausible topology
hypothesis. It is not yet a proven infrastructure deficiency.

### Current command evidence

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

## Parliament And Editorial Decision

**Hold and narrow.**

The seven Parliament voices and three editorial gates do not approve a Des
Moines design or capital package. They retain Des Moines only as a falsifiable
validation hypothesis.

### Binding findings

1. The current scenario has zero loaded demand and proves no benefit.
2. k=0 is not a credible physical baseline for an operating interchange.
3. Historical k-values are not comparable without topology provenance.
4. The reviewed file was a validation program, not a conceptual design.
5. Snapshot observations cannot support annual probability or stored throughput
   retention.
6. Connector and diversion concepts lacked displacement, community-health,
   rural-access, climate, safety, and intermodal gates.

## Validation Gates

| Gate | Required result |
|---|---|
| Topology comparability | k-values reproduce on the same documented graph definition |
| Physical baseline | Actual interchange connectivity is at least k=1 and manually validated |
| Loaded demand | Nonzero, source-labeled demand produces a bounded closure effect |
| Intervention realism | New paths are explicit graph edges with merge/diverge constraints |
| Constructability | A node-accurate option has ROW, structure, utility, and staging screens |
| Evidence | Snapshot observations are not annualized |
| Safety | Diversion and connector geometry pass truck and local-road safety review |
| Equity | Displacement, health, and non-driving impacts are bounded and reviewed |
| Rural access | Agricultural and emergency-access effects are source-backed |
| Climate | Proposed paths do not share the governing hazard failure zone |
| Alternatives | Operations-only and intermodal options are compared |
| Null result | Rejection criteria remain acceptable outcomes |

## Explicit Holds

- No physical alignment selected.
- No connector count selected.
- No cost, benefit-cost ratio, or positive NPV.
- No construction schedule or procurement plan.
- No guaranteed throughput, PTI, SLA, or recovery outcome.
- No Iowa DOT, FHWA, MPO, carrier, or community endorsement.

## External Review Ask

Reviewers receive this packet plus `docs/reviews/i80-external-review-docket.md`. They are not asked to endorse
construction. They are asked whether the validation plan should:

1. advance as written;
2. narrow to a smaller topology, demand, or community question;
3. add a missing evidence gate; or
4. be rejected because Des Moines is the wrong I-80 flagship hypothesis.

### Decision matrix

| Result | Meaning | ROUTE action |
|---|---|---|
| Advance | The validation question is credible with listed repairs | Open a bounded evidence/geometry wave |
| Narrow | The question is useful but too broad or incorrectly measured | Amend the validation plan before execution |
| Reject | Des Moines is the wrong hypothesis or metric | Close the candidate and record the null result |

### Current review results

| Lane | Status | Decision | Rationale | Missing data | Required plan change |
|---|---|---|---|---|---|
| DOT or MPO practitioner | pending | — | — | — | — |
| Freight operator or economist | pending | — | — | — | — |
| Transportation researcher | pending | — | — | — | — |

**Roll-up decision:** pending.

The roll-up may be `advance`, `narrow`, `reject`, or `mixed`. A mixed result
must identify which validation work can proceed and which claim or mechanism
remains held.

## Regeneration

```powershell
npm run build:i80:packet
npm run check:i80:packet
```
