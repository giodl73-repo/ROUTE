---
name: External Rehearsal Packet Selection Runbook
slug: external-rehearsal-packet-selection-runbook
type: how-to
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/reviews/communications-external-rehearsal-readiness.md
  - docs/templates/external-rehearsal-packet-template.md
  - docs/reviews/sponsor-dot-dry-run-packet-001.md
  - docs/reports/industry-stakeholder-fixture-closeout-report.md
  - docs/reports/industry-stakeholder-evidence-lane-matrix.md
  - docs/reports/route-evidence-posture.md
  - docs/decks/split-deck-presenter-guide.md
---

# External Rehearsal Packet Selection Runbook

## Purpose

This runbook helps select the first real external rehearsal packet after the
internal pressure-test package passes with risk.

It does not select a venue, schedule a meeting, claim external validation, or
authorize public use. It exists to prevent a blank packet template from becoming
an improvised briefing.

This runbook does not create endorsement, official-plan status, construction
readiness, guaranteed service, numeric ROI, eligibility, compliance, public
readiness, agency approval, or external readiness.

## Entry Rule

Do not fill an external packet until all four inputs exist:

| Input | Required Evidence | If Missing |
|---|---|---|
| Named venue | Specific body, reviewer group, sponsor, agency office, industry group, community group, or staff audience. | Stay internal. |
| Audience lane | One primary lane: sponsor, state, regional, freight, rural, community, congressional, FHWA/USDOT, or funder. | Use the internal sponsor-to-DOT dry-run packet only. |
| Concrete source-backed example | A selected STAKE-FIX row or new venue-specific source pack with source custody and role review. | Hold the packet. |
| Closing ask | Intake, evidence, source pack, demo fixture, standards review, or bounded pilot. | Hold the packet. |

The nine populated internal fixtures can help choose a lane, but they do not
prove external acceptance. A real venue still needs selected materials, affected
role review, prohibited-claim scan, and validation closeout.

## Venue Selection Matrix

| Candidate Venue Type | Use When The Main Question Is | Minimum Fixture Anchor | Required Added Roles | Safe Closing Ask | Default Decision |
|---|---|---|---|---|---|
| Sponsor / strategic reviewer | Is the evidence-bounded argument worth a next review step? | STAKE-FIX-009 plus one audience-relevant fixture. | Scope Keeper, Citation Auditor, Numeracy Checker. | Source pack or demo fixture. | hold until named sponsor and material set exist |
| State DOT / state policy reviewer | What would make this actionable or not for a state? | STAKE-FIX-004 and STAKE-FIX-009. | State DOT Planner, Traffic Engineer, Schematic Cartographer. | Intake or state source pack. | hold until state-specific authority and delivery holds are recorded |
| Regional peer group | What crosses state boundaries without implying commitment? | STAKE-FIX-003, STAKE-FIX-004, STAKE-FIX-007, or STAKE-FIX-009. | State DOT Planner plus affected rural, resilience, freight, or map roles. | Evidence handoff or standards review. | hold until regional scope and dissent rows exist |
| Freight / shipper / carrier audience | What operating pain points should refine the plan? | STAKE-FIX-001, STAKE-FIX-002, and STAKE-FIX-008. | Freight Industry, Long-Haul Trucker, Regional Shipper, Freight Economist. | Operating source pack. | hold until no SLA, utilization, or ROI claim is promoted |
| Rural / agricultural audience | Are production-zone and non-metro access burdens visible? | STAKE-FIX-003 and STAKE-FIX-006. | Rural Advocate, Rural Farmer, Rural Resident, Transit-Dependent or Intercity Traveler when relevant. | Local source pack or intake. | hold until local geography and burden evidence are sourced |
| Community / environmental audience | Are local impacts and health concerns treated as constraints? | STAKE-FIX-005 and STAKE-FIX-007. | Foxx, Environmental Community, Local Official, Climate Engineer when relevant. | Impact source pack or dissent intake. | hold until named impact and mitigation claims remain held |
| Congressional staff | What is the policy question without making a funding or construction ask? | STAKE-FIX-008 and STAKE-FIX-009, plus the lane being discussed. | Scope Keeper, Numeracy Checker, Freight Economist, State DOT Planner. | Evidence contract or standards review. | hold until no funding recommendation or eligibility claim appears |
| FHWA / USDOT technical reviewer | What can be inspected, reproduced, or challenged? | STAKE-FIX-009 plus command/evidence surfaces. | Citation Auditor, Numeracy Checker, Optimization Methodologist, State DOT Planner, Schematic Cartographer. | Technical evidence review or demo fixture. | hold until selected technical claims have validation closeout |
| Funder | What can be supported before ROI or construction claims? | STAKE-FIX-008 and STAKE-FIX-009. | Numeracy Checker, Freight Economist, Scope Keeper, State DOT Planner. | ROI/cost source pack. | hold until no numeric ROI or funding claim is promoted |

## Packet Assembly Steps

1. Copy `docs/templates/external-rehearsal-packet-template.md` into a named
   review file only after a real venue exists.
2. Fill rehearsal metadata with the venue, audience lane, presenter, recorder,
   source custody owner, and closing ask.
3. Select only the materials needed for that audience. Do not include every
   deck, report, fixture, and appendix by default.
4. Choose one primary fixture anchor and any secondary fixture rows needed to
   answer the audience's main question.
5. Add venue-specific source custody when the fixture is being used as more
   than an internal example.
6. Re-run required role review for the selected venue, not just for the generic
   communications package.
7. Run a prohibited-claim scan over the selected packet and materials.
8. Run `npm run check:l0` after packet edits.
9. Record the decision as `hold_external_rehearsal`, `pass_with_risk`, or
   `fail_scope`.

## Material Selection Rules

| Audience Need | Include | Usually Exclude |
|---|---|---|
| Strategic framing | Public solution deck, evidence posture, media-safe fact sheet, selected fixture closeout rows. | Technical crate appendices unless asked. |
| Technical inspection | ROUTE technology deck, claim trace, demo capture, evidence posture, source operations roadmap, selected fixture rows. | Political/funder language unless relevant. |
| State delivery review | State value brief, asset condition appendix, standards gates appendix, external readiness checklist, selected state/delivery fixtures. | National ROI framing unless a source pack exists. |
| Industry operations review | Industry brief, freight promise report, relay hubs report, fixture rows for terminal/freight/ROI. | Construction or funding narratives. |
| Community or rural review | Rural access report, resilience report, maps-not-proof report, selected rural/non-driving/community fixtures. | SLA or map-readiness claims. |
| Media inquiry | Media README, fact sheet, claim guide, Q&A, source index, visual-assets guide. | Internal rehearsal scorecards unless context requires them. |

## Pass / Hold Questions

Before a packet can move past `hold_external_rehearsal`, answer:

| Question | Pass Condition |
|---|---|
| Is the venue named? | A real body, reviewer group, sponsor, agency office, staff audience, or stakeholder group is recorded. |
| Is the audience lane primary? | One lane controls material selection and role review. |
| Is the fixture venue-appropriate? | The fixture anchor answers the audience question without implying endorsement or external validation. |
| Are affected roles reviewed? | Required role lanes have pass, pass_with_risk, hold, or fail records for this venue. |
| Are all stronger claims blocked? | Official-plan, construction, SLA, ROI, eligibility, compliance, endorsement, approval, and public-readiness claims are absent or explicitly held. |
| Did validation run? | Prohibited-claim scan and L0 are recorded after packet edits. |

## Failure Modes

| Failure Mode | Repair |
|---|---|
| Packet has no real venue. | Keep it as an internal dry run. |
| Packet includes every available artifact. | Remove materials that do not answer the primary audience question. |
| Internal fixtures are described as validation. | Replace with "bounded internal examples" and name the venue-specific evidence still needed. |
| Maps are used as proof. | Add map captions and route the claim through maps-not-proof posture. |
| ROI language becomes a number or business case. | Route through ROI/cost source pack and Numeracy Checker review. |
| State, DOT, congressional, industry, or community presence is implied as approval. | Add a non-approved-claims row and hold the packet. |

## Gate

Decision: **selection_runbook_ready; external_rehearsal_held**

Rationale: ROUTE now has enough internal fixture coverage to choose a first
external rehearsal lane deliberately. It still lacks a named external venue,
selected material packet, venue-specific source custody, affected role review,
and validation closeout. Until those exist, the package remains internal.
