---
name: Source-Backed Stakeholder Fixture 003
slug: source-backed-stakeholder-fixture-003
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/reports/industry-stakeholder-source-fixture-campaign.md
  - docs/reports/industry-stakeholder-evidence-lane-matrix.md
  - docs/how-to/stakeholder-fixture-closeout-runbook.md
  - docs/templates/source-packs/stakeholder-fixture-source-pack-template.md
  - docs/reports/rural-access-national-service-network-report.md
  - docs/reports/t3-t4-access-evidence-appendix.md
  - .roles/parliament/rural-advocate.md
  - .roles/stakeholders/rural-farmer.md
  - .roles/stakeholders/rural-resident.md
  - .roles/stakeholders/state-dot.md
  - https://www.ams.usda.gov/services/transportation-analysis/rti
  - https://www.transportation.gov/grants/rural-surface-transportation-grant-program
---

# Source-Backed Stakeholder Fixture 003

## Purpose

This fixture closes one bounded rural/agriculture source-backed example for the
industry/stakeholder fixture campaign.

It uses USDA Agricultural Marketing Service rural transportation material and
the USDOT Rural Surface Transportation Grant Program page to show that rural
transportation, agricultural transportation, freight movement, and access to
agricultural, commercial, energy, or intermodal facilities are real review
lanes for ROUTE.

This fixture does not create rural community, farmer, USDA, USDOT, state DOT,
county, facility, or industry endorsement. It does not prove rural zone
coverage, route support, emergency access, funding eligibility, construction
readiness, guaranteed service, numeric ROI, compliance, approval, public
readiness, or external readiness.

## Fixture Metadata

| Field | Entry |
|---|---|
| Fixture ID | STAKE-FIX-003 |
| Stakeholder lane | rural access / agricultural transportation / production-zone access |
| Source pack owner | Citation Auditor |
| Meeting / intake artifact | public USDA AMS rural transportation study page and public USDOT Rural Surface Transportation Grant Program page |
| Source-backed requirement | Rural and agricultural access claims should name transportation effects on rural communities, agricultural transportation, freight movement, and access to agricultural or intermodal facilities before stronger T3/T4 access claims. |
| Affected geography / zone | National rural and agricultural transportation context; no county, state, corridor, facility, or zone selected. |
| Claim posture before fixture | represented by rural roles and rural access report; source-needed for concrete rural/agricultural access examples. |
| Intended ROUTE artifact to change | fixture campaign row / claim trace row / evidence posture / media source index. |
| Review lanes required | Scope Keeper, Citation Auditor, Numeracy Checker, Rural Advocate, Rural Farmer, Rural Resident, State DOT Planner, Traffic Engineer, Freight Economist. |

## Source Custody Rows

| Source ID | Source Path / URL | Title | Publisher / Owner | Date / Year | Access Note | Source Type | Units / Field Names | Reviewer |
|---|---|---|---|---|---|---|---|---|
| STAKE-SRC-003A | `https://www.ams.usda.gov/services/transportation-analysis/rti` | Study of Rural Transportation Issues | USDA Agricultural Marketing Service | accessed 2026-06-17; report responds to Food, Conservation, and Energy Act of 2008 Section 6206 | public URL | federal rural/agricultural transportation study page | no numeric value used; concepts used: rural transportation issues, transportation effects on rural communities, agricultural transportation, trucking/rail/barge/ocean modes | Citation Auditor |
| STAKE-SRC-003B | `https://www.transportation.gov/grants/rural-surface-transportation-grant-program` | The Rural Surface Transportation Grant Program | U.S. Department of Transportation | page last updated 2025-05-30; accessed 2026-06-17 | public URL | federal program information page | no funding amount used; concepts used: rural connectivity, safety/reliability of people and freight movement, access to agricultural/commercial/energy/intermodal facilities | Citation Auditor |

## Requirement-To-Refinement Rows

| Row ID | Requirement | Source ID | Before Artifact / Label | Change Applied | After Artifact / Label | Role Hold / Dissent | Claim Allowed? | Next Evidence Step |
|---|---|---|---|---|---|---|---|---|
| STAKE-FIX-003 | Treat rural/agricultural access as a source-backed review lane before using T3/T4 rural access, production-zone, or agricultural supply-chain language in external-facing materials. | STAKE-SRC-003A / STAKE-SRC-003B | `docs/reports/industry-stakeholder-source-fixture-campaign.md`: STAKE-FIX-003 planned; rural access report is story-ready/heuristic with zone-specific proof held. | Populated this fixture with USDA and USDOT source custody and updated campaign/source-index/trace posture to show one bounded rural/agricultural access example. | STAKE-FIX-003 becomes pass_with_risk for internal rehearsal; rural/agricultural access can be cited as a source-backed review lane. | County/zone coverage, facility access, emergency access, agricultural route selection, grant eligibility, project readiness, and funding claims remain held. | internal only / source-backed example | Add county or facility source, named production-zone obligation, access path, contact proof, state/local delivery review, and dissent rows before stronger rural access claims. |

## Evidence Boundary

| Safe Finding | Held Finding |
|---|---|
| USDA AMS identifies rural transportation issues with an emphasis on agricultural transportation. | ROUTE has measured farm-to-market, harvest-window, processor, elevator, or export-terminal access for a named zone. |
| USDOT describes rural surface transportation goals in terms of connectivity, safety/reliability of people and freight movement, and rural economic access. | ROUTE has selected a rural project, route, spur, bridge, or interchange for construction or funding. |
| USDOT lists access to agricultural, commercial, energy, or intermodal facilities as a type of eligible project context for the program. | Any ROUTE candidate is eligible for funding, should receive a grant, or satisfies program requirements. |
| ROUTE can use these sources to justify keeping rural/agricultural access in T3/T4 intake and evidence ledgers. | USDA, USDOT, state DOTs, counties, farmers, rural residents, or facilities endorse Interstate 2.0 or ROUTE. |

## Required Role Review

| Role Lane | Review Question | Result | Finding / Hold |
|---|---|---|---|
| Scope Keeper | Does the fixture remain an evidence artifact rather than a rural project or funding recommendation? | pass | The fixture changes claim posture only for a national rural/agricultural access review lane. |
| Citation Auditor | Are sources traceable by title, owner, date/access note, and URL? | pass | USDA AMS and USDOT page titles, owners, access notes, URLs, and used concepts are recorded. |
| Numeracy Checker | Are numeric claims, grant amounts, access distances, and calculations avoided or explicit? | pass | No funding amount, distance, volume, travel time, ROI, or benefit total is promoted. |
| Rural Advocate | Does the fixture keep rural access visible without overclaiming route proof? | pass_with_risk | Rural and agricultural access are source-backed as review lanes; named zone proof remains held. |
| Rural Farmer | Does the fixture preserve farm/production-zone logistics as a source-needed lane? | pass_with_risk | Agricultural transportation is represented; harvest-window, weight, processor/elevator, and export access claims need local sources. |
| Rural Resident | Does the fixture avoid turning rural connectivity into healthcare, evacuation, or employment proof? | pass_with_risk | Rural connectivity remains visible; emergency, healthcare, evacuation, and labor-market claims require separate sources. |
| State DOT Planner | Are delivery authority, eligibility, funding, maintenance, and environmental process bounded? | hold | Program context is not a ROUTE eligibility or funding finding. State/local delivery review remains required. |
| Traffic Engineer | Are safety and reliability bounded as program goals rather than measured outcomes? | pass_with_risk | The fixture supports review-lane language only; safety/reliability performance is not measured. |
| Freight Economist | Does the fixture avoid monetizing agricultural access value? | pass_with_risk | Agricultural access is source-backed as a concern; cost, benefit, and ROI remain gated. |

## Closeout Checklist

| Item | Pass / Hold | Evidence |
|---|---|---|
| Source custody row filled. | pass | `STAKE-SRC-003A` and `STAKE-SRC-003B` name URLs, titles, owners, access notes, source types, used concepts, and reviewer. |
| Requirement row filled. | pass | `STAKE-FIX-003` states the rural/agricultural access review-lane requirement. |
| Before/after artifact or label captured. | pass | Campaign and trace move STAKE-FIX-003 from planned/source-needed to pass_with_risk for an internal source-backed example. |
| Editorial roles reviewed. | pass_with_risk | Scope, citation, and numeracy findings recorded above. |
| Affected stakeholder lanes reviewed. | pass_with_risk | Rural Advocate, Rural Farmer, Rural Resident, State DOT Planner, Traffic Engineer, and Freight Economist lanes recorded. |
| Dissent or hold preserved. | pass | Zone coverage, project selection, funding, eligibility, emergency access, ROI, and delivery-authority claims remain held. |
| Prohibited-claim scan passes. | pass | Hits are guardrail, held, or non-approved contexts. |
| `docs/traces/route-claim-promotion-trace.md` updated if claim posture changes. | pass | `TRACE-CLAIM-011` added for rural/agricultural access fixture. |
| `docs/vtrace/VERIFICATION.md` updated if Round 5 gate status changes. | pass | STAKE-FIX-003 row added to Round 5 gate. |

## Gate

Decision: **pass_with_risk for internal rehearsal**

Rationale: This fixture provides a bounded source-backed rural/agricultural
access example. ROUTE can cite rural/agricultural access as a real review lane
for T3/T4 intake and evidence posture. It does not authorize county-specific
coverage, route promotion, emergency access, construction, funding, eligibility,
compliance, endorsement, approval, public-readiness, external-readiness, or
numeric ROI claims.
