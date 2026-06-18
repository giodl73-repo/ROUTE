---
name: Source-Backed Stakeholder Fixture 008
slug: source-backed-stakeholder-fixture-008
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/reports/industry-stakeholder-source-fixture-campaign.md
  - docs/reports/industry-stakeholder-evidence-lane-matrix.md
  - docs/how-to/stakeholder-fixture-closeout-runbook.md
  - docs/templates/source-packs/stakeholder-fixture-source-pack-template.md
  - docs/reports/route-roi-cost-framework.md
  - docs/reports/roi-without-fake-numbers-report.md
  - .roles/editorial/numeracy-checker.md
  - .roles/editorial/citation-auditor.md
  - .roles/editorial/scope-keeper.md
  - .roles/parliament/freight-economist.md
  - .roles/stakeholders/state-dot.md
  - https://www.transportation.gov/mission/office-secretary/office-policy/transportation-policy/benefit-cost-analysis-guidance
  - https://www.transportation.gov/mission/office-secretary/office-policy/transportation-policy/benefit-cost-analysis-spreadsheet-template
---

# Source-Backed Stakeholder Fixture 008

## Purpose

This fixture closes one bounded ROI, cost, finance, and benefit-cost source-
backed example for the industry/stakeholder fixture campaign.

It uses USDOT benefit-cost analysis guidance and the USDOT BCA spreadsheet
template page to show that benefit-cost work requires a methodological source,
structured fields, source custody, and explicit exclusions before any number is
promoted.

This fixture does not create USDOT, FHWA, state DOT, funder, applicant,
economist, or industry endorsement. It does not prove project eligibility,
funding readiness, cost estimate quality, benefit value, benefit-cost ratio,
positive ROI, grant competitiveness, business-case strength, construction
readiness, official-plan status, approval, public readiness, or external
readiness.

## Fixture Metadata

| Field | Entry |
|---|---|
| Fixture ID | STAKE-FIX-008 |
| Stakeholder lane | ROI / cost / finance / benefit-cost review |
| Source pack owner | Numeracy Checker |
| Meeting / intake artifact | public USDOT BCA guidance page and USDOT BCA spreadsheet-template page |
| Source-backed requirement | ROI, cost, finance, benefit, and benefit-cost claims should preserve source-backed methodology, price-year, analysis-period, benefit/cost category, inclusion/exclusion, uncertainty, and role-review controls before any numeric ROI or funding language. |
| Affected geography / zone | National USDOT BCA guidance context; no state, corridor, hub, package, applicant, grant program, or project selected. |
| Claim posture before fixture | represented by ROI/cost framework, ROI without fake numbers report, funder brief, Numeracy Checker, Freight Economist, and media claim guide; source-needed for a concrete ROI/cost evidence-contract example. |
| Intended ROUTE artifact to change | fixture campaign row / claim trace row / evidence posture / media source index. |
| Review lanes required | Scope Keeper, Citation Auditor, Numeracy Checker, Freight Economist, State DOT Planner. |

## Source Custody Rows

| Source ID | Source Path / URL | Title | Publisher / Owner | Date / Year | Access Note | Source Type | Units / Field Names | Reviewer |
|---|---|---|---|---|---|---|---|---|
| STAKE-SRC-008A | `https://www.transportation.gov/mission/office-secretary/office-policy/transportation-policy/benefit-cost-analysis-guidance` | Benefit-Cost Analysis Guidance for Discretionary Grant Programs | U.S. Department of Transportation, Office of the Chief Economist / Assistant Secretary for Transportation Policy | 2026 update; page last updated 2025-12-23; accessed 2026-06-18 | public URL with linked guidance document | federal BCA methodology guidance | no numeric value used; concepts used: BCA, expected benefits, expected costs, analysis period, methodology, source consistency, discretionary grant application context | Citation Auditor |
| STAKE-SRC-008B | `https://www.transportation.gov/mission/office-secretary/office-policy/transportation-policy/benefit-cost-analysis-spreadsheet-template` | Benefit-Cost Analysis Spreadsheet Template for Discretionary Grant Programs | U.S. Department of Transportation, Office of the Chief Economist / Assistant Secretary for Transportation Policy | page last updated 2025-12-23; accessed 2026-06-18 | public URL with linked spreadsheet template | federal BCA spreadsheet-template page | no numeric value used; concepts used: structured BCA template, spreadsheet artifact, BCA tags, sourceable calculation surface | Numeracy Checker |

## Requirement-To-Refinement Rows

| Row ID | Requirement | Source ID | Before Artifact / Label | Change Applied | After Artifact / Label | Role Hold / Dissent | Claim Allowed? | Next Evidence Step |
|---|---|---|---|---|---|---|---|---|
| STAKE-FIX-008 | Treat ROI, cost, finance, benefit, and benefit-cost language as an evidence contract requiring method, price year, scope, analysis period, included/excluded benefits, included/excluded costs, uncertainty, source custody, and role review before any number or funding posture is promoted. | STAKE-SRC-008A / STAKE-SRC-008B | `docs/reports/industry-stakeholder-source-fixture-campaign.md`: STAKE-FIX-008 planned; ROI framework existed but no populated source-backed ROI/cost fixture existed. | Populated this fixture with USDOT source custody and updated campaign/source-index/trace posture to show one bounded ROI/cost evidence-contract example. | STAKE-FIX-008 becomes pass_with_risk for internal rehearsal; ROI/cost/benefit-cost concerns can be cited as source-backed review controls. | Numeric ROI, benefit-cost ratio, dollar value, project cost, grant rating, funding eligibility, funding recommendation, business-case conclusion, applicant competitiveness, and construction funding remain held. | internal only / source-backed example | Add named option, price year, cost basis, benefit classes, excluded benefits, excluded costs, uncertainty treatment, calculation artifact, source files, and role review before stronger finance claims. |

## Evidence Boundary

| Safe Finding | Held Finding |
|---|---|
| USDOT BCA guidance supports treating benefit-cost work as a systematic method for identifying, quantifying, and comparing expected benefits and costs. | ROUTE has completed a BCA for any corridor, hub, project, grant application, or package. |
| USDOT's BCA spreadsheet-template page supports requiring a structured calculation artifact before numeric finance claims. | ROUTE has a completed spreadsheet, validated model, accepted calculation, or grant-ready finance package. |
| ROUTE can use these sources to justify price-year, scope, analysis-period, inclusion/exclusion, uncertainty, and source-custody controls in ROI materials. | ROUTE has proven a benefit-cost ratio, positive ROI, cost estimate, grant rating, eligibility, funding recommendation, or business case. |
| A finance fixture can close as a hold or exclusion row when sources, units, uncertainty, or excluded benefits are missing. | ROUTE has demonstrated economic value, cost effectiveness, monetized benefits, fiscal capacity, or public-readiness. |

## Required Role Review

| Role Lane | Review Question | Result | Finding / Hold |
|---|---|---|---|
| Scope Keeper | Does the fixture remain an evidence artifact rather than a funding, project, grant, or construction claim? | pass | The fixture changes claim posture only for ROI/cost/benefit-cost review controls. |
| Citation Auditor | Are sources traceable by title, owner, date/access note, and URL? | pass | USDOT guidance and template page titles, owners, update/access notes, URLs, and used concepts are recorded. |
| Numeracy Checker | Are dollar values, ratios, discount values, cost totals, benefit totals, and ratings avoided or explicit? | pass | No dollar value, ratio, rating, cost estimate, benefit total, ROI, BCA result, or funding value is promoted. |
| Freight Economist | Does the fixture preserve freight/economic value as a review lane without monetizing unsupported benefits? | pass_with_risk | Benefit classes remain source-backed review controls; freight reliability and operating value require named sources before monetization. |
| State DOT Planner | Are grant, applicant, delivery, match, finance, and project-readiness decisions bounded? | hold | USDOT methodology context is not a state finance plan, grant application, eligibility finding, or funding decision. |

## Closeout Checklist

| Item | Pass / Hold | Evidence |
|---|---|---|
| Source custody row filled. | pass | `STAKE-SRC-008A` and `STAKE-SRC-008B` name URLs, titles, owners, dates/access notes, source types, used concepts, and reviewer. |
| Requirement row filled. | pass | `STAKE-FIX-008` states the ROI/cost evidence-contract requirement. |
| Before/after artifact or label captured. | pass | Campaign and trace move STAKE-FIX-008 from planned/source-needed to pass_with_risk for an internal source-backed example. |
| Editorial roles reviewed. | pass_with_risk | Scope, citation, and numeracy findings recorded above. |
| Affected stakeholder lanes reviewed. | pass_with_risk | Freight Economist and State DOT Planner lanes recorded. |
| Dissent or hold preserved. | pass | Numeric ROI, benefit-cost ratio, dollar value, project cost, grant rating, eligibility, funding recommendation, business case, and construction funding remain held. |
| Prohibited-claim scan passes. | pass | Hits are guardrail, held, or non-approved contexts. |
| `docs/traces/route-claim-promotion-trace.md` updated if claim posture changes. | pass | `TRACE-CLAIM-016` added for ROI/cost fixture. |
| `docs/vtrace/VERIFICATION.md` updated if Round 5 gate status changes. | pass | STAKE-FIX-008 row added to Round 5 gate. |

## Gate

Decision: **pass_with_risk for internal rehearsal**

Rationale: This fixture provides a bounded source-backed ROI, cost, finance,
and benefit-cost example. ROUTE can cite methodology, spreadsheet-artifact,
price-year, analysis-period, inclusion/exclusion, uncertainty, and source-
custody questions as real review controls. It does not authorize numeric ROI,
benefit-cost ratio, dollar value, project cost, grant rating, funding
eligibility, funding recommendation, business-case strength, endorsement,
approval, public-readiness, external-readiness, or construction claims.
