---
name: ROUTE Communications Pressure Test Simulation
slug: route-communications-pressure-test-simulation
type: review
status: draft
rubric_version: v1.0
author: copilot
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
  - docs/decks/interstate-2-0-pitch.md
  - docs/decks/route-technology-story.md
  - docs/decks/split-deck-presenter-guide.md
  - docs/briefs/political-value-brief.md
  - docs/briefs/state-value-brief.md
  - docs/briefs/industry-value-brief.md
  - docs/briefs/funder-value-brief.md
  - docs/reports/route-evidence-posture.md
  - docs/reports/route-roi-cost-framework.md
  - docs/reports/roi-without-fake-numbers-report.md
  - docs/reports/rural-access-national-service-network-report.md
  - docs/reports/resilience-before-crisis-report.md
  - docs/reports/maps-are-not-proof-report.md
  - docs/reports/requirement-to-refinement-demonstration-report.md
  - docs/reviews/communications-role-review.md
  - https://transportation.org/meetings/regional-associations/
  - https://highways.dot.gov/about/field-offices
---

# ROUTE Communications Pressure Test Simulation

## Scope

This simulation stress-tests the current Interstate 2.0 / ROUTE communications
package through escalating review venues:

1. intra-state regional planning meeting;
2. state DOT / governor-sponsor meeting;
3. multi-state regional meeting;
4. congressional hearing;
5. FHWA / USDOT technical review.

The simulation uses `.roles` and stakeholder lenses from ROUTE. It does not
represent real approval, endorsement, agency review, statutory compliance,
construction readiness, guaranteed service levels, or positive ROI.

## Regional Organization Choice

Use AASHTO regional associations as the simulated multi-state regional layer.

AASHTO is the state-DOT-centered venue that best matches this package: it is
state-led, transportation-specific, and already organizes regional associations
for state DOT coordination. FHWA division offices fit the final technical-review
layer because FHWA operates division offices serving state and federal
transportation program coordination.

Do not use Supreme Court circuits as the primary regional layer. They are
legible for legal audiences, but they are not transportation planning venues and
would make the package feel procedurally artificial.

## Simulation Rules

| Rule | Meaning |
|---|---|
| No endorsement | A simulated pass means "ready for the next review round," not official support. |
| No construction ask | Every round closes with story, intake, evidence, demo, source-pack, or pilot asks. |
| No numeric ROI | ROI remains an evidence contract until price year, source pack, uncertainty, and review close. |
| No guaranteed SLA | T1/T2/T3/T4 promise windows are planning targets. |
| No map proof | Maps are structural surfaces unless claim-specific evidence closes. |
| Dissent is signal | Stakeholder objections become requirements, holds, or evidence asks. |

## Package Under Test

| Package Surface | Role In Simulation |
|---|---|
| `interstate-2-0-pitch.md` | Primary public vision story. |
| `route-technology-story.md` | Technical follow-up showing refinement and evidence mechanics. |
| `split-deck-presenter-guide.md` | Red lines, talk tracks, and objection handling. |
| Audience briefs | Political, state, industry, and funder variants. |
| Report slate | Doctrine, relay hubs, 48-hour freight, ROI, rural access, resilience, maps, and demo reports. |
| Evidence posture | Claim boundary for every round. |
| Role review | Dissent and pass-with-risk basis. |

## Round 1: Intra-State Regional Planning Meeting

### Venue

Regional planning organization, MPO/RPO, freight advisory committee, or
state-defined district meeting inside one state.

### Simulated Participants

| Participant | Lens |
|---|---|
| Regional planner / MPO staff | Wants local requirements captured without being overrun by a national map. |
| County commissioner / local official | Wants interchange, tax base, and community impact visible. |
| Rural farmer / rural resident | Wants farm access, healthcare access, evacuation, and local roads represented. |
| Long-haul trucker / regional shipper | Wants parking, HOS, reliability, bridge/weight/clearance, and terminal access captured. |
| Environmental community | Wants noise, runoff, air quality, habitat, and displacement before concrete. |
| ROUTE presenter | Uses the public deck first, then the demo/report package only as needed. |

### Simulated Objections

| Objection | Risk If Mishandled | Safe Response |
|---|---|---|
| "Is this map saying our county gets bypassed?" | Map becomes a final-plan claim. | "No. The map is a structural service surface. Your access concern becomes a T3/T4 obligation or held evidence row." |
| "Where are the actual projects?" | Audience pushes for construction claims. | "This round is requirements before concrete. The output is an intake and evidence package." |
| "Freight gets everything again." | Rural/equity failure. | "Freight reliability is one hook, but rural access, emergency access, community health, and local burden are first-class review lanes." |
| "How do I know this changes anything?" | Demo credibility gap. | "The current demo shows thresholds, diagnostics, candidate dockets, and holds; the next fixture should show your requirement changing an artifact." |

### Failure Modes

| Failure Mode | Trigger |
|---|---|
| local_fail_map_overclaim | Presenter implies map line is selected, final, or proof-grade. |
| local_fail_no_intake | Meeting ends without named requirements or evidence asks. |
| local_fail_freight_only | Rural, transit, environmental, or local-official concerns are treated as secondary. |
| local_fail_project_promise | Presenter implies construction, funding, or delivery is next. |

### Pass Criteria

| Pass Item | Evidence To Record |
|---|---|
| Local requirements captured as fields. | Rural access, terminal access, safety, parking, community, environment, and delivery rows. |
| Map posture stated. | Caption or meeting note says structural/held-claim surface. |
| One dissent row preserved. | Concern becomes a hold, evidence ask, or role-review note. |
| Next ask is bounded. | Fund intake package, evidence package, or demo fixture, not construction. |

### Feedback To Pass Next Round

| Feedback | Package Change |
|---|---|
| Local officials need a visible intake form. | Create a one-page local/regional intake template with field names from rural, freight, environmental, and delivery roles. |
| Rural stakeholders need examples. | Add one rural example card to the presenter guide: harvest access, trauma-center access, evacuation, or farm-to-terminal movement. |
| Demo needs a before/after. | Select one local requirement to become the before/after fixture in the demo package. |

## Round 2: State DOT / Governor-Sponsor Meeting

### Venue

State DOT planning leadership, governor infrastructure office, state freight
advisory committee, and state legislative transportation staff.

### Simulated Participants

| Participant | Lens |
|---|---|
| State DOT planner | Funding eligibility, match, maintenance, ROW, environmental review, federal process. |
| Governor policy staff | Jobs, resilience, regional fairness, state competitiveness. |
| State freight office | Reliability, terminal access, truck parking, intermodal, industry data. |
| Environmental / civil rights staff | Distributional burden and community review. |
| ROUTE presenter | Uses state brief, evidence posture, ROI reports, and demo report. |

### Simulated Objections

| Objection | Risk If Mishandled | Safe Response |
|---|---|---|
| "Are you asking us to endorse a national build map?" | Official-plan overclaim. | "No. The state ask is to add requirements, evidence, and delivery constraints before any option is promoted." |
| "Who pays for maintenance forever?" | Delivery realism failure. | "Maintenance and lifecycle cost are required cost classes before ROI or funding claims." |
| "Does this compete with our STIP/LRTP priorities?" | State authority conflict. | "ROUTE can map state priorities to national service claims; it does not replace the state process." |
| "Where is the benefit-cost analysis?" | Fake ROI pressure. | "Current ROI is an evidence contract. Numeric ROI is blocked until sources, price year, uncertainty, and review exist." |

### Failure Modes

| Failure Mode | Trigger |
|---|---|
| state_fail_authority | Presenter implies state DOT is subordinate to ROUTE. |
| state_fail_delivery | ROW, match, maintenance, environmental review, or lifecycle cost is missing. |
| state_fail_roi | Numeric ROI is implied without source pack. |
| state_fail_no_state_payload | State leaves without knowing what to provide. |

### Pass Criteria

| Pass Item | Evidence To Record |
|---|---|
| State delivery checklist accepted as required input. | Match, maintenance, ROW, environmental, lifecycle, phasing, owner. |
| State requirements produce artifact targets. | Stop/SLA, T3/T4 access, resilience, terminal, or ROI source-pack rows. |
| ROI remains gated. | No numeric ROI or cost claim appears. |
| State authority preserved. | Meeting note says ROUTE supports planning review; it does not become official plan. |

### Feedback To Pass Next Round

| Feedback | Package Change |
|---|---|
| State wants a payload for regional peers. | Produce a "state-to-AASHTO regional packet" outline: shared corridors, freight lanes, resilience risks, evidence asks, and non-claims. |
| Delivery constraints need standard fields. | Add delivery fields to the intake template and demo scenario. |
| Funder ask needs smaller slices. | Split next asks into intake, evidence, demo, source pack, and pilot packages. |

## Round 3: AASHTO Regional Association Meeting

### Venue

AASHTO regional association meeting, simulated as a multi-state DOT peer review.

### Simulated Participants

| Participant | Lens |
|---|---|
| DOTs from neighboring states | Cross-border service, maintenance, funding, and governance. |
| Regional freight / port / border stakeholders | OD lanes, drayage, terminals, border/port access, reliability. |
| Rural / environmental / equity observers | Distribution of access, burdens, and mitigation. |
| Schematic Cartographer / Optimization Methodologist | Map truth, identity, constraints, candidate/hold logic. |
| ROUTE presenter | Uses split decks, state brief, industry brief, maps report, ROI report, and role review. |

### Simulated Objections

| Objection | Risk If Mishandled | Safe Response |
|---|---|---|
| "Why does your state get the spine?" | Political allocation fight. | "T1/T2/T3/T4 are service roles; selection claims require source-backed regional evidence and sensitivity." |
| "This map makes our connector look secondary." | Map hierarchy misread. | "The map is a structural service surface. Regional requirements can reopen treatment only through contact, access, or promise evidence." |
| "Who governs cross-border promises?" | Governance gap. | "This package does not create governance. It creates evidence contracts and pilot candidates for states to review." |
| "Where are shared standards?" | Region lacks concrete next step. | "The next pass is a regional evidence/source-pack standard: reliability, access, delivery, and resilience fields." |

### Failure Modes

| Failure Mode | Trigger |
|---|---|
| region_fail_home_state_bias | One state appears to be selling its map to others. |
| region_fail_no_sensitivity | No way to show how assumptions change tiers or holds. |
| region_fail_map_status | Structural map is treated as regional proof. |
| region_fail_no_cross_border_payload | No shared evidence packet emerges. |

### Pass Criteria

| Pass Item | Evidence To Record |
|---|---|
| Regional packet uses shared fields, not one-state claims. | OD lanes, terminal access, reliability, rural access, resilience, delivery, and holds. |
| Map proof boundary survives. | Every map is captioned as structural/held unless evidence-valid. |
| Dissent is recorded by state or stakeholder lane. | Peer objections become assumptions, sensitivity asks, or held rows. |
| One regional pilot candidate is bounded. | Candidate is intake/evidence/demo/pilot only, not construction. |

### Feedback To Pass Next Round

| Feedback | Package Change |
|---|---|
| Region needs a nonpartisan vocabulary. | Lead congressional materials with service hierarchy, evidence, and regional coordination rather than state wins. |
| Sensitivity needs to be visible. | Add a "what changes the answer?" section to the technical deck or appendix. |
| Governance is unresolved. | Add a governance non-claim: ROUTE does not create authority; it structures evidence for existing authorities. |

## Round 4: Congressional Hearing

### Venue

House or Senate transportation/infrastructure hearing, member briefing, or staff
roundtable.

### Simulated Participants

| Participant | Lens |
|---|---|
| Committee chair / ranking member | National interest, federal role, budget discipline. |
| Member from rural district | Rural access, agriculture, healthcare, evacuation. |
| Member from freight/port state | Freight reliability, jobs, ports, industrial competitiveness. |
| Member focused on equity/environment | Community harms, emissions, displacement, mitigation. |
| Fiscal staff | ROI, cost, price year, federal exposure, negative cases. |
| ROUTE presenter | Uses public deck, political brief, funder brief, ROI without fake numbers, evidence posture. |

### Simulated Objections

| Objection | Risk If Mishandled | Safe Response |
|---|---|---|
| "Are you asking Congress to fund a new interstate system?" | Construction-program overclaim. | "No. The ask is standards, intake, evidence, demo, source packs, and pilots before major commitments." |
| "Why should taxpayers trust this map?" | Map proof trap. | "They should not trust a map alone. The package is built so maps cannot outrun evidence labels and review." |
| "What is the ROI?" | Fake-number pressure. | "The responsible answer is source pack first: price year, benefits, costs, uncertainty, exclusions, and negative cases." |
| "How do communities avoid repeating past highway harms?" | Equity failure. | "Community health, displacement, air, noise, runoff, habitat, and access are review lanes before feature promotion." |

### Failure Modes

| Failure Mode | Trigger |
|---|---|
| congress_fail_mega_project | Package sounds like a blank-check construction program. |
| congress_fail_fiscal | ROI discipline is not strong enough for budget staff. |
| congress_fail_equity | Community and environmental costs are treated as afterthoughts. |
| congress_fail_no_federal_role | Ask does not fit standards, evidence, pilots, or program oversight. |

### Pass Criteria

| Pass Item | Evidence To Record |
|---|---|
| Federal ask is bounded. | Story, intake, evidence, demo, source-pack, pilot, or standard-setting package. |
| ROI fake-number pressure resisted. | No numeric ROI; future source-pack gate named. |
| Equity and community lanes are explicit. | Distributional burden and mitigation fields appear. |
| The map is not the proof. | Hearing script includes map-proof red line. |

### Feedback To Pass Next Round

| Feedback | Package Change |
|---|---|
| Congress wants accountability mechanics. | Add claim-promotion trace: claim, source, artifact, role review, gate, blocker, decision. |
| Fiscal staff wants budget discipline. | Add negative/marginal-result handling to every funding ask. |
| Members need district-safe language. | Add "what this does not authorize" slide/note before any hearing use. |

## Round 5: FHWA / USDOT Technical Review

### Venue

FHWA division/technical review, USDOT policy staff review, or federal program
eligibility screening.

### Simulated Participants

| Participant | Lens |
|---|---|
| FHWA division / program staff | Federal-aid process, eligibility, design standards, evidence, stewardship. |
| Safety / operations reviewer | Geometry, capacity, safety, reliability, incident management. |
| Planning / environment / civil rights reviewer | Planning consistency, NEPA posture, equity, public involvement. |
| Finance / grant reviewer | Cost basis, benefit-cost assumptions, federal share, lifecycle. |
| ROUTE maintainer | Uses evidence posture, VTRACE docs, demo runbook, maps report, ROI framework, role review. |

### Simulated Objections

| Objection | Risk If Mishandled | Safe Response |
|---|---|---|
| "Where is the statutory or program eligibility basis?" | Compliance overclaim. | "ROUTE does not claim eligibility. It identifies evidence needed before a sponsor can make that case." |
| "Do these routes meet design standards?" | Design-readiness overclaim. | "No design-standard compliance is claimed. Standards would be a future source and engineering review gate." |
| "Where are command outputs and reproducibility?" | Technical credibility failure. | "The current package has a demo runbook; stronger review needs captured command outputs and artifacts." |
| "Where is NEPA/public involvement?" | Public-process overclaim. | "Not claimed. Community/environmental concerns are intake and review lanes before promotion." |

### Failure Modes

| Failure Mode | Trigger |
|---|---|
| dot_fail_compliance | Package implies legal, environmental, safety, or design compliance. |
| dot_fail_reproducibility | Demo claims lack command outputs and artifacts. |
| dot_fail_source_custody | Numeric or external source fields lack traceability. |
| dot_fail_claim_trace | Claims cannot be walked from requirement to evidence to review. |

### Pass Criteria

| Pass Item | Evidence To Record |
|---|---|
| Claim-promotion trace exists. | Requirement, artifact, source, evidence label, role review, blocker, next step. |
| Demo outputs are captured. | Command, output path, expected rows, pass/hold status. |
| Compliance non-claims are explicit. | No legal, design, environmental, safety, or eligibility claim is made. |
| Source custody rules exist. | Every future quantitative field requires source path/title/date/access note. |

### Feedback To Pass Next Round

| Feedback | Package Change |
|---|---|
| Technical reviewers need a trace walkthrough. | Produce `docs/traces/route-claim-promotion-trace.md`. |
| Demo needs reproducible evidence. | Capture command outputs under controlled artifact path and summarize. |
| Source packs need schema. | Add source-pack templates for ROI, resilience, rural access, map claims, and demo evidence. |

## Cross-Round Pass Ladder

| Round | Pass Means | Next Package To Produce |
|---|---|---|
| Intra-state regional | Local concerns become requirements, holds, or evidence asks. | Local/regional intake template. |
| State | State delivery and authority constraints are visible. | State-to-regional packet outline. |
| AASHTO regional | Multi-state evidence fields and dissent are visible. | Regional evidence/source-pack standard. |
| Congressional | Federal ask is bounded and accountable. | Claim-promotion trace and "does not authorize" note. |
| FHWA / USDOT | Technical reviewers can reproduce and trace claims. | Captured demo evidence and source-pack schemas. |

## Required Package Edits Before Simulated External Use

| Priority | Edit | Owner Lens | Target |
|---|---|---|---|
| P1 | Local/regional intake template with rural, freight, environmental, delivery, and access fields. | Rural Advocate / Freight Industry / Environmental Community / State DOT | briefs or `docs/how-to/` |
| P1 | Standard map caption pattern: map level, claim label, excluded claims, artifact/gate pointer. | Schematic Cartographer / Citation Auditor | presenter guide / decks |
| P1 | "What this does not authorize" note for political and congressional settings. | Scope Keeper / State DOT | presenter guide / political brief |
| P1 | Claim-promotion trace walkthrough. | Optimization Methodologist / Citation Auditor | `docs/traces/route-claim-promotion-trace.md` |
| P2 | State-to-AASHTO regional packet outline. | State DOT / Freight Economist | state brief / regional appendix |
| P2 | Hazard-source matrix with time horizon and uncertainty fields. | Climate Resilience Engineer | resilience report/source-pack template |
| P2 | Before/after requirement-to-refinement fixture with captured command outputs. | route-cli / Optimization Methodologist | demo runbook / demo evidence |
| P2 | ROI source-pack schema with price year, source family, exclusions, uncertainty, and negative case. | Freight Economist / Numeracy Checker | ROI reports |

## Simulated Final Decision

Decision: **pass_with_risk for internal escalation simulation**

The package can be pressure-tested through the five-round ladder without
breaking evidence boundaries if presenters follow the red lines. It is not ready
for real external review as a claim package until the P1 edits exist,
especially local intake, map caption pattern, non-authorization note,
claim-promotion trace, and captured demo evidence.

## Non-Approved Claims

- Interstate 2.0 is an official adopted plan.
- Any corridor, hub, interchange, or standard is construction-ready.
- Any service window is a guaranteed operating SLA.
- Any map proves SLA, upgrade, terminal, asset, or environmental readiness.
- Any corridor, hub, package, or standard has positive ROI.
- State, regional, congressional, FHWA, or USDOT participants endorsed the
  package.
