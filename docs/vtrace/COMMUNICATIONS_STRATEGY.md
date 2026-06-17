# Communications Strategy

## Scope

Repo: ROUTE

VTRACE adoption scope: plan the user-facing communications package for ROUTE
without changing controlled requirements, specifications, implementation
claims, or public evidence status. This file governs how ROUTE should explain
the Interstate 2.0 research system, its current proof posture, and its future
decks, reports, research conclusions, ROI/cost narratives, and adoption
materials.

This strategy does not promote a transportation claim by itself. Any deck,
report, ROI, cost, or "why ROUTE matters" claim must trace to accepted source
IDs, generated artifacts, role review, and evidence posture before publication.

## Communications Thesis

ROUTE should be communicated as an evidence-bounded argument machine for
Interstate 2.0: score the existing network, expose gaps and tradeoffs, test
standards under pressure, and convert the surviving claims into reviewable
investment options.

The strongest message is not that ROUTE already knows which corridors should be
built. The message is that ROUTE can make national infrastructure arguments
auditable: every claim should show the data, command, confidence label,
stakeholder tension, and next evidence step that support or limit it.

## Audience Map

| Audience ID | Audience | Primary Question | Required Posture |
|---|---|---|---|
| AUD-001 | Portfolio sponsor / strategic reviewer | Why is ROUTE worth backing now? | Use evidence-bounded opportunity framing; no construction-readiness claims. |
| AUD-002 | Transportation analyst | What can I inspect, reproduce, or challenge? | Lead with corpus, maps, command outputs, source labels, and review records. |
| AUD-003 | State DOT / policy reviewer | What would make a claim actionable or not? | Separate analytical merit from funding, right-of-way, maintenance, and agency-readiness constraints. |
| AUD-004 | Freight, rural, transit, and environmental stakeholders | How are my tradeoffs represented? | Show role lanes, evidence labels, held claims, and dissent instead of consensus language. |
| AUD-005 | Research/publication reviewer | Which conclusions survived source, numeracy, and domain review? | Tie every conclusion to papers, reviews, source posture, and revision history. |
| AUD-006 | Game/system design audience | How does Interstate Tycoon use ROUTE without corrupting evidence? | Label mechanics as implemented, heuristic, simulated, planned, or held. |
| AUD-007 | Coding agent / maintainer | What docs or artifacts must change with a work package? | Use VTRACE IDs, work packages, validation commands, and ownership rows. |

## Surface Plan

| Surface ID | Source IDs | Audience | User Question | Generated Docs | Cadence | Owner | Status |
|---|---|---|---|---|---|---|---|
| COMMS-README-001 | NEED-001 / NEED-002 / SPEC-001 / SPEC-010 | AUD-001 / AUD-002 | What is ROUTE and where do I start? | `README.md` refresh plus `docs/README.md` map | every public docs wave | ROUTE maintainer | proposed |
| COMMS-ONEPAGER-001 | NEED-002 / NEED-003 / REQ-010 / VAL-004 | AUD-001 / AUD-003 | What is the concise strategic case without overclaiming? | `docs/decks/route-one-page.md` | before stakeholder review or portfolio showcase | ROUTE maintainer / Scope Keeper | proposed |
| COMMS-DECK-001 | NEED-001 through NEED-006 / OPS-001 through OPS-004 / WP-004 | AUD-001 / AUD-003 / AUD-004 | Why is the Interstate 2.0 solution fundable on its own terms? | `docs/decks/interstate-2-0-pitch.md` and `.pptx` | major milestone or review wave | ROUTE maintainer / review steward | draft; no visible ROUTE branding |
| COMMS-DECK-002 | NEED-001 through NEED-006 / OPS-001 through OPS-004 / WP-004 / WP-005 | AUD-001 / AUD-002 / AUD-003 / AUD-007 | How does ROUTE make the Interstate 2.0 plan inspectable, refinable, and evidence-bounded? | `docs/decks/route-technology-story.md` and `.pptx` | after solution pitch or technical sponsor review | ROUTE maintainer / review steward | draft |
| COMMS-PRESENTER-001 | COMMS-DECK-001 / COMMS-DECK-002 / COMMS-FUNDER-001 / COMMS-STATE-001 / COMMS-INDUSTRY-001 | AUD-001 / AUD-003 / AUD-004 / AUD-007 | How should presenters use the split decks without overclaiming? | `docs/decks/split-deck-presenter-guide.md` | before sponsor, state, industry, or funder meetings | ROUTE maintainer / Scope Keeper | draft |
| COMMS-REPORT-001 | SPEC-002 / SPEC-003 / SPEC-008 / EVID-002 / EVID-003 / EVID-008 | AUD-002 / AUD-005 | What conclusions are supported, held, or downgraded? | `docs/reports/route-evidence-posture.md` | after evidence campaign or release gate | research owner / Citation Auditor | draft |
| COMMS-DOCTRINE-001 | NEED-001 / NEED-002 / NEED-003 / REQ-010 / SPEC-010 / VAL-004 | AUD-001 / AUD-003 / AUD-004 / AUD-005 | What is the core Interstate 2.0 doctrine behind the decks and briefs? | `docs/reports/interstate-2-0-doctrine-report.md` | before external narrative package use | research owner / Scope Keeper | draft; official-plan and construction claims gated |
| COMMS-ROI-001 | NEED-003 / REQ-002 / REQ-008 / REQ-009 / SPEC-010 / VAL-004 | AUD-001 / AUD-003 / AUD-004 | What would count as ROI or cost evidence? | `docs/reports/route-roi-cost-framework.md` | before any ROI/cost deck claim | Freight Economist / Numeracy Checker | draft; ROI claims gated |
| COMMS-RESEARCH-001 | OPS-004 / SPEC-008 / EVID-008 / EVID-009 / EVID-010 | AUD-005 | Which research tracks have reviewable conclusions? | `docs/research-conclusions.md` or research-track briefs | after paper/review closeout | research owner / panel reviewers | draft; publication claims gated |
| COMMS-POLITICAL-001 | NEED-001 / NEED-002 / NEED-003 / REQ-010 / VAL-004 | AUD-001 / AUD-003 | How should elected officials talk about Interstate 2.0 without overclaiming? | `docs/briefs/political-value-brief.md` | before elected-official or sponsor meetings | Scope Keeper / Rural Advocate / Foxx / Freight Economist | draft; official-plan and construction claims gated |
| COMMS-STATE-001 | NEED-003 / REQ-008 / REQ-009 / SPEC-010 / VAL-004 | AUD-003 / AUD-004 | What do states get from Interstate 2.0 without implying a finished plan? | `docs/briefs/state-value-brief.md` | before state-facing intake or sponsor review | State DOT Planner / Scope Keeper | draft; corridor claims gated |
| COMMS-INDUSTRY-001 | NEED-003 / REQ-008 / REQ-009 / SPEC-010 / VAL-004 | AUD-004 | What does industry get from Interstate 2.0 and how can pain points refine the plan? | `docs/briefs/industry-value-brief.md` | before industry-facing intake or sponsor review | Freight Economist / Scope Keeper | draft; operating claims gated |
| COMMS-FUNDER-001 | NEED-003 / REQ-008 / REQ-009 / SPEC-010 / VAL-004 | AUD-001 / AUD-003 | What can a funder safely back before construction, ROI, or official-plan claims are ready? | `docs/briefs/funder-value-brief.md` | before funder-facing sponsor review | Freight Economist / Numeracy Checker / Scope Keeper | draft; numeric and construction claims gated |
| COMMS-REVIEW-001 | SPEC-008 / SPEC-009 / SPEC-010 / EVID-008 / EVID-009 / EVID-010 | AUD-001 / AUD-002 / AUD-003 / AUD-004 / AUD-005 | Do ROUTE roles agree with the current materials, and what do they require next? | `docs/reviews/communications-role-review.md` | before external communications package use | review steward / Scope Keeper | draft; external use gated by P1 changes |
| COMMS-PRESSURE-001 | COMMS-DECK-001 / COMMS-DECK-002 / COMMS-PRESENTER-001 / COMMS-REVIEW-001 / SPEC-010 | AUD-001 / AUD-003 / AUD-004 / AUD-007 | How does the package survive escalating stakeholder, state, regional, congressional, and DOT pressure? | `docs/reviews/communications-pressure-test-simulation.md` | before external rehearsal or sponsor dry run | review steward / Scope Keeper | draft; external use gated by P1 edits |
| COMMS-PRESSURE-RUN-001 | COMMS-PRESSURE-001 / COMMS-TRACE-001 / COMMS-DEMO-001 / COMMS-REPORT-001 | AUD-001 / AUD-003 / AUD-004 / AUD-007 | What happened when agents simulated the five-round pressure test, and what blocks a full pass? | `docs/reviews/communications-pressure-test-run-001.md` | after pressure-test rehearsal | review steward / Scope Keeper | draft; internal Round 5 rehearsal pass_with_risk; external readiness held |
| COMMS-PRESSURE-RUN-002 | COMMS-PRESSURE-RUN-001 / COMMS-ROLE-PASS-001 / COMMS-STAKEHOLDER-FIXTURE-001 / COMMS-ROUND5-EVIDENCE-001 | AUD-001 / AUD-002 / AUD-003 / AUD-004 / AUD-007 | Does the updated package pass the five-round ladder as an internal rehearsal, and what still blocks external use? | `docs/reviews/communications-pressure-test-run-002.md` | after pass-artifacts role review and fixture runbook | review steward / Scope Keeper | draft; internal five-round pass_with_risk; external/public readiness held |
| COMMS-EXTERNAL-READINESS-001 | COMMS-PRESSURE-RUN-002 / COMMS-STAKEHOLDER-FIXTURE-001 / COMMS-PRESENTER-001 / COMMS-REPORT-001 | AUD-001 / AUD-003 / AUD-004 / AUD-007 | What must close before a named external rehearsal can use the package? | `docs/reviews/communications-external-rehearsal-readiness.md` and `docs/templates/external-rehearsal-packet-template.md` | before any sponsor, state, industry, community, congressional, FHWA, or USDOT rehearsal | review steward / Scope Keeper | draft; hold_external_rehearsal; packet template exists |
| COMMS-CRATE-COVERAGE-001 | README.md / docs/SYSTEM_PLAN.md / docs/SPEC_INDEX.md / crate manifests / CLI command families | AUD-001 / AUD-002 / AUD-007 | Does the communications story represent the Rust workspace and where are the meaningful story gaps? | `docs/reviews/communications-crate-coverage-audit.md` | before selecting the next communications appendix | review steward / Scope Keeper | draft; partial_coverage |
| COMMS-SOURCEOPS-001 | COMMS-CRATE-COVERAGE-001 / docs/fletch-source-orchestration-spec.md / docs/source-fetch-cache-policy.md / evidence windows | AUD-002 / AUD-003 / AUD-007 | How does ROUTE turn source-needed claims into reviewable evidence tasks without overclaiming? | `docs/reports/source-operations-evidence-roadmap.md` | before technical sponsor or external rehearsal source discussion | route-data owner / Citation Auditor | draft; story-ready roadmap; claim promotion held |
| COMMS-OPTIMIZER-001 | COMMS-CRATE-COVERAGE-001 / docs/tier-optimizer-design.md / docs/optimizer-constraint-ledger-spec.md / docs/optimizer-artifact-manifest.md | AUD-002 / AUD-003 / AUD-007 | How does ROUTE turn service requirements into optimizer artifacts, held rows, and reviewable next steps? | `docs/reports/optimizer-evidence-appendix.md` | before technical sponsor or optimizer-focused rehearsal | route-network owner / Optimization Methodologist | draft; story-ready appendix; optimizer claims held |
| COMMS-BUNDLE-IDENTITY-001 | COMMS-CRATE-COVERAGE-001 / docs/route-architecture.md / docs/national-segment-identity-spec.md / docs/bundle-registry-spec.md / docs/tier-segment-stitching-spec.md | AUD-002 / AUD-003 / AUD-007 | Why are route labels not enough, and how does ROUTE preserve stable bundle/member/stitch identity? | `docs/briefs/bundle-identity-brief.md` | before technical sponsor, DOT, or map/simulation identity review | route-network owner / Scope Keeper | draft; story-ready brief; identity claims held |
| COMMS-T3T4-ACCESS-001 | COMMS-CRATE-COVERAGE-001 / docs/t3-t4-access-optimization.md / docs/reports/rural-access-national-service-network-report.md / docs/optimizer-constraint-ledger-spec.md / lower-tier access ledgers | AUD-002 / AUD-003 / AUD-004 / AUD-007 | How does ROUTE make rural, feeder, terminal, and local access pressure reviewable without overclaiming? | `docs/reports/t3-t4-access-evidence-appendix.md` | before rural, terminal, DOT, or lower-tier access review | route-network owner / Rural Advocate / Scope Keeper | draft; story-ready appendix; access claims held |
| COMMS-SIMGAME-001 | COMMS-CRATE-COVERAGE-001 / docs/game/interstate-tycoon-plan.md / docs/game/route-game-cli-design.md / data/pressure-test-scenarios.csv / data/game overlays / route-sim | AUD-002 / AUD-006 / AUD-007 | How can ROUTE use simulation and Interstate Tycoon without turning heuristic play into proof? | `docs/reports/simulation-game-evidence-boundary.md` | before game-facing release, browser demo, or scenario-heavy external rehearsal | game/system designer / Scope Keeper / V&V | draft; story-ready boundary; public readiness held |
| COMMS-STANDARDS-BLUEPRINT-001 | COMMS-CRATE-COVERAGE-001 / data/standards-proof-ledger.csv / data/blueprint-feature-packages.csv / data/blueprint-evidence-map.csv / docs/forum/standards-package-parliament.md | AUD-002 / AUD-003 / AUD-004 / AUD-007 | How does ROUTE downgrade standards and Blueprint packages before they become investment claims? | `docs/reports/standards-blueprint-gates-appendix.md` | before DOT, congressional, funder, or Blueprint-heavy review | Scope Keeper / State DOT Planner / Freight Economist | draft; story-ready appendix; investment claims held |
| COMMS-ASSET-EVIDENCE-001 | COMMS-CRATE-COVERAGE-001 / docs/tier-pavement-standards.md / data/tier-pavement-* / data/standards-l1-inventory.csv | AUD-002 / AUD-003 / AUD-004 / AUD-007 | How does ROUTE keep pavement, bridge, and asset-condition debt visible without promoting readiness claims? | `docs/reports/asset-condition-evidence-appendix.md` | before DOT, asset-condition, map/SLA, or Blueprint review | State DOT Planner / Scope Keeper / Citation Auditor | draft; story-ready appendix; asset claims held |
| COMMS-RELEASE-SCOPE-001 | COMMS-CRATE-COVERAGE-001 / docs/map-publication-scope.md / data/map-publication-* / data/release-manifest.csv / external rehearsal readiness | AUD-001 / AUD-002 / AUD-003 / AUD-007 | What can be shown, published, rehearsed, or released without implying public readiness? | `docs/reports/release-publication-scope-appendix.md` | before external rehearsal, public map use, browser/game demo, or release review | Scope Keeper / Schematic Cartographer / V&V | draft; story-ready appendix; public readiness held |
| COMMS-ROLE-PASS-001 | COMMS-REVIEW-001 / COMMS-PRESSURE-RUN-001 / COMMS-ROUND5-EVIDENCE-001 | AUD-001 / AUD-002 / AUD-003 / AUD-004 / AUD-007 | Do the pressure-test pass artifacts satisfy `.roles` review for another internal rehearsal? | `docs/reviews/communications-role-review-pass-artifacts.md` | after pass artifacts exist | review steward / Scope Keeper | draft; pass_with_risk for internal rehearsal; source-backed fixture held |
| COMMS-STAKEHOLDER-FIXTURE-001 | COMMS-ROLE-PASS-001 / COMMS-ROUND5-EVIDENCE-001 / COMMS-TRACE-001 | AUD-002 / AUD-003 / AUD-004 / AUD-007 | How does a real stakeholder requirement become a source-backed before/after fixture? | `docs/how-to/stakeholder-fixture-closeout-runbook.md` and `docs/templates/source-packs/stakeholder-fixture-source-pack-template.md` | before external rehearsal or stronger Round 5 claim | review steward / affected role lanes | draft; executable runbook exists; populated fixture pending |
| COMMS-MAPSTORY-001 | REQ-006 / REQ-007 / SPEC-006 / EVID-006 / EVID-007 / VAL-003 | AUD-001 / AUD-002 / AUD-004 | What do the current maps and SLA surfaces prove or not prove? | `docs/decks/route-map-story.md` and map-readiness explainer | after map/SLA package closeout | route-map owner / Schematic Cartographer | gated |
| COMMS-DEMO-001 | OPS-001 / OPS-003 / WP-003 / WP-005 | AUD-002 / AUD-007 | What command sequence demonstrates current capability? | `docs/how-to/run-route-demo.md` | when demo commands are stable | route-cli owner | draft |
| COMMS-TRACE-001 | TRACE.md / VERIFICATION.md / VALIDATION.md / EVIDENCE.md / REVIEW.md | AUD-002 / AUD-007 | How does a claim move from idea to evidence? | `docs/traces/route-claim-promotion-trace.md` | before Round 5 internal readiness pass | review steward | draft; required before technical readiness pass |
| COMMS-ROUND5-EVIDENCE-001 | COMMS-DEMO-001 / COMMS-TRACE-001 / COMMS-PRESSURE-RUN-001 / VERIFICATION.md | AUD-002 / AUD-007 | What command evidence, source-pack schemas, captions, and non-claims support Round 5 readiness? | `docs/evidence/round5-demo-capture.md`, `docs/templates/source-packs/`, presenter guide Round 5 controls | before Round 5 internal readiness pass | route-cli owner / review steward | draft; command bundle and threshold fixture captured; stakeholder fixture template exists; real source-backed fixture pending |
| COMMS-GAME-001 | NEED-003 / SPEC-010 / VAL-004 / COMMS-SIMGAME-001 | AUD-006 | What can Interstate Tycoon say without implying proof? | `docs/reports/simulation-game-evidence-boundary.md` and future `docs/game/interstate-tycoon-public-positioning.md` | before game-facing release | game/system designer / Scope Keeper | draft boundary exists; public positioning surface still pending |
| COMMS-CORPUS-001 | SPEC-001 / SPEC-012 / WP-001 | AUD-007 | Who owns each communications surface? | `docs/CORPUS.md` docs ownership section | once multiple surfaces exist | ROUTE maintainer | proposed |

## Message Architecture

| Message Layer | Approved Message | Required Evidence Posture | Do Not Say |
|---|---|---|---|
| Core value | ROUTE makes Interstate 2.0 claims traceable across data, maps, simulations, roles, and evidence. | May cite VTRACE source IDs and existing docs. | ROUTE proves the official national build plan. |
| Differentiator | ROUTE preserves dissent: freight, rural access, equity, delivery, climate, and map-truth concerns can hold or change claims. | Requires `.roles` and review-record references. | All stakeholders agree on one optimal answer. |
| Current capability | ROUTE has a Rust workspace, generated artifacts, stop/SLA/map gates, evidence ledgers, reviews, and research tracks. | Must label each named artifact as implemented, heuristic, planned, held, or risk-accepted. | Every map, cost, or ROI claim is publication-ready. |
| Investment story | ROUTE can identify where better evidence could change infrastructure priorities. | Requires claim-specific evidence and role review before external use. | ROUTE already justifies construction funding for named corridors. |
| ROI/cost story | ROUTE can define the evidence contract for ROI and cost analysis before estimating benefits. | Gated until sourced values, price-year rules, assumptions, and numeracy review exist. | A corridor has positive ROI without a cited model and cost basis. |

## Content Backlog

| Backlog ID | Deliverable | Purpose | Entry Criteria | Exit Criteria | Status |
|---|---|---|---|---|---|
| CONTENT-001 | ROUTE one-page strategic brief | Give sponsors the "why now" without overclaiming. | `COMMUNICATIONS_STRATEGY.md` accepted. | Scope Keeper confirms no construction, compliance, endorsement, or unsupported ROI claim. | ready_next |
| CONTENT-002 | Split pitch decks | Separate the public Interstate 2.0 solution pitch from the ROUTE technology/refinement-engine pitch. | One-page brief exists; selected screenshots/artifacts have evidence labels. | Solution deck has no visible ROUTE branding; technology deck maps claims to source IDs and review lanes. | draft |
| CONTENT-002A | Split deck presenter guide | Give presenters talk tracks, transitions, objections, red lines, and closing asks for the two decks. | Split decks and audience briefs exist. | Guide preserves public/ROUTE split and names safe/unsafe claims by audience. | draft |
| CONTENT-003 | Evidence posture report | Show what is implemented, heuristic, held, or blocked. | Current evidence rows and map/release inventories are selected. | Report distinguishes conclusions from open evidence debt. | draft |
| CONTENT-003A | Interstate 2.0 doctrine report | Define the flagship doctrine behind the public story: roads need a service hierarchy, staged evidence, and fundable refinement. | Research conclusions, SLA doctrine, tier doctrine, and evidence posture exist. | Report cites local source docs, names claim boundaries, and gives recommendations by adoption posture. | draft |
| CONTENT-004 | ROI/cost framework report | Define how ROI/cost claims will be calculated and reviewed. | Freight Economist, State DOT, Citation Auditor, and Numeracy Checker lanes selected. | Framework names model inputs, price-year rules, source classes, uncertainty treatment, and blockers. | draft; numeric claims gated |
| CONTENT-005 | Research conclusions index | Package research tracks into accepted, revised, held, or rejected conclusions. | Research-track owner selects papers and review records. | Conclusions table links to paper, review, evidence posture, and next evidence step. | draft; publication claims gated |
| CONTENT-005P | Political value brief | Give elected officials safe language, asks, and objection handling for Interstate 2.0. | Split public deck, presenter guide, and funder brief exist. | Brief names political messages, safe lines, asks, pushback responses, and blocked claims. | draft |
| CONTENT-005A | State value brief | Explain what states get and how state requirements refine the plan. | Split deck and ROI/cost framework exist. | Brief names state value, intake questions, staged asks, and blocked claims. | draft |
| CONTENT-005B | Industry value brief | Explain what industry gets and how operating pain points refine the plan. | Split deck and ROI/cost framework exist. | Brief names industry value, intake questions, staged asks, and blocked claims. | draft |
| CONTENT-005D | Funder value brief | Explain the safe near-term funding ask before construction or ROI claims. | Split deck, evidence posture, demo how-to, and ROI/cost framework exist. | Brief names fundable packages, safe wording, blocked claims, and evidence gates. | draft |
| CONTENT-005C | Communications role review | Review the materials through `.roles` and identify agreement, dissent, and next edits. | Split deck, briefs, evidence posture, ROI/cost, and research conclusions exist. | Review record names consensus, role tensions, material-specific suggestions, and external-use P1 changes. | draft; P1 draft changes addressed; external use remains evidence-gated |
| CONTENT-005C2 | Communications pass-artifacts role review | Review the pressure-test pass artifacts through `.roles` before another internal Round 5 rehearsal. | Trace, demo capture, source-pack templates, intake templates, regional packet, and verification gate exist. | Addendum names artifact decisions, role holds, claims approved for internal draft use, and remaining external-readiness blockers. | draft; pass_with_risk for internal rehearsal |
| CONTENT-005C3 | Stakeholder fixture closeout runbook | Make the remaining source-backed stakeholder fixture blocker executable without fabricating evidence. | Stakeholder fixture template and pass-artifacts role review exist. | Runbook defines source custody, before/after artifact change, role review, decisions, and closeout checklist. | draft; populated fixture pending |
| CONTENT-005E | Communications pressure-test simulation | Stress-test the package through local, state, regional, congressional, and DOT review rounds. | Split decks, reports, briefs, evidence posture, and role review exist. | Simulation names participants, failure modes, pass criteria, and edits needed before external review. | draft; P1 edits open |
| CONTENT-005F | Communications pressure-test run 001 | Record the agent-run verdict and backlog for passing the simulated review ladder. | Pressure-test simulation exists and agents review all five rounds. | Run record names round verdicts, objections, pass/fail risks, and backlog. | draft; internal Round 5 rehearsal pass_with_risk; external readiness held |
| CONTENT-005F2 | Communications pressure-test run 002 | Re-test the package after pass artifacts, role addendum, stakeholder fixture template, and closeout runbook. | Run 001, pass-artifacts role review, stakeholder fixture template, and runbook exist. | Run record names current five-round verdicts, remaining holds, and pass-to-review instructions. | draft; internal five-round pass_with_risk; external/public readiness held |
| CONTENT-005F3 | External rehearsal readiness checklist | Prevent internal simulation pass from being mistaken for external readiness. | Run 002 and presenter controls exist. | Checklist and packet template name venue, source, materials, role review, presenter controls, validation, pass conditions, and current gate. | draft; hold_external_rehearsal; packet template exists |
| CONTENT-005H | Communications crate coverage audit | Identify story gaps by comparing current communications package to implemented crate and command families. | Communications pressure-test stack and external readiness gate exist. | Audit names crate coverage, command-family gaps, risks, and recommended appendices. | draft; partial_coverage |
| CONTENT-005I | Source operations evidence roadmap | Make ROUTE's source acquisition, cache policy, FLETCH handoff, source health, snapshot-window, and proof-artifact workflow legible. | Crate coverage audit identifies source operations as the top story gap. | Report explains source-needed to source-backed workflow and keeps claim promotion held. | draft; story-ready roadmap; claim promotion held |
| CONTENT-005J | Optimizer evidence appendix | Explain the optimizer artifact chain beyond the 225-mile demo: promises, bundles, columns, stops, contacts, constraints, manifest, map/game hooks, and held-known rows. | Crate coverage audit identifies optimizer chain as a P1 story gap. | Appendix makes optimizer inspectability legible while holding final optimizer/construction/SLA claims. | draft; story-ready appendix; optimizer claims held |
| CONTENT-005K | Bundle identity technical brief | Explain why route labels are not enough and how bundle, member, stitch, alias, and state-scope identity prevents claim drift. | Crate coverage audit identifies bundle-first identity as a P1 story gap. | Brief gives reviewer pressure questions and red lines without promoting map, construction, SLA, ROI, release, compliance, or endorsement claims. | draft; story-ready brief; identity claims held |
| CONTENT-005L | T3/T4 access evidence appendix | Make lower-tier access machinery visible: zone obligations, feeder columns, terminal columns, held gaps, proof tasks, map exclusions, render boards, and stop placement. | Crate coverage audit identifies lower-tier access machinery as a P1 story gap. | Appendix ties rural and terminal access story to current artifacts while holding local-access, terminal, map-publication, promotion, SLA, ROI, and construction claims. | draft; story-ready appendix; access claims held |
| CONTENT-005M | Simulation and game evidence boundary | Explain which route-sim and Interstate Tycoon artifacts are implemented, heuristic, source-needed, held, or publication-gated. | Crate coverage audit identifies simulation/game evidence discipline as an open story gap. | Report separates scenario/game outcomes from proof claims and gives reviewer pressure questions for overlays, publication gates, browser readiness, and scenario labels. | draft; story-ready boundary; public readiness held |
| CONTENT-005N | Standards and Blueprint gates appendix | Explain standards proof ledgers, stakeholder classes, Blueprint package sequencing, evidence downgrades, cost/source posture, and next evidence steps. | Crate coverage audit identifies standards proof and Blueprint downgrade gates as a P1 story gap. | Appendix shows how weak claims are held or downgraded before investment framing without promoting policy, construction, SLA, ROI, eligibility, compliance, or endorsement claims. | draft; story-ready appendix; investment claims held |
| CONTENT-005O | Asset condition evidence appendix | Explain pavement, bridge, WIM, rest, local maintenance, source-access, repair-debt, funding-evidence, and downgrade/exclusion gates. | Crate coverage audit identifies pavement/bridge/asset evidence as a story gap. | Appendix makes asset debt legible while holding SLA, transit, map, upgrade, construction, ROI, eligibility, compliance, release, and endorsement claims. | draft; story-ready appendix; asset claims held |
| CONTENT-005Q | Release and publication scope appendix | Explain structural map publication, held-claim labels, release manifest metadata, exclusion rows, browser/game L2 holds, and external rehearsal readiness. | Crate coverage audit identifies release readiness as an underrepresented story gap. | Appendix separates map publication from public readiness and keeps scenario, browser, game, external rehearsal, SLA, ROI, construction, approval, and endorsement claims held. | draft; story-ready appendix; public readiness held |
| CONTENT-005G | Pressure-test pass artifacts | Add intake templates, regional packet, trace, source-pack templates, map captions, non-claims, and Round 5 verification gate. | Pressure-test run identifies readiness blockers. | Required artifacts exist and Round 5 gate identifies remaining captured-demo evidence. | draft; command bundle, threshold fixture, role addendum, stakeholder fixture template, and closeout runbook captured; real source-backed fixture pending |
| CONTENT-006 | Map/SLA story deck | Explain the stop-first network, map truth, and service promise surface. | Browser/map publication blocker resolved or explicitly excluded. | Schematic Cartographer and V&V review confirm the deck does not overstate publication readiness. | gated |
| CONTENT-007 | Maintainer demo how-to | Give analysts and agents a reproducible current demo path. | Demo command bundle selected. | Command output and expected artifacts are recorded or held. | draft |

## Research And Report Slate

| Priority | Report / Paper | Purpose | Claim Posture | Status |
|---|---|---|---|---|
| 1 | Interstate 2.0 Doctrine Report | Flagship narrative: roads need a service hierarchy, not one flat interstate category. | Story-ready; official-plan, construction, and final-route claims gated. | draft as `docs/reports/interstate-2-0-doctrine-report.md` |
| 2 | Relay Hubs: The Aviation Model For Freight | Explain driver exchange, charging, staging, maintenance, future AV handoff, and regional jobs. | Story-ready / heuristic; labor, EV, AV, utilization claims gated. | draft as `docs/reports/relay-hubs-aviation-model-report.md` |
| 3 | The 48-Hour Freight Promise | Make T1 national reliability legible to funders and industry. | Story-ready / heuristic; operating SLA proof gated. | draft as `docs/reports/forty-eight-hour-freight-promise-report.md` |
| 4 | ROI Without Fake Numbers | Explain benefit/cost categories, evidence gates, uncertainty, and why premature ROI is irresponsible. | Evidence contract; numeric ROI/cost claims gated. | draft as `docs/reports/roi-without-fake-numbers-report.md` |
| 5 | Rural Access In A National Service Network | Keep farm regions, production zones, small metros, emergency access, and rural resilience in the story. | Story-ready / heuristic; zone/access claims gated. | draft as `docs/reports/rural-access-national-service-network-report.md` |
| 6 | Resilience Before Crisis | Translate flood, wildfire, snow/ice, port disruption, mountain closures, and evacuation into fundable obligations. | Story-ready / heuristic; hazard and recovery proof gated. | draft as `docs/reports/resilience-before-crisis-report.md` |
| 7 | Maps Are Not Proof | Explain evidence-bounded infrastructure storytelling and why structural maps do not prove readiness. | Story-ready; map/SLA/upgrade claims gated. | draft as `docs/reports/maps-are-not-proof-report.md` |
| 8 | Requirement-To-Refinement Demonstration Report | Document the loop from requirement to threshold, artifact, candidate/hold, and evidence ask. | Implemented / heuristic; full optimizer before/after fixture gated. | draft as `docs/reports/requirement-to-refinement-demonstration-report.md` |

## Deck And Report Guardrails

- Use "evidence-bounded", "reviewable", "held", "source-needed",
  "confidence-limited", and "accepted with risk" when those are the true status.
- Every quantitative claim in a deck, report, ROI/cost note, or research
  conclusion must name its source path or be labeled as a placeholder.
- ROI/cost content must state the price year, cost basis, benefit categories,
  excluded benefits, uncertainty treatment, and review status before any summary
  number is promoted.
- Decks may use maps and game visuals as illustrations only when captions state
  whether they are structural, heuristic, simulated, held, or publication-ready.
- Stakeholder-facing materials must show at least one tradeoff or dissent row
  when promoting a route, standard, feature package, or investment thesis.
- Communications artifacts should cite ROUTE docs and evidence IDs instead of
  duplicating long product narratives.

## Derivation Rules

- Every mission need gets a communications disposition or an explicit
  internal-only rationale.
- Every important CONOPS scenario gets a how-to, trace walkthrough, stakeholder
  deck, or explicit no-docs-impact decision.
- Every public-facing requirement or specification gets a concept, reference,
  example, report section, or deck claim only after evidence posture is known.
- Every closed work package updates a release note, trace walkthrough, deck
  source row, report row, or `no communications impact` record.
- User-facing communications must not replace VTRACE requirements,
  specification baselines, verification, validation, evidence, or review gates.

## Review Checklist

| Item | Required | Decision | Evidence / Rationale |
|---|---|---|---|
| Communications surfaces trace to controlled source IDs. | yes | pass | Surface plan cites `NEED-*`, `OPS-*`, `REQ-*`, `SPEC-*`, `WP-*`, `VAL-*`, `EVID-*`, and VTRACE files. |
| Deck/report/ROI surfaces avoid unsupported public claims. | yes | pass_with_risk | ROI/cost, map-story, and publication-facing surfaces are gated until source, numeracy, V&V, and role review evidence exists. |
| Public claims preserve ROUTE scope. | yes | pass | Strategy frames ROUTE as research, tooling, review, design analysis, and evidence-bounded argument; it does not claim construction readiness, statutory compliance, official endorsement, or predictive authority. |
| Numeric claims are cited or absent. | yes | pass | This strategy introduces no new transportation quantities, costs, ROI values, volumes, distances, or benefit totals. |
| User-facing interfaces have expected docs disposition. | if applicable | pass_with_risk | Demo/how-to surfaces are planned, but command output evidence remains future work. |
| Docs ownership and cadence are visible. | yes | pass | Surface plan names owners and cadence; `docs/CORPUS.md` ownership update is a proposed follow-up. |

## Role Review Notes

| Role Lens | Communications Impact | Finding | Disposition |
|---|---|---|---|
| Scope Keeper | Strategy stays in communications planning and does not score a corridor, select a route, prescribe construction, or claim agency readiness. | No scope drift found. | pass |
| Citation Auditor | Strategy creates no new numeric transportation claims and gates future ROI/cost claims behind source tracing. | Future decks and reports require source paths before publication. | pass_with_risk |
| Numeracy Checker | Strategy includes no calculations, estimates, units, ROI values, cost totals, or benefit totals. | ROI/cost framework must define price year and arithmetic rules before any summary value. | pass_with_risk |
| Freight Economist | ROI/cost messaging is treated as an evidence contract, not a sales number. | Require model inputs, benefit classes, cost basis, uncertainty, and negative/marginal-result handling before ROI claims. | deferred_to_CONTENT-004 |
| State DOT Planner | Delivery and agency-readiness language is explicitly separated from analytical merit. | Future pitch materials must preserve funding, right-of-way, maintenance, and feasibility constraints. | pass_with_risk |
| Rural / transit / environmental stakeholders | Stakeholder materials must show tradeoffs and dissent instead of a single optimized answer. | Future content needs selected stakeholder lanes by claim type. | pass_with_risk |
| Schematic Cartographer / V&V | Map and SLA storytelling is gated until publication or explicit-held posture is clear. | Map-story deck remains gated by current browser/map validation risk. | pass_with_risk |

## Gate

Decision: pass_with_risk

Rationale: ROUTE now has a controlled communications strategy that can start the
one-page, deck, report, research-conclusion, ROI/cost, map-story, and demo-docs
pipeline without overclaiming the project. The residual risk is intentional:
most sales/adoption content is not yet publishable until source-backed claims,
role review, command evidence, and numeracy checks exist for the concrete
surface.

## Next Stage

Recommended next one-file deliverable: `docs/decks/route-one-page.md` as
`CONTENT-001`, or `docs/reports/route-roi-cost-framework.md` as `CONTENT-004`
if the priority is ROI/cost discipline before pitch material.
