---
name: ROUTE Industry And Stakeholder Source Fixture Campaign
slug: route-industry-stakeholder-source-fixture-campaign
type: report
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/reports/industry-stakeholder-evidence-lane-matrix.md
  - docs/how-to/stakeholder-fixture-closeout-runbook.md
  - docs/templates/source-packs/stakeholder-fixture-source-pack-template.md
  - docs/reviews/source-backed-stakeholder-fixture-001.md
  - docs/reviews/source-backed-stakeholder-fixture-002.md
  - docs/reviews/source-backed-stakeholder-fixture-003.md
  - docs/reviews/source-backed-stakeholder-fixture-004.md
  - docs/reviews/source-backed-stakeholder-fixture-005.md
  - docs/reviews/communications-role-review-pass-artifacts.md
  - docs/reviews/communications-pressure-test-run-003.md
  - docs/media/media-source-index.md
  - docs/reports/route-evidence-posture.md
---

# ROUTE Industry And Stakeholder Source Fixture Campaign

## Purpose

This campaign turns the industry/stakeholder evidence-lane matrix into an
execution queue.

The matrix shows which lanes are represented. This campaign says what source
fixtures would be required before a reviewer could say a lane has a bounded
source-backed example.

This is not a validation record. It does not create endorsement, official-plan
status, construction readiness, guaranteed service, numeric ROI, eligibility,
compliance, public readiness, external readiness, agency approval, or broad
industry validation.

## Current Campaign State

Decision: **fixture_campaign_open**

Five fixtures are populated: the Port NOLA terminal-access example, the FHWA
truck-parking freight-operations example, the USDA/USDOT rural/agricultural
access example, the FHWA/DOT state delivery-control example, and the FHWA/EPA
community/environmental impact example. All other lanes remain planned fixture
work. Until more fixtures close, ROUTE can say the lanes are represented and
source-gated, not externally validated.

## Fixture Ledger

| Fixture ID | Lane | Current State | Evidence Target | Required Artifact Effect | Minimum Role Lanes | Pass Threshold |
|---|---|---|---|---|---|---|
| STAKE-FIX-001 | Ports / terminal access | pass_with_risk for internal rehearsal | Public terminal road-access source. | Source-backed fixture and trace-ready terminal-access label. | Scope Keeper, Citation Auditor, Numeracy Checker, Optimization Methodologist, Freight Economist, Freight Industry, Schematic Cartographer, State DOT Planner. | Closed for one bounded internal example; no external or broad terminal claim. |
| STAKE-FIX-002 | Freight carriers / shippers / operations | pass_with_risk for internal rehearsal | Public FHWA truck parking source. | Truck parking/rest/HOS becomes a bounded source-backed operating-constraint example. | Freight Industry, Long-Haul Trucker, Regional Shipper, Freight Economist, Numeracy Checker, Scope Keeper, Citation Auditor, Traffic Engineer, State DOT Planner. | Closed for one bounded internal example; no operating SLA, ROI, carrier, shipper, or corridor-specific parking claim. |
| STAKE-FIX-003 | Rural / agriculture / production-zone access | pass_with_risk for internal rehearsal | Public USDA rural transportation source and USDOT rural surface transportation program source. | Rural/agricultural access becomes a bounded source-backed review-lane example. | Rural Advocate, Rural Farmer, Rural Resident, State DOT Planner, Scope Keeper, Citation Auditor, Numeracy Checker, Traffic Engineer, Freight Economist. | Closed for one bounded internal example; no route support, access guarantee, funding, eligibility, county, or facility claim. |
| STAKE-FIX-004 | State DOT / delivery / maintenance | pass_with_risk for internal rehearsal | Public FHWA asset-management source and DOT Navigator FHWA NEPA/project-development source. | Delivery, maintenance, asset, financial-plan, investment-strategy, and project-development controls become a bounded source-backed example. | State DOT Planner, Traffic Engineer, Scope Keeper, Citation Auditor, Numeracy Checker, Schematic Cartographer, Freight Economist, Environmental Community. | Closed for one bounded internal example; no state approval, project readiness, funding, environmental clearance, eligibility, or asset-condition claim. |
| STAKE-FIX-005 | Community / environmental / local impact | pass_with_risk for internal rehearsal | Public FHWA social-environment source and EPA transportation air-pollution source. | Community/social-environment, vehicle-pollution, and health-effect concerns become a bounded source-backed review-control example. | Foxx, Environmental Community, Local Official, Scope Keeper, Citation Auditor, Numeracy Checker, Traffic Engineer, State DOT Planner. | Closed for one bounded internal example; no named impact, mitigation, clearance, public-involvement, compliance, or community-support claim. |
| STAKE-FIX-006 | Transit-dependent / intercity / non-driving access | planned | Public intercity coach, park-and-ride, first/last-mile, passenger facility, accessibility, or non-driving access source. | Add one non-driving access requirement, hold, or exclusion row. | Transit-Dependent, Intercity Traveler, Rural Resident, Scope Keeper, Citation Auditor, Numeracy Checker. | Non-driving access impact is visible; no transit service, facility, or access benefit claim. |
| STAKE-FIX-007 | Climate / resilience / emergency management | planned | Public hazard, closure, evacuation, detour, port disruption, wildfire, flood, snow/ice, or recovery source. | Add one hazard or recovery requirement with uncertainty and time horizon. | Climate Engineer, State DOT Planner, Traffic Engineer, Scope Keeper, Citation Auditor, Numeracy Checker. | Hazard pathway is source-backed or held; no resilience benefit, hardening, or recovery-performance claim. |
| STAKE-FIX-008 | ROI / cost / finance | planned | Public cost basis, price year, benefit category, freight delay, safety, asset repair, operations, funding program, or uncertainty source. | Add one ROI/cost evidence-contract row with units and excluded benefits. | Numeracy Checker, Freight Economist, Funder lens, Scope Keeper, Citation Auditor. | Inputs and exclusions are defined; no numeric ROI, benefit-cost, eligibility, or funding recommendation claim. |
| STAKE-FIX-009 | Technical / DOT-style review | planned | Named dry-run venue packet, selected materials, presenter, recorder, role review, and prohibited-claim scan. | External rehearsal packet can be evaluated as held/pass/fail. | Scope Keeper, Citation Auditor, Numeracy Checker, State DOT Planner, Traffic Engineer, affected lane roles. | A named rehearsal record exists; no agency review or approval unless explicitly documented by the external venue. |

## Fixture Intake Rules

| Rule | Required Practice |
|---|---|
| Start with the claim boundary. | State whether the lane is represented, source-needed, source-backed, held, or not in scope. |
| Use the template. | Each populated fixture must follow `docs/templates/source-packs/stakeholder-fixture-source-pack-template.md` or explain the deviation. |
| Preserve negative outcomes. | A fixture may produce a hold, exclusion, dissent row, or stricter caption instead of a positive example. |
| Keep source custody complete. | Source row must name title, owner, date/year, access note, field names or units when relevant, and reviewer. |
| Require before/after posture. | Show what claim, label, row, caption, source pack, or evidence row changed. |
| Re-run affected roles. | Editorial lanes are mandatory; domain lanes depend on the fixture. |
| Block promotion on missing source details. | If source custody is incomplete, the fixture remains `held_source`. |

## Review Questions By Lane

| Lane | Rude Question The Fixture Must Survive |
|---|---|
| Freight operations | "Where did a real operating constraint change your artifact, and what did you refuse to claim?" |
| Rural/agriculture | "Where is the source that says this access problem matters to production, emergency access, or rural service?" |
| State DOT / delivery | "What did the DOT-style review still reject or hold?" |
| Community/environment | "Where is dissent preserved, not converted into a vague benefits paragraph?" |
| Non-driving access | "What source says road-service changes affect travelers without cars, and what remains unproven?" |
| Resilience | "What hazard or closure evidence changed the claim boundary?" |
| ROI/cost | "What units, price year, excluded benefits, and uncertainty stop this from becoming fake ROI?" |
| Technical review | "What exact venue, materials, and scan results support saying this survived a review?" |

## Closeout Record Shape

Every closed fixture should add a short closeout using this shape:

| Field | Entry |
|---|---|
| Fixture ID |  |
| Lane |  |
| Source rows |  |
| Requirement row |  |
| Before artifact / label |  |
| After artifact / label |  |
| Roles reviewed |  |
| Decision | pass_internal / pass_with_risk / held_source / held_artifact / held_role / fail_scope |
| Claims allowed | represented / source-backed example / internal only / held |
| Claims still blocked |  |

## Campaign Exit Criteria

| Exit Level | Required Evidence | Allowed Language |
|---|---|---|
| Representation package | Matrix exists and roles/reports cover the lanes. | "These lanes are represented and source-gated." |
| Fixture package | At least one populated fixture exists in each priority lane, with role review. | "Each priority lane has at least one bounded source-backed example." |
| Rehearsal package | Named venue packet, selected materials, role review, prohibited-claim scan, and closeout exist. | "The package passed or held a named rehearsal under the recorded conditions." |
| External validation | Named external participant, source permission, venue, claim scope, and closeout authorize a specific statement. | Only the specific validated statement may be used. |

## Gate

Decision: **open_campaign**

Rationale: The campaign is now executable: the lanes, fixture IDs, evidence
targets, artifact effects, review roles, and pass thresholds are named.
STAKE-FIX-001 through STAKE-FIX-005 are populated for bounded internal
examples. All other fixture lanes remain planned until source custody,
before/after artifact posture, and role review close.
