---
name: ROUTE ROI and Cost Framework
slug: route-roi-cost-framework
type: report
status: draft
rubric_version: v1.0
author: copilot
created: 2026-06-16
updated: 2026-06-16
sources:
  - README.md
  - docs/decks/route-one-page.md
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
  - docs/vtrace/REQUIREMENTS.md
  - docs/vtrace/CODE_RIGOR.md
  - docs/sla-promise-portfolio.md
  - docs/route-architecture.md
  - docs/STANDARDS_EVALUATION.md
  - docs/tier-optimizer-design.md
  - docs/map-publication-scope.md
---

# ROUTE ROI and Cost Framework

## Purpose

This report defines how ROUTE should talk about ROI and costs before it claims
ROI or costs.

The communications goal is simple: funders need to see that Interstate 2.0 can
become an investable program, but ROUTE must not invent a benefit-cost number
before the sources, assumptions, price year, uncertainty treatment, and review
lanes are ready.

This framework is therefore an evidence contract. It says what would count, how
it would be traced, and what must stay blocked until the evidence exists.

## Executive Frame

ROUTE should sell ROI as a staged question, not a premature answer:

```text
national service promise
  -> candidate investment package
  -> benefit classes
  -> cost classes
  -> source and confidence labels
  -> scenario tests
  -> stakeholder review
  -> fundable option
```

The near-term ask is not "fund this exact corridor because the ROI is proven."
The near-term ask is "fund the planning, evidence, demo, and decision package
that can turn corridor ideas into comparable investment options."

## What Counts As Value

ROUTE's value model should be broad enough for politicians, states, industry,
and communities, but disciplined enough that every value claim can be reviewed.

| Benefit Class | Plain-English Value | Example Evidence Needed | Status Rule |
|---|---|---|---|
| Freight reliability | Shippers and carriers get more predictable travel windows. | Observed or modeled travel-time reliability, SLA scenario outputs, source confidence labels. | Heuristic until direct reliability evidence and sensitivity ranges exist. |
| Throughput and bottleneck relief | The network carries more useful movement under normal and stressed conditions. | Max-flow, scenario throughput, demand assumptions, bottleneck diagnostics. | Heuristic until sensitivity and commodity/market limits are explicit. |
| Resilience | Closures, weather, incidents, and port disruptions hurt less and recover faster. | Scenario library, alternate-route capacity, failure-rate/source ledgers, recovery gates. | Held or heuristic until stressor and recovery acceptance gates are sourced. |
| Rural and production access | Agriculture, smaller markets, and production zones keep visible access to the national system. | T3/T4 access obligations, service-zone maps, stakeholder review, source-backed anchors. | Must show access obligation and affected zone before promotion. |
| Port, border, and intermodal performance | Gate, yard, warehouse, rail, and border approaches become part of the plan. | Terminal/access ledgers, port/border sources, T4 access treatment. | Blocked where terminal-access proof is still excluded. |
| Safety and operations | Interchange, lane, staging, rest, and parking improvements reduce operational risk. | Site geometry, incident history, truck parking/rest/charging inventories, review lanes. | Cannot claim safety benefit without source-backed mechanism. |
| Driver workforce quality | Relay hubs can support regional shifts and a premium freight chauffeur role. | Relay operating model, labor assumptions, driver schedule scenarios, stakeholder review. | Concept until operating assumptions are tested. |
| EV and autonomous readiness | Future technology gets planned corridors, charging nodes, and supervised handoff points. | Charging demand, grid/site readiness, policy constraints, AV operating assumptions. | Planned or heuristic until deployment assumptions are sourced. |
| Economic competitiveness | States and regions can compete through better service promises and logistics access. | Industry use cases, market access analysis, state views, scenario comparisons. | Narrative only until benefits are monetized or bounded. |
| Community and environmental outcomes | Concerns are visible early enough to shape or hold options. | Environmental/community-health review, non-driving access review, mitigation and dissent records. | Must preserve dissent and cannot be reduced to a single net benefit without review. |

## What Counts As Cost

Cost should be treated as a portfolio stack, not a single construction number.

| Cost Class | What It Includes | Required Before Use |
|---|---|---|
| Planning and evidence | Studies, source acquisition, modeling, scenario design, role review, demo preparation. | Work scope, owner, validation level, and deliverable list. |
| Right-of-way and delivery | Land, utility, permitting, environmental review, agency delivery constraints. | State DOT / delivery-feasibility review and source-backed assumptions. |
| Capital construction | Pavement, lanes, interchanges, bridges, terminals, hubs, charging, staging, hardening. | Cost basis, price year, unit sources, contingency, and excluded items. |
| Operations and maintenance | Maintenance, inspection cadence, incident management, staffing, hub operations, systems. | Lifecycle horizon, operating owner, maintenance standard, and funding assumption. |
| Technology and controls | Sensors, communications, charging systems, dispatch, data systems, AV support, cybersecurity. | System boundary, procurement assumption, lifecycle replacement assumption. |
| Community mitigation | Noise, air quality, safety, access, relocation, community benefits, local design changes. | Affected stakeholder lanes and mitigation scope. |
| Financing and program risk | Inflation, schedule risk, grant match, debt service, procurement risk, escalation. | Price-year rule, uncertainty range, and scenario treatment. |

Every cost claim must name the price year and whether it is capital-only,
lifecycle, public-sector, private-sector, or total social cost.

## The ROUTE ROI Equation

ROUTE should not publish one master ROI number. It should publish a comparable
benefit-cost frame with evidence labels.

```text
ROI posture =
  benefit classes with source/confidence labels
  minus cost classes with price-year and scope labels
  tested across scenarios
  reviewed by required roles
  reported with blockers and excluded benefits
```

For external use, each option should report:

| Field | Required Content |
|---|---|
| Option ID | Stable package, corridor, hub, standard, or scenario identifier. |
| Service promise | T1/T2/T3/T4 promise affected, if any. |
| Bundle identity | Stable bundle or accepted transitional identity surface. |
| Benefit classes | Which benefits are counted, held, excluded, or narrative-only. |
| Cost classes | Which costs are counted, held, excluded, or outside scope. |
| Evidence posture | Implemented, heuristic, planned, held, source-needed, or confidence-limited. |
| Scenario coverage | Which adverse or operating scenarios were tested. |
| Stakeholder review | Which roles reviewed the claim and what changed. |
| Decision posture | Fund planning, fund pilot, fund study, hold, downgrade, or reject. |

## State Value Proposition

States should see ROI as a way to make their priorities more fundable, not as a
black-box score imposed from outside.

| State Need | ROUTE ROI/Cost Translation |
|---|---|
| "Why should our project matter nationally?" | Connect the state ask to freight reliability, access, resilience, SLA tier, or relay/hub value. |
| "Can we shape the plan?" | Treat state requirements as model inputs that can change routes, stops, hubs, staging, or claim status. |
| "Can this fit a funding window?" | Split a large vision into planning, pilot, hub, safety, charging, interchange, and corridor packages. |
| "What is not ready?" | Keep delivery, right-of-way, source, environmental, and maintenance blockers visible instead of hiding them. |
| "How do we avoid losing local concerns?" | Require state DOT, community, environmental, rural, and non-driving access review where claims affect them. |

## Industry Value Proposition

Industry should see the ROI frame as an operating model for reliability, not a
generic infrastructure wishlist.

| Industry Need | ROUTE ROI/Cost Translation |
|---|---|
| Predictable delivery windows | Tie benefits to T1/T2 SLA windows, relay schedules, and reliability evidence. |
| Better bottleneck visibility | Separate interchange, port, border, terminal, and regional relief mechanisms. |
| Practical EV transition | Plan charging and service nodes around actual freight movement and relay hubs. |
| Driver workforce improvement | Treat relay hubs as workforce and service-quality infrastructure, not only pavement. |
| Staged technology deployment | Move from human relay operations to supervised AV trunk segments only where assumptions are explicit. |
| Input into requirements | Give carriers, shippers, ports, warehouses, agriculture, and manufacturers a requirement intake that can change the plan. |

## Relay Hub ROI Logic

Relay hubs are where the vision becomes operational and where ROI categories can
be made tangible.

| Hub Function | Value Mechanism | Cost Mechanism | Evidence Needed |
|---|---|---|---|
| Crew exchange | Regional shifts, less fatigue, better schedule quality. | Hub staffing, security, dispatch, facilities. | Operating schedule assumptions and labor review. |
| Charging depot | Predictable heavy-duty charging and service. | Grid connection, chargers, land, maintenance. | Charging demand, grid/site feasibility, utilization assumptions. |
| Transfer point | Load, trailer, driver, and future AV handoffs. | Yard design, equipment, dwell-time management. | Dwell-time model, safety/operations review. |
| Marketplace node | Earlier load matching and less idle time. | Data systems, governance, carrier integration. | Carrier/shipping use cases and utilization model. |
| Resilience checkpoint | Staging during incidents, closures, or reroutes. | Emergency operations, storage, redundancy. | Scenario evidence and recovery gates. |
| State development asset | Jobs, services, and visible infrastructure package. | Local access, mitigation, permitting, community benefits. | State/community review and delivery posture. |

## Evidence Gate Before Any ROI Claim

No ROI or cost claim should move from "framework" to "claim" unless this gate is
closed.

| Gate Item | Required |
|---|---|
| Price year | Named and applied consistently. |
| Time horizon | Named for capital, lifecycle, and operating costs. |
| Geographic scope | State, corridor, region, hub, or national scope is explicit. |
| Identity scope | Bundle/segment/stop/hub identifiers are stable or held. |
| Benefit inclusion | Included, excluded, held, and narrative-only benefits are listed. |
| Cost inclusion | Included, excluded, held, and private/public split are listed. |
| Source posture | Sources, source gaps, live-snapshot limits, and access-gated data are visible. |
| Uncertainty | Ranges, scenarios, or sensitivity rules are included. |
| Negative case | The report can show weak, marginal, or failed ROI without hiding it. |
| Role review | Freight Economist, Numeracy Checker, Citation Auditor, State DOT, affected stakeholder lanes, and Scope Keeper review are recorded. |

## Report Templates To Produce Next

This framework supports three funder-facing artifacts.

| Artifact | Job | Allowed Claims |
|---|---|---|
| ROI/cost explainer | Explain the benefit and cost model without numbers. | Framework, categories, gate, and next evidence steps. |
| State value sheet | Show what a state gets from the planning process. | State-specific requirements and possible staged package types, not ROI totals. |
| Industry value sheet | Show how carriers, shippers, ports, warehouses, agriculture, and EV/AV partners can use ROUTE. | Operating value logic and intake process, not monetized benefits. |

## Current Claim Boundary

This draft does not claim:

- positive ROI for any corridor, hub, standard, or map;
- construction readiness;
- official agency endorsement;
- statutory, environmental, safety, or delivery compliance;
- final cost estimates;
- final benefit-cost ratios;
- publication-grade SLA or throughput proof.

It does claim that ROUTE now has a controlled structure for future ROI/cost
work, and that any future ROI claim should be weaker than the gate unless the
gate is explicitly closed.

## Role Review Notes

| Role Lens | Finding | Disposition |
|---|---|---|
| Scope Keeper | The report defines an ROI/cost evidence contract and does not select, fund, or prescribe any construction project. | pass |
| Citation Auditor | The report cites repo-local strategy and evidence posture docs; it introduces no external numeric claims. | pass_with_risk |
| Numeracy Checker | No calculations, price-year values, unit costs, benefit totals, ROI ratios, or monetary values are introduced. Price-year and uncertainty rules are required before future use. | pass |
| Freight Economist | Benefit classes are separated from cost classes and include negative/marginal-result handling before any ROI promotion. | pass_with_risk |
| State DOT Planner | Delivery, right-of-way, maintenance, and agency feasibility are treated as cost/risk classes, not assumed solved. | pass_with_risk |
| Stakeholder Review | Rural, community, environmental, non-driving access, and industry tradeoffs are listed as review requirements before claim promotion. | pass_with_risk |

## Gate

Decision: pass_with_risk

Rationale: ROUTE now has a draft ROI/cost framework suitable for internal
communications planning. It is not a numeric ROI report. The residual risk is
intentional: every future ROI, cost, benefit, or funding claim remains blocked
until sourced assumptions, price-year rules, uncertainty treatment, and role
review are complete.
