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

## Evidence Boundary

This framework does not claim any state route has an official ROUTE tier, that a
state has accepted the role assignment, that a state system is fully scored, or
that any SLA, construction, cost, ROI, funding, compliance, endorsement,
validation, public-readiness, or approval claim is established.
