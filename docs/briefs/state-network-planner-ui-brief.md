---
name: State Network Planner UI Brief
slug: state-network-planner-ui-brief
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - docs/briefs/route-first-client-wedge-package.md
  - docs/reports/route-business-model-report.md
  - docs/reports/route-competitive-landscape-report.md
  - docs/briefs/iowa-state-service-network-offer.md
  - docs/briefs/iowa-service-network-discovery-workshop.md
  - docs/briefs/iowa-service-network-sample-readout.md
  - docs/briefs/texas-state-service-network-offer.md
  - docs/reports/release-publication-scope-appendix.md
  - docs/reports/simulation-game-evidence-boundary.md
  - docs/reports/route-evidence-posture.md
---

# State Network Planner UI Brief

## Product Idea

ROUTE should have a client-facing state network planner UI.

The app opens with a preloaded candidate state network: cities, gateways,
freight nodes, rural regions, ports, terminals, hospitals, campuses, corridors,
candidate T1/T2/T3/T4/R roles, resilience stressors, and a suggested 90-day
service package.

The client can then edit the network instead of reacting to a static report.
They can add places, move a link between tiers, raise or lower planning-promise
targets, prioritize resilience failures, change investment-package order, and
export a readout that preserves evidence labels and claim holds.

## Why This Matters

The UI turns ROUTE from a report generator into a working session product.

The buyer should feel: "This is our network. We can test our priorities before
anyone hardens the map, claims ROI, or asks for construction approval."

## Primary Users

| User | Needs |
|---|---|
| State DOT planner | See how top cities, regions, gateways, and corridors fit into a service hierarchy. |
| Turnpike / toll operator | Adjust corridor promises, recovery priorities, and customer-service packages. |
| Port / terminal authority | Add terminal access nodes, truck staging issues, and land-side failure modes. |
| Freight coalition | Compare corridor packages across jurisdictions without forcing a single official plan. |
| Executive sponsor | Review tradeoffs, package sequence, and evidence holds in one screen. |

## First Screen

The first screen should be the usable planning surface, not a marketing page.

| Region | Content |
|---|---|
| Header | Client/state name, scenario name, evidence posture, export button. |
| Main map / schematic | Preloaded candidate network with tier colors, node classes, held labels, and selected route/package. |
| Left panel | Places, corridors, gateways, terminals, and unresolved additions. |
| Right panel | Dials for service priority, resilience, rural access, freight priority, asset debt, and investment appetite. |
| Bottom tray | Package sequence, evidence holds, unresolved tradeoffs, and next proof tasks. |

## Editable Controls

| Control | Client Action | ROUTE Output |
|---|---|---|
| Add place | Add city, terminal, border crossing, hospital, campus, production zone, or rural region. | New node with source-needed label until evidence is attached. |
| Change tier | Promote or demote a corridor between T1/T2/T3/T4/R candidate roles. | Tier-change note, affected promise rows, and role-review flag. |
| Promise dial | Adjust planning target emphasis for reliability, access, recovery, or terminal service. | Candidate promise backlog with legal-SLA hold. |
| Resilience dial | Raise priority for flood, snow, wildfire, closure, evacuation, bridge, incident, or port disruption. | Failure-mode ledger and alternate-route proof tasks. |
| Investment dial | Emphasize operations, asset repair, interchange work, terminal access, rural access, or long-range capital. | Revised package sequence. |
| Evidence toggle | Show source-backed, heuristic, source-needed, held, and next-proof rows. | Claim-boundary view for reviewers. |
| Compare scenario | Compare current suggestion with client edits. | Change log, tradeoff table, and exportable readout. |

## Dials

The app should use dials or sliders for planning emphasis, not fake precision.

| Dial | Low End | High End | Claim Boundary |
|---|---|---|---|
| Reliability | Accept more delay variance. | Prioritize predictable travel and recovery. | Not a guaranteed operating SLA. |
| Freight priority | General mobility emphasis. | Freight gateway, port, terminal, and production-zone emphasis. | Not proof of freight demand or ROI. |
| Rural access | Spine-focused package. | Stronger feeder/access obligations. | Not proof every rural gap is solved. |
| Resilience | Normal operations focus. | More closure, evacuation, alternate-route, and recovery emphasis. | Not disaster-readiness proof. |
| Asset debt | Ignore for scenario view. | Surface pavement, bridge, and maintenance blockers. | Not asset repair validation. |
| Capital appetite | Quick operations package. | Larger long-range capital package. | Not funding eligibility or construction readiness. |

## Data Objects

The UI should save edits as a planning-session artifact.

| Object | Required Fields |
|---|---|
| Session | client_id, region_id, scenario_id, created_at, evidence_posture, export_status |
| Node | node_id, label, node_class, candidate_role, source_status, client_added |
| Link | link_id, from_node, to_node, candidate_tier, edit_status, source_status |
| Promise | promise_id, tier_or_node, target_type, target_text, legal_sla_status |
| Dial | dial_id, value, rationale, changed_by, changed_at |
| Package | package_id, sequence_rank, package_type, affected_nodes_links, claim_status |
| Evidence hold | hold_id, affected_object, blocked_claim, next_evidence_step |
| Export | export_id, included_sections, prohibited_claim_scan_status |

## Session Outputs

| Output | Use |
|---|---|
| Edited network view | Shows what changed from the suggested plan. |
| Promise backlog | Lists candidate promises and legal-SLA holds. |
| Resilience agenda | Lists closure, recovery, evacuation, and alternate-route concerns. |
| Investment package sequence | Groups actions into operations, asset, access, resilience, and capital packages. |
| Evidence hold table | Shows what cannot be claimed yet. |
| Client readout export | Turns the session into a board, authority, sponsor, or leadership briefing. |

## Minimum Viable UI

The first prototype should support one state or authority package.

| Capability | MVP Requirement |
|---|---|
| Load scenario | Load one prebuilt state network fixture, such as Iowa or Texas. |
| Show candidate network | Render nodes, links, tiers, and held labels. |
| Edit places and tiers | Add node, change tier, and mark rationale. |
| Adjust dials | Save dial values and show changed package emphasis. |
| Show evidence holds | Display source-backed, heuristic, source-needed, and held labels. |
| Export readout | Generate a session summary with non-claims. |

Do not make the first UI a full GIS replacement, traffic dashboard, or
engineering design tool. It should be a session workbench for the 90-day
diagnostic.

## Non-Claims In The UI

Every export should carry these boundaries:

- Client edits are requirements or planning preferences, not validation.
- A promoted tier is a candidate role, not official designation.
- A promise target is a planning target, not a legal SLA.
- A package sequence is a decision aid, not funding eligibility.
- A map view is structural, not construction proof.
- A scenario comparison is heuristic until source and role review close.
- No edit implies agency endorsement, public approval, ROI, or construction
  readiness.

## Role Review Questions

| Role | Review Question |
|---|---|
| Scope Keeper | Does the UI avoid official-plan, construction, approval, and endorsement claims? |
| State DOT Planner | Does it preserve agency authority, delivery constraints, and public-process holds? |
| Freight Economist | Do freight and package dials avoid numeric ROI or demand proof? |
| Numeracy Checker | Are slider values ordinal preferences rather than unsupported calculations? |
| Schematic Cartographer | Does the map preserve structural/held labels and avoid proof-by-picture? |
| V&V | Are saved edits, exports, and claim labels reproducible from session data? |

## Boundary

This brief defines a product concept and implementation target. It does not
claim that the UI exists, has been validated, is public-ready, has client users,
has agency acceptance, produces legal SLAs, proves ROI, proves demand, proves
construction readiness, replaces GIS/modeling systems, or closes external
review.
