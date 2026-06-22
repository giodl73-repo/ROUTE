---
name: Full State System Tierization Framework
slug: full-state-system-tierization-framework
type: report
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/full-state-system-tier-role-taxonomy.csv
  - data/full-state-system-tierization-template.csv
  - data/full-state-system-tierization-iowa-sample.csv
  - data/full-state-system-tierization-slate-001.csv
  - data/full-state-system-tierization-slate-001-scorecard.csv
  - data/full-state-system-tierization-slate-002.csv
  - data/full-state-system-tierization-slate-002-scorecard.csv
  - data/full-state-system-tierization-slate-003.csv
  - data/full-state-system-tierization-slate-003-scorecard.csv
  - data/full-state-system-tierization-slate-004.csv
  - data/full-state-system-tierization-slate-004-scorecard.csv
  - data/full-state-system-tierization-slate-005.csv
  - data/full-state-system-tierization-slate-005-scorecard.csv
  - data/full-state-system-tierization-slate-006.csv
  - data/full-state-system-tierization-slate-006-scorecard.csv
  - data/full-state-system-tierization-slate-007.csv
  - data/full-state-system-tierization-slate-007-scorecard.csv
  - data/full-state-system-tierization-slate-008.csv
  - data/full-state-system-tierization-slate-008-scorecard.csv
  - data/full-state-system-tierization-slate-009.csv
  - data/full-state-system-tierization-slate-009-scorecard.csv
  - data/full-state-system-tierization-slate-010.csv
  - data/full-state-system-tierization-slate-010-scorecard.csv
  - data/state-tierization-fit-role-vector-profile-001.csv
  - data/state-tierization-fit-state-coverage-001.csv
  - data/state-source-inventory-adapter-field-contract-001.csv
  - data/state-source-inventory-adapter-row-contract-001.csv
  - data/state-source-inventory-adapter-precheck-001.csv
  - data/state-client-payload-manifest-001.csv
  - data/state-client-payload-preflight-001.csv
  - data/state-client-payload-preflight-evaluation-001.csv
  - data/state-payload-candidate-tierization-001.csv
  - data/state-payload-candidate-role-review-001.csv
  - data/state-system-failure-metric-menu.csv
  - docs/reports/state-system-failure-metrics-framework.md
  - docs/reports/state-market-system-value-add-report.md
---

# Full State System Tierization Framework

## Core Shift

The state product should not only draw selected corridors. It should tierize the
full state system.

That means every source segment in the state highway inventory receives a
service role or an explicit non-promotion reason:

- `T1` statewide trunk promise;
- `T2` regional market connector;
- `T3` rural and access connector;
- `T4` terminal and local access;
- `R` resilience overlay;
- `M` maintenance or monitor role;
- `X` outside current service scope.

The goal is full coverage of the state's system inventory, not full promotion.
Many segments should remain `M` or `X`. That is valuable because it prevents the
package from cherry-picking attractive routes and makes the tradeoff visible.

## Why This Adds Value

States already know their road labels. ROUTE adds a role audit:

| Normal state map | ROUTE full-system tierization |
|---|---|
| Shows route ownership and classification. | Shows what service job each segment is supposed to perform. |
| Highlights major corridors. | Assigns every state-system segment a role or a non-promotion reason. |
| Treats state highways as secondary to interstates. | Tests whether state highways provide redundancy, rural access, terminal access, or load shedding. |
| Separates planning maps from operational failures. | Connects roles to failure metrics and evidence gaps. |
| Produces a static picture. | Produces an editable inventory, scorecard, map, and proof docket. |

## Required Inputs

A credible full-state tierization requires:

1. State roadway inventory or source graph.
2. Segment ownership and functional class.
3. Candidate priority places: cities, gateways, terminals, hospitals, campuses,
   rural regions, freight districts, and evacuation or emergency nodes.
4. Known restrictions: bridges, truck limits, seasonal closures, work-zone
   exposure, urban bottlenecks, terminal constraints, and local access limits.
5. Failure history or source-needed placeholders.
6. Client review of what each tier should promise.

Without those inputs, ROUTE can provide a sample or template, but it cannot claim
the state system has been fully tierized.

## Role Assignment Rules

| Role | Assignment rule |
|---|---|
| T1 | Assign only when the segment carries a top statewide or gateway promise and has a plausible evidence path. |
| T2 | Assign when the segment connects regional markets, relieves a T1, or provides a credible alternate for a trunk promise. |
| T3 | Assign when the segment prevents rural isolation or connects smaller markets into the service network. |
| T4 | Assign when the segment is needed for terminal, institutional, industrial, port, airport, downtown, or local freight access. |
| R | Overlay on any role when the segment has backup, recovery, evacuation, detour, pass, bridge, or incident value. |
| M | Assign when the segment remains important to maintain but should not be promoted as a service promise. |
| X | Assign when the feature is outside the current service scope or should be excluded with a reason preserved. |

## Deliverable Shape

The full-state package should produce:

- a complete segment table using
  `data/full-state-system-tierization-template.csv`;
- a role taxonomy using `data/full-state-system-tier-role-taxonomy.csv`;
- state-specific sample rows or source-backed rows;
- a map that can filter by `T1`, `T2`, `T3`, `T4`, `R`, `M`, and `X`;
- a failure scorecard keyed to `data/state-system-failure-metric-menu.csv`;
- a proof docket separating observed, source-needed, heuristic-held, and blocked
  rows.

## Iowa Sample

`data/full-state-system-tierization-iowa-sample.csv` is an illustrative sample,
not a complete Iowa inventory. It shows the intended pattern:

- I-80 and I-35 sample rows as `T1` with `R` overlays;
- US 20 and US 30 as possible `T2` state-system redundancy and connector rows;
- US 61, US 63, and Iowa 9 as access/resilience candidates;
- DSM airport access as `T4`;
- one low-volume state segment as `M` to prove that full tierization includes
  non-promotion.

## Slate 001

`data/full-state-system-tierization-slate-001.csv` applies the same pattern to
Texas, California, Florida, and Iowa. It is still a sample, not a full
source-backed inventory, but it changes the state artifacts from corridor
selection to role assignment:

- each state has `T1`, `T2`, `T3`, `T4`, and `R` rows;
- each state includes at least one row that keeps a segment in `M` or `X` rather
  than promoting it;
- the scorecard in
  `data/full-state-system-tierization-slate-001-scorecard.csv` attaches failure
  metrics to the tierization rows.

## Slate 002

`data/full-state-system-tierization-slate-002.csv` extends the redo pattern to
New York, Illinois, Georgia, and Pennsylvania. It emphasizes legacy dense
networks where the client value is not identifying famous corridors, but
separating statewide trunks from state/US-route redundancy, rural access,
terminal access, resilience overlays, and non-promoted maintained routes.

## Slate 003

`data/full-state-system-tierization-slate-003.csv` extends the redo pattern to
Ohio, North Carolina, Michigan, and Washington. It emphasizes state systems where
full value comes from recognizing secondary but critical roles: Appalachian and
river access, barrier-island/coastal access, Upper Peninsula continuity, and
mountain-pass/port resilience.

## Slate 004

`data/full-state-system-tierization-slate-004.csv` extends the redo pattern to
Arizona, Colorado, Tennessee, and Missouri. It emphasizes systems where the
interstate spine is obvious but the product value is in assigning roles to
border access, mountain alternates, river terminals, Ozark/southeast coverage,
rural continuity, and maintained/non-promoted segments.

## Slate 005

`data/full-state-system-tierization-slate-005.csv` extends the redo pattern to
Minnesota, Wisconsin, Virginia, and Louisiana. It emphasizes winter operations,
Great Lakes and Mississippi River access, Mid-Atlantic port/evacuation risk, and
Gulf/coastal energy resilience while preserving maintenance-only and
outside-scope rows.

## Slate 006

`data/full-state-system-tierization-slate-006.csv` extends the redo pattern to
Oregon, Alabama, Kentucky, and Massachusetts. It emphasizes coastal isolation,
Columbia Gorge and Gulf access, state parkway redundancy, air-cargo terminal
access, dense New England constraints, and explicit outside-scope rows.

## Slate 007

`data/full-state-system-tierization-slate-007.csv` extends the redo pattern to
Indiana, South Carolina, Maryland, and Nevada. It emphasizes crossroads
redundancy, coastal evacuation, compact bridge/tunnel and port systems, desert
connectivity, sparse-service access, and explicit maintenance or exclusion rows.

## Slate 008

`data/full-state-system-tierization-slate-008.csv` extends the redo pattern to
Oklahoma, Arkansas, Mississippi, and New Jersey. It emphasizes plains
crossroads, Delta and Ozark access, Gulf and river freight, dense turnpike/port
systems, shore evacuation, and explicit maintenance or exclusion rows.

## Slate 009

`data/full-state-system-tierization-slate-009.csv` extends the redo pattern to
Utah, New Mexico, Idaho, and Maine. It emphasizes intermountain trunks, border
and airport access, sparse rural continuity, winter pass recovery,
forest/agricultural access, and explicit maintenance rows.

## Slate 010

`data/full-state-system-tierization-slate-010.csv` extends the redo pattern to
Kansas, Nebraska, North Dakota, and Montana. It emphasizes plains and northern
tier trunks, agriculture and energy access, sparse rural continuity, mountain
and winter recovery, terminal access, and explicit maintenance rows.

## Fit Kernel 001

`data/state-tierization-fit-role-vector-profile-001.csv` and
`data/state-tierization-fit-state-coverage-001.csv` convert the slate set into a
bounded role-assignment fit profile. The profile extracts six reusable signal
families: statewide trunk/gateway, regional redundancy/load shedding, rural
access continuity, terminal/local access, resilience/recovery exposure, and
maintenance/non-promotion.

This is a heuristic fitting layer. It shows that ROUTE can represent the
complexity consistently across the current state samples, but it still requires
source road inventory, client priority nodes, and evidence review before any
state-specific role is promoted.

## Source Inventory Adapter Contract 001

`data/state-source-inventory-adapter-field-contract-001.csv`,
`data/state-source-inventory-adapter-row-contract-001.csv`, and
`data/state-source-inventory-adapter-precheck-001.csv` define the generic source
package a state, consultant, port, airport, MPO, or private infrastructure
operator must provide before ROUTE applies the fit kernel to a real inventory.

The contract covers segment identity, topology, jurisdiction, road class,
priority nodes, alternate-route relationships, restrictions, observed failures,
terminal access, and non-promotion reasons. It keeps the first ingest posture at
`source-needed` and blocks full-inventory, official-tier, SLA, ROI, approval,
construction, validation, and public-readiness claims until source review is
complete.

## Client Payload Scaffold 001

`data/state-client-payload-manifest-001.csv` and
`data/state-client-payload-preflight-001.csv` package the adapter contract into
client-fillable templates for segment inventory, priority nodes, terminal
access, restriction/failure evidence, and non-promotion coverage.

The scaffold is the first handoff shape for a state, consultant, authority, or
private operator. It validates only template readiness. Client data remains
`not-provided` until a real payload is filled, source references are attached,
and the adapter preflight is rerun against those rows.

## Client Payload Preflight Evaluation 001

`data/state-client-payload-preflight-evaluation-001.csv` reads the scaffolded
payload templates and checks manifest completeness, segment shape, priority-node
references, terminal-access references, restriction/failure references, and
non-promotion references. It deliberately keeps source custody and promotion
readiness held because the rows are sample templates, not a filled client
payload.

## Payload Candidate Tierization 001

`data/state-payload-candidate-tierization-001.csv` and
`data/state-payload-candidate-role-review-001.csv` apply the fit kernel to the
generic payload sample and emit candidate `T1`, `T2`, `T4`, and `M` rows. Every
row remains `source-needed`, every role requires review, and the output is a
candidate transform only until a filled client payload replaces the sample rows.

## Evidence Boundary

This framework does not claim any state route has an official ROUTE tier, that a
state has accepted the role assignment, that a state system is fully scored, or
that any SLA, construction, cost, ROI, funding, compliance, endorsement,
validation, public-readiness, or approval claim is established.
