---
name: ROUTE Communications Role Review
slug: route-communications-role-review
type: review
status: draft
rubric_version: v1.0
author: copilot
created: 2026-06-16
updated: 2026-06-16
sources:
  - .roles/ROLE.md
  - .roles/editorial/scope-keeper.md
  - .roles/editorial/citation-auditor.md
  - .roles/editorial/numeracy-checker.md
  - .roles/parliament/freight-economist.md
  - .roles/parliament/traffic-engineer.md
  - .roles/parliament/climate-engineer.md
  - .roles/parliament/rural-advocate.md
  - .roles/parliament/optimization-methodologist.md
  - .roles/parliament/schematic-cartographer.md
  - .roles/parliament/foxx.md
  - .roles/stakeholders/state-dot.md
  - .roles/stakeholders/freight-industry.md
  - .roles/stakeholders/regional-shipper.md
  - .roles/stakeholders/long-haul-trucker.md
  - .roles/stakeholders/rural-farmer.md
  - .roles/stakeholders/rural-resident.md
  - .roles/stakeholders/local-official.md
  - .roles/stakeholders/transit-dependent.md
  - .roles/stakeholders/environmental-community.md
  - docs/decks/interstate-2-0-pitch.md
  - docs/decks/route-technology-story.md
  - docs/decks/split-deck-presenter-guide.md
  - docs/reports/interstate-2-0-doctrine-report.md
  - docs/reports/relay-hubs-aviation-model-report.md
  - docs/reports/forty-eight-hour-freight-promise-report.md
  - docs/reports/roi-without-fake-numbers-report.md
  - docs/reports/rural-access-national-service-network-report.md
  - docs/reports/resilience-before-crisis-report.md
  - docs/reports/maps-are-not-proof-report.md
  - docs/reports/requirement-to-refinement-demonstration-report.md
  - docs/briefs/political-value-brief.md
  - docs/briefs/state-value-brief.md
  - docs/briefs/industry-value-brief.md
  - docs/briefs/funder-value-brief.md
  - docs/reports/route-evidence-posture.md
  - docs/reports/route-roi-cost-framework.md
  - docs/research-conclusions.md
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
  - docs/how-to/run-route-demo.md
---

# ROUTE Communications Role Review

## Scope

This review applies ROUTE's `.roles` lenses to the current communications
package:

- Interstate 2.0 solution deck;
- ROUTE technology deck;
- split deck presenter guide;
- Interstate 2.0 doctrine report;
- relay hubs aviation model report;
- 48-hour freight promise report;
- ROI without fake numbers report;
- rural access national service network report;
- resilience before crisis report;
- maps are not proof report;
- requirement-to-refinement demonstration report;
- political value brief;
- state value brief;
- industry value brief;
- funder value brief;
- research conclusions index;
- ROI/cost framework;
- evidence posture report;
- communications strategy;
- requirement-to-refinement demo how-to.

This review does not validate corridor, cost, ROI, SLA, engineering, safety,
environmental, or official agency claims. It evaluates whether the materials are
well-scoped, persuasive, honest about evidence posture, and ready for the next
communications iteration.

## Overall Decision

Decision: **pass_with_risk**

The roles broadly agree that the split into two decks is the right direction:

1. **Interstate 2.0 deck** should sell the solution vision without ROUTE
   platform branding.
2. **ROUTE technology deck** should explain how the platform makes that vision
   inspectable, refinable, and evidence-bounded.

The package is directionally strong and safer than the earlier combined deck.
The main residual risk is not overclaiming; it is **under-specifying the next
audience-specific intake and demo steps**. The roles want the next version to
show how states, industry, communities, and evidence actually enter the
refinement loop.

## Consensus Findings

| ID | Finding | Roles Agreeing | Disposition |
|---|---|---|---|
| CONS-001 | Splitting the public solution pitch from the ROUTE technology deck is correct. | Scope Keeper, State DOT, Freight Economist, Schematic Cartographer, Optimization Methodologist | accepted |
| CONS-002 | The current package avoids unsupported ROI, construction-readiness, official-plan, and publication-grade SLA claims. | Citation Auditor, Numeracy Checker, Scope Keeper, Freight Economist | accepted |
| CONS-003 | The service hierarchy story is the strongest core narrative. | Eisenhower-style national frame, Freight Economist, Rural Advocate, State DOT, Schematic Cartographer | accepted |
| CONS-004 | Relay hubs are the best human/operational bridge between current trucking, driver quality of life, EV charging, and future autonomy. | Freight Industry, Freight Economist, State DOT, Rural Advocate, Traffic Engineer | accepted_with_risk |
| CONS-005 | The evidence posture report and ROI/cost framework are necessary guardrails before external funder use. | Numeracy Checker, Citation Auditor, Freight Economist, Scope Keeper | accepted |
| CONS-006 | The five new reports preserve evidence boundaries and improve the research/report slate without promoting construction, official-plan, guaranteed-SLA, or numeric ROI claims. | Scope Keeper, Citation Auditor, Numeracy Checker, Optimization Methodologist | accepted |
| CONS-007 | The new reports make the right tensions visible: freight value vs. rural access, map persuasion vs. map proof, resilience obligation vs. hazard proof, and demo artifact loop vs. optimizer proof. | Freight Economist, Rural Advocate, Schematic Cartographer, Climate Resilience Engineer, State DOT | accepted_with_risk |

## Role Findings And Suggestions

| Role | Decision | What The Role Likes | Concern / Tension | Suggested Change |
|---|---|---|---|---|
| Scope Keeper | pass | Deck split solves the biggest scope problem: solution vs. platform. | Some package names still mix ROUTE and Interstate 2.0 in ways that could confuse public vs. technical audiences. | Keep public-facing filenames and labels solution-oriented; keep ROUTE named only in technology/platform materials. |
| Citation Auditor | pass_with_risk | No external numeric transportation claims or ROI values are introduced. | The service promise numbers are internal doctrine but still numbers in public-facing decks. | Add presenter note or appendix language: "service promises are planning targets, not verified guarantees." |
| Numeracy Checker | pass | No cost totals, ROI ratios, volumes, or benefit totals appear. | Future cost/ROI slides could drift quickly if summary numbers are added. | Require price year, horizon, included/excluded benefits, and uncertainty before any numeric slide. |
| Freight Economist | pass_with_risk | The materials sell reliability as the economic language of Interstate 2.0. | The package still lacks a concrete industry data ask. | Add an industry intake appendix: OD lanes, delay windows, commodity sensitivity, dwell time, backhaul, bridge/clearance/weight blockers, and PTI/reliability evidence. |
| Freight Industry | pass_with_risk | Relay hubs, managed lanes, charging, and staged AV path are compelling. | Operational constraints are under-specified. | Add freight requirements: HOS, truck parking, WIM/PrePass, bridge weights, clearances, dimensional permits, charging dwell, maintenance bases. |
| State DOT Planner | pass_with_risk | State brief correctly avoids "Washington hands down a map." | Delivery realism needs to appear earlier in funder-facing materials. | Add a state/funder slide or appendix with match funding, maintenance burden, right-of-way, environmental review, phasing, and lifecycle cost. |
| Traffic Engineer | pass_with_risk | T1 roadway needs, interchanges, managed lanes, and redundancy are recognizable operational categories. | Deck language can imply fixes before geometry, V/C, bridge, pavement, and safety evidence exists. | Use "candidate improvement families" in technical notes; keep engineering proof gated. |
| Schematic Cartographer | pass_with_risk | Map slide is visually strong and now scoped as schematic proof-of-concept. | Labels are unreadable at deck scale; detailed map claim could be misunderstood. | Add a map appendix or speaker note: "for topology and hierarchy only; detailed labels and claim labels live in generated artifacts." |
| Optimization Methodologist | pass_with_risk | ROUTE technology deck correctly frames refinement as public goal -> requirement -> model change -> plan refinement. | It needs one concrete demo of a requirement changing a plan. | Build the next demo script around a state/industry requirement entering the system and changing a staged option or evidence label. |
| Rural Advocate | pass_with_risk | T3/T4 access and production-zone language keeps rural access visible. | The solution pitch still leans freight-first; rural life-safety and agricultural access could be more emotionally legible. | Add one rural example card: agriculture, trauma/health access, evacuation, or production-zone access. |
| Anthony Foxx / Equity | pass_with_risk | Community concerns are now named as early inputs, not afterthoughts. | The pitch needs a stronger guardrail against repeating old highway harms. | Add "who is at the table" or "concerns before concrete" language to the solution or state brief. |
| Environmental Community | pass_with_risk | EV transition and resilience are framed as opportunities rather than proof. | Community health, air quality, runoff, noise, and habitat are not yet concrete enough. | Add environmental/community-health intake rows before any feature package is promoted. |
| Climate Resilience Engineer | pass_with_risk | Resilience is included as a core service obligation. | "Resilience" is generic unless tied to hazard classes and time horizon. | Add hazard examples: flood, heat, wildfire, storm surge, snow/ice, mountain pass closure, port disruption. |
| Transit-Dependent / Access Lens | pass_with_risk | T3/T4 and hub language can support access beyond through-freight. | Passenger/non-driving access is not prominent in the public pitch. | Keep passenger/transit claims gated, but add intake prompt for intercity coach, park-and-ride, and first/last-mile access. |

## Supplemental Review: Report Slate

This supplemental pass applies the same `.roles` lanes to the five reports added
after the first communications package review. The result is still
`pass_with_risk`: the reports are safe as internal draft communications
artifacts, but they should drive specific next evidence packages before any
external or claim-promoting use.

| Report | Primary Role Lanes | Decision | Role Finding | Required Next Evidence / Edit |
|---|---|---|---|---|
| `roi-without-fake-numbers-report.md` | Freight Economist, Numeracy Checker, Citation Auditor, State DOT, Foxx, Environmental Community | pass | Correctly refuses numeric ROI and requires price year, scope, uncertainty, exclusions, and negative cases. | Add a future ROI source-pack template with freight flow, delay/reliability, drayage, operating cost, lifecycle cost, mitigation, and distributional-impact fields before any numeric work. |
| `rural-access-national-service-network-report.md` | Rural Advocate, Rural Farmer, Rural Resident, Local Official, Transit-Dependent, State DOT | pass_with_risk | Strongly protects T3/T4 as first-class access tiers and blocks score-only promotion. | Add rural access intake fields for farm-to-elevator/processor distance, bridge/weight limits, trauma-center access, evacuation capacity, intercity coach stops, and local maintenance burden. |
| `resilience-before-crisis-report.md` | Climate Resilience Engineer, Traffic Engineer, Long-Haul Trucker, State DOT, Environmental Community | pass_with_risk | Correctly frames resilience as a service obligation and avoids crisis-proof or hardening-proof claims. | Add a hazard-source matrix for flood, wildfire, snow/ice, heat, storm surge, mountain closure, port disruption, and incident recovery, including time horizon and uncertainty treatment. |
| `maps-are-not-proof-report.md` | Schematic Cartographer, Optimization Methodologist, Traffic Engineer, Transit-Dependent, Citation Auditor | pass | Cleanly separates render validity, structural maps, held-claim surfaces, and evidence-valid claims. | Add a standard map caption pattern for decks: map level, claim label, excluded claims, and pointer to generated artifact/gate. |
| `requirement-to-refinement-demonstration-report.md` | Optimization Methodologist, Schematic Cartographer, State DOT, Freight Industry, Rural Advocate, Scope Keeper | pass_with_risk | Correctly presents the current demo as an artifact loop, not full optimizer proof. | Capture a concrete before/after fixture where one state, industry, rural, or community requirement changes an artifact, hold, or evidence label. |

## Supplemental Role Findings

| Role | Finding On New Reports | Disposition |
|---|---|---|
| Freight Economist | ROI discipline is credible because it permits weak or negative cases, but the next package needs actual commodity flow, delay, drayage, backhaul, and operating-cost source fields. | pass_with_risk |
| Rural Advocate | Rural access is not treated as leftover map coverage; however, future evidence must distinguish true rural access gaps from centroid, map, or source artifacts. | pass_with_risk |
| Rural Farmer | The rural report should eventually name harvest-window, weight-limit, grain elevator, processor, and export-terminal evidence fields before agricultural claims strengthen. | pass_with_risk |
| Rural Resident | Emergency and healthcare access are safely labeled as scenarios, but promotion needs trauma-center, evacuation, and employment-access evidence rather than generalized rural language. | pass_with_risk |
| State DOT Planner | All five reports preserve delivery boundaries. The next funder-facing package still needs match funding, maintenance, ROW, environmental review, lifecycle cost, and phasing fields. | pass_with_risk |
| Freight Industry / Regional Shipper | The ROI and demo reports are good guardrails, but industry intake must capture HOS, parking, WIM/PrePass, bridge weight, clearance, dwell, PTI, drayage, and intermodal constraints. | pass_with_risk |
| Long-Haul Trucker | Resilience and relay language should eventually account for HOS effects, safe parking, grade/weather closure, and legal rest options during disruption. | pass_with_risk |
| Climate Resilience Engineer | The resilience report names useful hazard classes but should add source families, time horizon, and uncertainty before any hazard-strengthened claim. | pass_with_risk |
| Environmental Community / Foxx | ROI and resilience reports preserve community/environmental review, but future templates should explicitly track air quality, noise, runoff, habitat, displacement, and who bears costs. | pass_with_risk |
| Schematic Cartographer | The maps report is well bounded. The practical next edit is reusable caption language so every deck map carries its proof posture. | pass |
| Optimization Methodologist | The demo report is honest about current limitations. The next prototype needs a reproducible before/after scenario, rejected/held row, and command evidence. | pass_with_risk |
| Citation Auditor | The five reports introduce no external numeric transportation claims. Future templates must require traceable source path, source title, year/date, and access note for every quantitative field. | pass |
| Numeracy Checker | No arithmetic, unit conversions, ROI ratios, cost totals, or traffic volumes are introduced. Future ROI and resilience work must state units, price year, time horizon, and uncertainty before values appear. | pass |
| Scope Keeper | All five reports match their declared `type: report` and remain communications/evidence-boundary artifacts, not design proposals or construction programs. | pass |

## Key Dissent / Productive Tensions

| Tension | Roles | Review Interpretation |
|---|---|---|
| Freight ROI vs. rural/equity access | Freight Economist vs. Rural Advocate / Foxx | The materials should not collapse all value into freight ROI. Keep freight reliability as the economic hook, but preserve rural, community, and access requirements as first-class plan inputs. |
| Vision map vs. evidence map | Schematic Cartographer vs. public pitch needs | The map should remain in the solution deck as a vision visual, but detailed map proof belongs in ROUTE technical materials and generated artifacts. |
| Ambitious relay story vs. operational proof | Freight Industry / Traffic Engineer / Numeracy Checker | Relay hubs are compelling, but labor, charging, safety, utilization, and AV claims must stay staged and evidence-gated. |
| National service promise vs. official guarantee | Citation Auditor / State DOT / Scope Keeper | Promise windows are useful planning targets; they must not be presented as guaranteed shipping times or official public commitments. |
| Funder excitement vs. delivery reality | State DOT / Moses-style delivery lens / Scope Keeper | The package should inspire, but the next round needs phasing, match funding, maintenance, ROW, and environmental review realism. |

## Material-Specific Review

| Material | Decision | Role Notes | Suggested Next Edit |
|---|---|---|---|
| `interstate-2-0-pitch.md` | pass_with_risk | Strong solution story; no visible ROUTE branding; service hierarchy works. | Add one "who is at the table" / community-intake slide or speaker-note section before external use. |
| `route-technology-story.md` | pass_with_risk | Clear ROUTE platform story; evidence labels and review system help. | Add one concrete demo slide: "requirement enters -> plan/evidence label changes." |
| `state-value-brief.md` | pass_with_risk | Good state posture; avoids official-plan overclaim. | Add delivery checklist: match, maintenance, ROW, environmental review, lifecycle cost, phasing. |
| `industry-value-brief.md` | pass_with_risk | Good operating model and relay framing. | Add explicit data ask: OD lanes, bottlenecks, dwell, HOS, parking, bridge/weight/clearance, WIM, charging. |
| `route-roi-cost-framework.md` | pass | Strong guardrail against fake numbers. | Keep as required gate before any numeric funding claim. |
| `roi-without-fake-numbers-report.md` | pass | Strong public explanation of why premature ROI is unsafe. | Add future ROI source-pack template before numeric work. |
| `rural-access-national-service-network-report.md` | pass_with_risk | Keeps rural, agricultural, emergency, terminal, and smaller-metro access visible without solving it by assertion. | Add rural/access evidence intake fields before external rural-access claims. |
| `resilience-before-crisis-report.md` | pass_with_risk | Strong resilience frame; avoids hardening and hazard-proof overclaim. | Add hazard-source matrix and time-horizon language. |
| `maps-are-not-proof-report.md` | pass | Excellent guardrail for schematic and publication-scope misuse. | Add reusable caption/note pattern to deck guide. |
| `requirement-to-refinement-demonstration-report.md` | pass_with_risk | Honest about current artifact loop and full before/after fixture gap. | Build and capture one before/after requirement scenario. |
| `route-evidence-posture.md` | pass | Strong claim-boundary artifact. | Keep updated whenever a deck or brief promotes a claim. |
| `research-conclusions.md` | pass_with_risk | Useful plain-English synthesis. | Add paper/review pointers later if research artifacts are formalized. |
| `COMMUNICATIONS_STRATEGY.md` | pass | Now controls the split deck and audience-surface backlog. | Add this review as the role-review evidence pointer. |

## Required Changes Before External Use

These were identified as required before external/published use. The current
draft package now addresses the P1 items at the communications level; each item
still requires source evidence or artifact-specific review before it can support
a concrete corridor, hub, ROI, operating, construction, or SLA-readiness claim.

| Priority | Required Change | Owner Lens | Target Surface | Draft Status |
|---|---|---|---|---|
| P1 | Add service-promise disclaimer: planning targets, not verified guarantees. | Citation Auditor / Scope Keeper | Deck notes, evidence posture, or solution deck appendix | addressed in solution deck, technology deck, state brief, industry brief, and evidence posture |
| P1 | Add state delivery checklist: match, maintenance, ROW, environmental review, phasing, lifecycle cost. | State DOT Planner | State brief / ROUTE tech deck | addressed in state brief and technology deck demo slide |
| P1 | Add industry data-ask checklist. | Freight Economist / Freight Industry | Industry brief / demo package | addressed in industry brief and technology deck demo slide |
| P1 | Add community/environmental intake language. | Foxx / Environmental Community | Solution deck / state brief | addressed in solution deck stakeholder slide and technology deck demo slide |
| P2 | Add map-use note: schematic hierarchy visual, detailed proof in generated artifacts. | Schematic Cartographer / V&V | Solution deck speaker notes / tech deck | addressed in solution deck and technology deck |
| P2 | Add one requirement-to-refinement demo script. | Optimization Methodologist | `docs/how-to/run-route-demo.md` or deck slide | addressed in technology deck and demo how-to; next stronger fixture still needed |
| P2 | Add hazard-class examples for resilience. | Climate Resilience Engineer | Solution deck / evidence posture | partially addressed through resilience examples; hazard-specific evidence campaign still needed |
| P2 | Add report-specific intake templates for ROI, rural access, resilience, map captions, and requirement-to-refinement demo evidence. | Freight Economist / Rural Advocate / Climate Engineer / Schematic Cartographer / Optimization Methodologist | New report slate / demo package / presenter guide | open; reports identify the fields but do not yet provide reusable templates |

## Claims Approved For Internal Draft Use

- Interstate 2.0 is a service-network vision, not just a map.
- The deck split is the right communications structure.
- ROUTE can be presented as the refinement/evidence engine behind the vision.
- State and industry requirements should be treated as plan inputs.
- Relay hubs are a compelling staged operating model.
- ROI/cost work is an evidence contract, not a current number.

## Claims Not Approved

- Any named corridor, hub, interchange, or standard has positive ROI.
- Any map proves SLA, upgrade, transit, terminal-access, or asset-condition
  readiness.
- Promise windows are guaranteed operating commitments.
- Relay hubs solve labor, EV, AV, safety, or utilization problems.
- State DOT delivery, environmental review, ROW, or maintenance issues are
  solved.
- Communities or stakeholders have validated the plan.

## Review Gate

Decision: **pass_with_risk**

Rationale: The materials are internally coherent, appropriately split by
audience, and evidence-bounded. The roles do not fully "agree" on priorities,
and that is useful: freight, state delivery, rural access, community impact,
environmental health, map truth, and optimization rigor each add a required next
step. The communications package can continue internally, but external use
should still wait for artifact-specific owner approval and any required source
evidence before concrete claims are used externally.
