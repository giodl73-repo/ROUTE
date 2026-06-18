---
name: Source-Backed Stakeholder Fixture 002
slug: source-backed-stakeholder-fixture-002
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
  - docs/briefs/industry-value-brief.md
  - docs/reports/forty-eight-hour-freight-promise-report.md
  - .roles/stakeholders/freight-industry.md
  - .roles/stakeholders/long-haul-trucker.md
  - .roles/stakeholders/regional-shipper.md
  - .roles/parliament/freight-economist.md
  - https://ops.fhwa.dot.gov/freight/infrastructure/truck_parking/index.htm
---

# Source-Backed Stakeholder Fixture 002

## Purpose

This fixture closes one bounded freight-operations source-backed example for
the industry/stakeholder fixture campaign.

It uses FHWA truck-parking material to show that truck parking, rest access,
hours-of-service breaks, and unsafe overflow parking are real freight operating
constraints that ROUTE should keep visible in industry-facing intake and
evidence posture.

This fixture does not create carrier, shipper, broker, truck-stop, FHWA, USDOT,
state DOT, or industry endorsement. It does not prove a corridor, hub, managed
lane, service window, relay model, construction package, ROI, eligibility,
compliance, approval, public readiness, or external readiness claim.

## Fixture Metadata

| Field | Entry |
|---|---|
| Fixture ID | STAKE-FIX-002 |
| Stakeholder lane | freight industry / long-haul trucker / regional shipper |
| Source pack owner | Citation Auditor |
| Meeting / intake artifact | public FHWA truck parking page |
| Source-backed requirement | Freight operating claims should include truck parking, rest access, HOS break needs, and unsafe overflow parking as source-backed constraints before stronger reliability or service claims. |
| Affected geography / zone | National Highway System / interstate freight operations; no route-specific geography selected. |
| Claim posture before fixture | represented by roles and industry brief; source-needed for concrete freight operating constraint examples. |
| Intended ROUTE artifact to change | fixture campaign row / claim trace row / evidence posture / media source index. |
| Review lanes required | Scope Keeper, Citation Auditor, Numeracy Checker, Freight Economist, Freight Industry, Long-Haul Trucker, Regional Shipper, Traffic Engineer, State DOT Planner. |

## Source Custody Rows

| Source ID | Source Path / URL | Title | Publisher / Owner | Date / Year | Access Note | Source Type | Units / Field Names | Reviewer |
|---|---|---|---|---|---|---|---|---|
| STAKE-SRC-002 | `https://ops.fhwa.dot.gov/freight/infrastructure/truck_parking/index.htm` | Truck Parking | Federal Highway Administration, Office of Operations Freight Management and Operations | accessed 2026-06-17; page includes 2025 and 2026 update references | public URL | federal freight operations resource | no numeric value used; concepts used: truck parking, HOS breaks, unsafe parking locations, safety concern, state/MPO parking needs | Citation Auditor |

## Requirement-To-Refinement Rows

| Row ID | Requirement | Source ID | Before Artifact / Label | Change Applied | After Artifact / Label | Role Hold / Dissent | Claim Allowed? | Next Evidence Step |
|---|---|---|---|---|---|---|---|---|
| STAKE-FIX-002 | Treat truck parking and rest access as source-backed freight operating constraints before using freight reliability, relay, or service-window language in industry-facing materials. | STAKE-SRC-002 | `docs/reports/industry-stakeholder-source-fixture-campaign.md`: STAKE-FIX-002 planned; industry brief names truck parking/rest gaps as data asks without a populated source-backed fixture. | Populated this fixture with FHWA source custody and updated campaign/source-index/trace posture to show one bounded freight-operations example. | STAKE-FIX-002 becomes pass_with_risk for internal rehearsal; truck parking/rest/HOS can be cited as a source-backed operating-constraint example. | Corridor-specific parking supply, parking adequacy, exact locations, operating delay, service-window impact, ROI, funding, and delivery authority remain held. | internal only / source-backed example | Add state or corridor-level truck parking inventory, facility locations, utilization, time-of-day demand, delivery-window context, and role review before stronger lane or corridor claims. |

## Evidence Boundary

| Safe Finding | Held Finding |
|---|---|
| FHWA identifies truck parking as a national safety and freight-operations concern. | ROUTE has measured parking adequacy for any corridor, state, or hub. |
| Truck parking is necessary for drivers to take hours-of-service breaks. | ROUTE proves a service window, relay schedule, or driver-rest plan. |
| Lack of official available parking can push trucks toward unsafe locations. | Any named corridor, interchange, rest area, or truck stop should be built, expanded, funded, or approved. |
| ROUTE can use this source to justify keeping truck parking/rest access in freight intake and evidence ledgers. | FHWA, carriers, shippers, truck-stop operators, or drivers endorse Interstate 2.0 or ROUTE. |

## Required Role Review

| Role Lane | Review Question | Result | Finding / Hold |
|---|---|---|---|
| Scope Keeper | Does the fixture remain an evidence artifact rather than a freight program or project recommendation? | pass | The fixture changes claim posture only for a national operating-constraint example. |
| Citation Auditor | Is the source traceable by title, owner, date/access note, and URL? | pass | FHWA page title, owner, access date, URL, and used concepts are recorded. |
| Numeracy Checker | Are numeric claims, units, and calculations avoided or explicit? | pass | No percentage, supply count, cost, crash, ROI, or delay value is promoted. |
| Freight Economist | Does the fixture avoid turning a parking constraint into monetized freight value? | pass_with_risk | Parking/rest is source-backed as a constraint; economic value and ROI remain gated. |
| Freight Industry | Does the fixture represent operational constraints without implying carrier or industry acceptance? | pass_with_risk | HOS/rest/parking concerns are represented; no industry validation or lane support is claimed. |
| Long-Haul Trucker | Does the fixture preserve driver rest and safety as an operating concern? | pass_with_risk | Driver-rest concern is visible; exact facility adequacy and safe-route guidance remain held. |
| Regional Shipper | Does the fixture avoid overclaiming delivery-window performance? | pass_with_risk | The fixture supports operating-constraint intake only; service-window and customer reliability claims remain held. |
| Traffic Engineer | Are roadway shoulder, ramp, and safety implications bounded? | pass_with_risk | Unsafe parking locations are recognized as a concern, not a corridor-specific safety finding. |
| State DOT Planner | Are state/MPO needs, funding, maintenance, and delivery authority bounded? | hold | State and corridor parking needs require state/corridor source packs and delivery review. |

## Closeout Checklist

| Item | Pass / Hold | Evidence |
|---|---|---|
| Source custody row filled. | pass | `STAKE-SRC-002` names URL, title, owner, access note, source type, used concepts, and reviewer. |
| Requirement row filled. | pass | `STAKE-FIX-002` states the truck parking/rest/HOS operating-constraint requirement. |
| Before/after artifact or label captured. | pass | Campaign and trace move STAKE-FIX-002 from planned/source-needed to pass_with_risk for an internal source-backed example. |
| Editorial roles reviewed. | pass_with_risk | Scope, citation, numeracy, and freight-economics findings recorded above. |
| Affected stakeholder lanes reviewed. | pass_with_risk | Freight Industry, Long-Haul Trucker, Regional Shipper, Traffic Engineer, and State DOT Planner lanes recorded. |
| Dissent or hold preserved. | pass | State/corridor parking adequacy, funding, service-window, ROI, and delivery-authority claims remain held. |
| Prohibited-claim scan passes. | pass | Hits are guardrail, held, or non-approved contexts. |
| `docs/traces/route-claim-promotion-trace.md` updated if claim posture changes. | pass | `TRACE-CLAIM-010` added for freight-operations fixture. |
| `docs/vtrace/VERIFICATION.md` updated if Round 5 gate status changes. | pass | STAKE-FIX-002 row added to Round 5 gate. |

## Gate

Decision: **pass_with_risk for internal rehearsal**

Rationale: This fixture provides a bounded source-backed freight-operations
example: truck parking, rest access, and HOS break needs can be cited as real
operating constraints in internal rehearsal and media source guidance. It does
not authorize corridor-specific parking adequacy, service-window, relay,
managed-lane, construction, funding, ROI, eligibility, compliance, endorsement,
approval, public-readiness, or external-readiness claims.
