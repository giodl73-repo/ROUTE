---
name: International Adapter Source Pack Template
slug: international-adapter-source-pack-template
type: template
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - docs/reviews/international-hierarchy-replication-closeout-001.md
  - data/international-hierarchy-replication-summary-001.csv
  - data/international-cross-region-scoring-rubric-001.csv
  - docs/how-to/international-hierarchy-iteration-playbook.md
---

# International Adapter Source Pack Template

## Purpose

Use this template when a Canada, EU-region, India, Japan, China, border, or
other non-U.S. pilot is ready to move from heuristic hierarchy rows toward
source-bound adapter work.

This template does not create an official network, country or regional
approval, policy alignment, route designation, construction readiness,
guaranteed SLA, disaster-readiness, numeric ROI, eligibility, compliance,
endorsement, external validation, public-readiness, or external-readiness
claim.

## Adapter Metadata

| Field | Entry |
|---|---|
| Adapter pack ID |  |
| Jurisdiction / region |  |
| Scope boundary | country / province-state group / corridor region / border gateway / port region / other |
| Candidate hierarchy review |  |
| Current map / artifact |  |
| Source pack owner |  |
| Local reviewer lanes required | transport planner / freight reviewer / port reviewer / rural-access reviewer / resilience reviewer / environmental-community reviewer / numeracy reviewer / cartographer / technical reviewer |
| Claim posture before pack | heuristic-held / source-needed / gated / held |
| Intended promotion | source-candidate / parse-ready / source-needed / held |

## Required Source Families

| Source Family | Source ID | Source Path / URL | Owner / Publisher | Date / Year | Access Note | Required Fields | Adapter Target | Claim Boundary |
|---|---|---|---|---|---|---|---|---|
| Road graph / classification | INTL-SRC-ROAD-001 |  |  |  | public / restricted / source-needed | route id, geometry, classification, directionality, access limits | road_graph | classification source does not prove service role |
| Node catalog | INTL-SRC-NODE-001 |  |  |  | public / restricted / source-needed | ports, terminals, border crossings, logistics hubs, metros, rural/production nodes | node_catalog | node presence does not prove priority or endorsement |
| Freight / passenger / production need | INTL-SRC-NEED-001 |  |  |  | public / restricted / source-needed | flow, volume class, OD pair, commodity, passenger/rural/access signal | need_surface | need signal does not prove project priority or ROI |
| Port / terminal / border operation | INTL-SRC-TERM-001 |  |  |  | public / restricted / source-needed | terminal, crossing, access road, dwell/queue proxy, connector role | terminal_access | terminal source does not prove access adequacy or throughput |
| Hazard / resilience / constraint | INTL-SRC-HAZ-001 |  |  |  | public / restricted / source-needed | hazard type, time horizon, closure/disruption proxy, alternate route, uncertainty | constraint_ledger | hazard source does not prove disaster-readiness |
| Governance / policy / authority | INTL-SRC-GOV-001 |  |  |  | public / restricted / source-needed | agency role, plan status, authority, restriction, approval boundary | governance_ledger | governance source does not imply approval or policy alignment |
| Service target / SLA assumption | INTL-SRC-SLA-001 |  |  |  | public / restricted / source-needed | target, unit, time period, reliability basis, sensitivity range | service_targets | target remains assumption until adopted and validated elsewhere |

## Field Mapping

| Adapter Field | Source ID | Source Field | Transform / Normalization | Units | Evidence Label | Gap If Missing |
|---|---|---|---|---|---|---|
| jurisdiction_scope |  |  |  |  | source-needed | adapter cannot leave fixture scope |
| road_graph_id |  |  |  |  | source-needed | no source-bound route rows |
| node_id |  |  |  |  | source-needed | node remains heuristic centroid |
| candidate_tier |  |  |  |  | heuristic-held | role cannot promote |
| need_class |  |  |  |  | source-needed | need surface remains assumed |
| constraint_class |  |  |  |  | source-needed | resilience and construction claims remain held |
| service_target |  |  |  |  | held | no SLA promotion |

## Promotion Gate

| Gate Item | Required Before Promotion | Result |
|---|---|---|
| Source custody complete | every source row has owner, title/path, date/year, access note, and reviewer | pending |
| Field mapping complete | adapter fields map to source fields with transforms and units | pending |
| Gap backlog generated | missing fields become source-needed or held rows | pending |
| Role review complete | local transport, freight, resilience, numeracy, map, and technical lanes review the pack | pending |
| Prohibited-claim scan complete | official, approval, SLA, construction, ROI, compliance, endorsement, validation, public-readiness, and external-readiness claims are not promoted | pending |

## Gate

Decision: held_template

Rationale: This template makes non-U.S. source-bound adapter work executable
without treating fixture maps, scorecards, or local source rows as official
network proof or external validation.
