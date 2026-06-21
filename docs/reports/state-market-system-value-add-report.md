---
name: State Market-System Value-Add Report
slug: state-market-system-value-add-report
type: report
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/international-market-system-map-v1.csv
  - data/international-market-system-map-export-001.csv
  - tools/render_international_market_system_maps.py
  - docs/reviews/state-market-system-slate-001.md
  - docs/reviews/state-market-system-slate-002.md
  - docs/reviews/state-market-system-slate-003.md
  - docs/reviews/state-market-system-slate-004.md
  - docs/reviews/state-market-system-slate-005.md
---

# State Market-System Value-Add Report

## Answer

The state market-system maps are valuable only if they are treated as discovery
and decision surfaces, not as claims that ROUTE has found roads a DOT does not
already know.

Most states already know their major corridors, ports, metro anchors, border
gateways, mountain passes, and rural coverage gaps. ROUTE's product value is the
conversion from familiar geography into a structured service portfolio:

- a draft promise hierarchy;
- a visible distinction between trunk, connector, and access claims;
- a claim-hold ledger for evidence, authority, costs, legal SLA, construction,
  eligibility, validation, endorsement, and public-readiness;
- an editable client workshop surface where the state can change the priorities
  before ROUTE regenerates the package.

## What States Already Know

| Known state knowledge | Why a map alone is weak |
|---|---|
| Largest cities and metro pairs | The state does not need ROUTE to identify Denver, Phoenix, Nashville, Detroit, Seattle, or St. Louis. |
| Named corridors and gateways | DOTs already manage interstates, bridges, ports, airports, borders, and freight districts. |
| Chronic resilience issues | Mountain passes, coastal evacuation, river crossings, winter closures, and urban bottlenecks are already familiar. |
| Rural access pressure | States know which regions feel under-served, even when exact service promises are not formalized. |

## What ROUTE Adds

| ROUTE artifact | Value beyond a normal corridor map |
|---|---|
| Tier assignment | Forces each link to declare whether it is a trunk promise, market connector, or access feeder. |
| Service promise text | Turns geography into a proposed operating purpose: reliability, resilience, terminal access, rural coverage, port access, border access, institutional access, or freight continuity. |
| Evidence posture | Prevents the map from implying official priority, construction readiness, legal SLA, numeric ROI, funding eligibility, compliance, endorsement, or validation. |
| Market-layer inventory | Lets a client see whether the draft overweights metro growth, freight, ports, rural access, resilience, or terminal feeders. |
| Editability | Gives a state a practical workshop question: "What are your top places to connect, and what promise should each tier carry?" |
| Failure metrics | Converts known pain into measurable questions: interchange exposure, alternate-route penalty, interstate overreliance, state-system redundancy, terminal friction, rural isolation, and recovery evidence gaps. |
| Full-system tierization | Forces every state-system segment to receive a service role or a non-promotion reason, so the package does not cherry-pick only attractive corridors. |

## What Should Be Clear In A Client Meeting

A state should not leave the first meeting thinking ROUTE has produced an
official plan. It should leave with a clearer decision table:

| Client question | ROUTE output |
|---|---|
| Which places matter most? | Candidate nodes and city pairs. |
| What promise should each connection carry? | Reliability, resilience, freight, port, border, rural access, terminal access, or institutional access labels. |
| Which promises are top tier? | T1/T2/T3 draft hierarchy. |
| What evidence is missing? | Source-needed rows and held-claim posture. |
| What changes if priorities shift? | Regenerated map, SLA slate, proof docket, and review packet. |
| Where is the current system failing or fragile? | A bounded metric scorecard with evidence posture, not unsupported deficiency claims. |

## Failure Metrics Layer

The map package needs a scorecard layer because a state-facing sale is weak if
it only says "these corridors matter." The stronger offer is:

> Give us your priority places and unacceptable failures. ROUTE will show where
> the current network appears overconcentrated, where alternates impose too much
> service penalty, where state highways may need a redundancy role, and where
> evidence is missing before any SLA can be promoted.

The metric definitions are in
`docs/reports/state-system-failure-metrics-framework.md` and
`data/state-system-failure-metric-menu.csv`.

The full-inventory tierization pattern is defined in
`docs/reports/full-state-system-tierization-framework.md`. That layer is what
turns ROUTE from a selected-corridor story into a state-system audit.

## Slate 005 Examples

| State | Obvious knowledge | ROUTE value-add question |
|---|---|---|
| Colorado | The state already knows the Front Range and I-70 mountain corridor matter. | Which promise is highest priority: Front Range reliability, mountain-pass resilience, airport access, Western Slope coverage, plains freight, or southern rural access? |
| Tennessee | The state already knows Memphis, Nashville, Knoxville, and Chattanooga anchor the network. | Should the first package optimize Memphis freight, Nashville hub reliability, east-state redundancy, Tri-Cities/Appalachian access, or west Tennessee coverage? |
| Missouri | The state already knows KC, Columbia, and St. Louis define the I-70 spine. | Should the system prioritize KC-STL trunk reliability, Columbia/capital access, Springfield/Ozarks, Mississippi river access, Joplin gateway, or southeast rural coverage? |

## Product Implication

The sellable product is not a static map. It is an editable service-network
workbench:

1. A client selects priority places and promise types.
2. ROUTE proposes T1/T2/T3 links and held evidence labels.
3. The client adjusts tiers and adds or removes places.
4. ROUTE regenerates maps, SLA candidates, resilience questions, proof dockets,
   and presentation material.
5. Any stronger claim remains blocked until source rows, authority, cost, legal,
   construction, validation, and endorsement gates are satisfied.

## Held Claims

This report does not claim official state priority, construction readiness,
legal SLA, cost estimate, numeric ROI, funding eligibility, compliance,
endorsement, external validation, public readiness, or state approval.
