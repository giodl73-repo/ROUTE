# ROUTE Spec Index

Start here when deciding which document owns a claim.

| Document | Owns | Use when |
|---|---|---|
| `docs/SYSTEM_PLAN.md` | Living roadmap, Milepost phases, roles, truth labels, forward plan | You need the current operating plan |
| `docs/DIMENSIONS.md` | Current 16-dimension registry, evidence path, and truth label | You need the canonical rubric dimension list |
| `docs/STANDARDS_EVALUATION.md` | Standards proof obligations, pressure-test gates, and T1/T1 evaluation framing | You need to know whether a tier standard has earned its place |
| `specs/2026-05-10-tier-node-service-standard.md` | Route-tier and node-class service standard for deciding which routes/stops deserve national schematic prominence | You need to decide whether a route is a T1 trunk, T2 connector, T3 feeder, or T4 local spur |
| `docs/tier-optimizer-design.md` | Constraint ordering and optimizer loop for selecting tier lines, stops, contacts, and Beck layout from SLA promises | You need the overall route/stop/T2 ordering before changing selectors or map topology |
| `docs/route-stop-column-schema.md` | Common schema vocabulary for promise-pair, route, stop, service, repair, graph, and manifest columns | You need to add or refactor optimizer artifacts without inventing a one-off CSV contract |
| `docs/t2-regional-treatment.md` | Doctrine for T2 regional service treatment, parent-trunk lineage, duplicate handling, contacts, relief loops, and upward/downward pressure | You need to change T2 regionalization, service selection, colors, or duplicate/demotion rules |
| `docs/t3-t4-access-optimization.md` | Doctrine for T3/T4 zone access, terminal/local obligations, gap classes, map treatment, and upward pressure rules | You need to build regional feeder maps, local access ledgers, or lower-tier bubble-up logic |
| `docs/optimizer-artifact-manifest.md` | Contract for optimizer run manifests, gate statuses, held-known blockers, row counts, and bundle pass rules | You need to add optimizer stages or interpret `data/tier-optimizer-runs.csv` |
| `docs/tier-node-evaluation-design.md` | Implementation design for `route tier-connectivity` and route/node demotion candidates | You need to run or extend the graph-based tier connectivity evaluation |
| `docs/beck-renderer-contract.md` | Contract for optimizer-fed Beck rendering, topology truth, bend/stop rules, diagnostics, and map-atlas limits | You need to change schematic rendering, diagnose map cheats, or decide what the renderer may distort |
| `docs/sla-promise-portfolio.md` | Doctrine for selecting T1 48h/36h SLA promise pairs and deciding when lower-tier pressure can reopen T1 | You need to add, drop, rank, or defend national promise pairs |
| `docs/milepost-4-closeout.md` | Milepost 4 closure decision, passing pressure gates, and held Blueprint/publication claims | You need to know why Pressure Test can close while Blueprint remains locked |
| `docs/milepost-5-closeout.md` | Milepost 5 Forum closeout, review records, explicit holds, and Blueprint intake rules | You need to know what constraints Blueprint inherits from Forum |
| `docs/milepost-6-closeout.md` | Milepost 6 Blueprint closeout, gate bundle, remaining held claims, and Milepost 7 handoff | You need to know why Blueprint can close without proving held infrastructure claims |
| `docs/milepost-7-plan.md` | Milepost 7 Program plan, tasklist, done criteria, and release surface | You need the current plan for release/reproducibility work |
| `docs/milepost-7-closeout.md` | Milepost 7 Program closeout, release gate result, release policy, and remaining held claims | You need to know whether the current release candidate is reproducible |
| `docs/milepost-8-plan.md` | Milepost 8 Evidence Campaign checklist, candidate holds, recommended first target, and done criteria | You need the next source-validation campaign plan |
| `docs/milepost-8-closeout.md` | Milepost 8 closeout for the T1/T1 failure evidence campaign, improved hold, source attempt result, and next evidence step | You need to know why the selected hold remains held after source work |
| `docs/milepost-9-plan.md` | Milepost 9 Evidence Operations checklist, repeat-window target, snapshot-history guard, and done criteria | You need the next operating plan for turning the T1/T1 improved hold into repeatable evidence work |
| `docs/milepost-9-closeout.md` | Milepost 9 Evidence Operations closeout, gate result, source-window guard, and continued T1/T1 hold | You need to know why evidence operations closed without promoting T1/T1 diamond recovery |
| `docs/milepost-10-plan.md` | Recursive Tier Optimizer goal, tasklist, done criteria, and first implementation slices | You need the current plan for making T1/T2/T3/T4 line and stop selection algorithmic |
| `data/significant-moments.csv` | Flair ledger for major conceptual breakthroughs, their artifacts, commits, and next threads; gated by `route significant-moments --gate` | You need to remember why the system changed direction or preserve a major design insight |
| `docs/significant-moments.md` | Human-readable guide for adding significant-moment flairs | You need to decide whether a breakthrough belongs in the moments ledger |
| `docs/evidence-campaigns/milepost-8-target.md` | Milepost 8 target decision for T1/T1 failure evidence, scope, expected outcome, and non-goals | You need to know which held claim the campaign is working |
| `docs/evidence-campaigns/milepost-8-source-attempt.md` | Milepost 8 A-band source attempt record for Iowa 511 and INDOT TrafficWise T1/T1 failure evidence | You need to know what source access produced and why the hold remains |
| `data/evidence-campaign-source-plan.csv` | Milepost 8 campaign source checklist for A/B-band T1/T1 failure evidence feeds | You need the campaign-specific source acquisition plan |
| `data/t1-evidence-windows.csv` | Milepost 9 source-window ledger for T1/T1 evidence operations, freshness, and snapshot-history promotion guards | You need to know whether T1/T1 observations are snapshot-only, repeated-window, historical, or blocked |
| `docs/evidence-campaigns/milepost-9-iowa-repeat-window.md` | Milepost 9 executable Iowa 511 repeated-polling path and promotion limits | You need to run or review the Des Moines T1/T1 repeated-window source path |
| `docs/evidence-campaigns/milepost-9-indot-ohgo-enrichment.md` | Milepost 9 INDOT/OHGO enrichment path for the I-80/I-90 shared-corridor blocker | You need to understand why INDOT is source-accessible but not observation-grade yet |
| `docs/evidence-campaigns/milepost-9-snapshot-history-guard.md` | Milepost 9 CLI guard that prevents snapshot-only rows from becoming annual or recovery-grade evidence | You need the promotion rules for evidence-window rows |
| `docs/reviews/milepost-9-evidence-operations-review.md` | Milepost 9 review decision for evidence-window metadata, Iowa repeat path, INDOT/OHGO blocker, and continued hold | You need the review decision before changing T1/T1 failure or Blueprint claim status |
| `docs/reviews/beck-sla-spec-role-review.md` | Role-persona review of the Beck renderer contract and SLA promise portfolio, including optimizer and schematic-cartography role gaps | You need the review decision before treating those two specs as doctrine |
| `docs/reviews/t3-t4-access-optimization-review.md` | Role-persona review of the T3/T4 access optimization spec and its implementation holds | You need the review decision before treating lower-tier access optimization as implemented |
| `docs/reviews/milepost-9-closeout-review.md` | Closeout review for Milepost 9 metadata consistency, release verification safety, and residual risks | You need the post-closeout QA review for the evidence operation |
| `docs/reviews/milepost-8-t1-failure-evidence-review.md` | Milepost 8 source/evidence review deciding that T1/T1 diamond recovery remains held after Iowa/INDOT source attempts | You need the review decision before changing T1/T1 claim status |
| `docs/reviews/milepost-8-closeout-review.md` | Closeout review for Milepost 8 metadata consistency, source-plan wording, and residual risks | You need the post-closeout QA review for the evidence campaign |
| `docs/reviews/milepost-5-7-source-release-review.md` | Review of Milepost 5-7 source traceability, held outputs, release gate coverage, and residual risks | You need the post-closeout source/release review findings |
| `docs/release/release-checklist.md` | Release policy for public, held, internal, and source-needed artifacts plus local release steps | You need to decide whether an artifact can be published |
| `data/release-manifest.csv` | Release artifact manifest with owner milepost, public status, verification command, and notes | You need to verify the current release surface |
| `docs/blueprint/milepost-6-plan.md` | Milepost 6 Blueprint slice plan, Forum intake rules, done criteria, and package spine | You need the current plan for finishing Blueprint |
| `docs/blueprint/feature-packages.md` | Human-readable Blueprint package taxonomy and package briefs tied to the package ledger | You need to understand how package classes should be used before promotion |
| `docs/blueprint/phase-sequence.md` | Blueprint phase order and promotion rules for Phase 0/1/2 packages | You need the evidence order before interpreting package sequence as investment scope |
| `data/blueprint-feature-packages.csv` | Milepost 6 feature-package ledger with stakeholder class, evidence status, Forum constraint, mitigation fields, blockers, and next evidence step | You need to gate Blueprint packages before design-spec promotion |
| `data/blueprint-evidence-map.csv` | Package-to-standard downgrade map linking Blueprint claims to standards proof rows, Forum holds, blockers, and promotion rules | You need to verify Blueprint is not promoting held or heuristic evidence |
| `data/blueprint-cost-ranges.csv` | Blueprint cost and lifecycle range ledger with source status and cost-claim labels | You need to separate planning placeholders from source-backed cost claims |
| `data/blueprint-phase-sequence.csv` | Blueprint dependency sequence with package prerequisites, promotion gates, blockers, and next artifacts | You need to see why packages move in Phase 0/1/2 order |
| `data/forum-docket.csv` | Milepost 5 review docket with parliament, stakeholder, editorial, panel, and owner review contracts | You need to know which claims need review before Blueprint |
| `docs/forum/milepost-4-held-claims-parliament.md` | First Forum parliament review of the Milepost 4 held claims and Blueprint lock | You need the initial adversarial review record for entering The Forum |
| `docs/forum/standards-stakeholder-pass.md` | Stakeholder review of the standards package, classifying standards by operational value, source gates, expansion risk, and mitigation role | You need stakeholder constraints before Blueprint feature packaging |
| `docs/forum/milepost-4-closeout-editorial.md` | Editorial scope, citation-traceability, and numeracy gate for the Milepost 4 closeout | You need to know whether the closeout can be used as a reviewed Forum input |
| `docs/forum/standards-package-parliament.md` | Parliament review of stakeholder-classed standards, adding Blueprint intake rules for mitigation, delivery, exposure, and rural-access exceptions | You need adversarial constraints before standards become Blueprint feature packages |
| `docs/forum/no-delta-scenarios-parliament.md` | Parliament review of Donner/Atlanta no-delta executable scenarios | You need to know why bound scenarios with no modeled delta cannot support benefit claims |
| `research/publications/C.1+od-freight-reliability/reviews/MILEPOST5-RECHECK.md` | Milepost 5 panel recheck of C.1 SLA/PTI and reliability-cost claims under absent NPMRDS/FPM validation | You need to know why C.1 can inform Blueprint only as heuristic evidence |
| `docs/INTERSTATE_TYCOON.md` | Game-facing product concept that translates ROUTE simulations into a highway tycoon experience | You need to explain or prototype ROUTE as a playable game |
| `docs/game/interstate-tycoon-plan.md` | Execution plan for the Interstate Tycoon paper, CLI, browser, campaign, and public-demo phases | You need the current game build sequence and gates |
| `docs/game/des-moines-diamond-g0.md` | First playable paper prototype for the Interstate Tycoon T1/T1 tutorial scenario | You need the G0 rules, cards, screen/copy/audio contracts, or session-log shape |
| `docs/game/donner-weather-closure-g0.md` | Second campaign paper prototype for the I-80 mountain-pass weather resilience lesson | You need the G0 rules, cards, evidence labels, or session-log shape for Stop 2 |
| `docs/game/donner-weather-closure-amendments.md` | Forward-only amendment log for Donner Weather Closure rules and copy | You need to see which Stop 2 playtest findings changed the scenario |
| `docs/game/donner-weather-closure-playtest.md` | Score sheet, season log, surprise log, and promotion checklist for Donner Weather Closure playtests | You need to run or review the Stop 2 paper playtest |
| `docs/game/donner-weather-closure-playtest-001.md` | First simulated blind-player playtest log for the Mountain Pass scenario | You need the first Stop 2 playtest evidence record and amendment candidates |
| `docs/game/donner-weather-closure-cli-playtest-001.md` | First CLI playtest for the Mountain Pass scenario after v0.2 amendments | You need to verify the Donner G1-A seed and its publication hold |
| `docs/game/des-moines-diamond-amendments.md` | Forward-only amendment log for Des Moines Diamond rules and copy | You need to see which playtest findings changed the scenario |
| `docs/game/des-moines-diamond-playtest.md` | Score sheet, season log, surprise log, and promotion checklist for Des Moines Diamond playtests | You need to run or review a G0 playtest |
| `docs/game/des-moines-diamond-playthrough.md` | Narrated reference playthrough showing the intended topology aha and publication lock | You need the Tier-B transcript for review or blind-playtest comparison |
| `docs/game/des-moines-diamond-blind-playtest-001.md` | First simulated blind-player playtest log and amendment candidates | You need the first G0 playtest evidence record |
| `docs/game/des-moines-diamond-blind-playtest-002.md` and `003.md` | Follow-on v0.2 simulated playtests for optimizer and evidence-first personas | You need post-amendment playtest evidence |
| `docs/game/des-moines-diamond-playtest-synthesis.md` | Cross-playtest synthesis and promotion readiness decision | You need the current G0-B/G0-C evidence summary |
| `docs/game/des-moines-diamond-panel-g0.md` | TIGRIS/HUNT/ASPECT/PROSE/SCORE/QUEST panel decision for the first Des Moines paper scenario | You need the G0-B pass, G0-C hold, or G1-A implementation requirements |
| `docs/game/des-moines-diamond-cli-playtest-001.md` | First G1-A terminal playtest for `route game scenarios`, `inspect`, and live engine hooks | You need to verify the first game CLI slice against the paper scenario |
| `docs/game/des-moines-diamond-cli-playtest-002.md` | First deterministic `run-season` CLI playtest with state write, log append, and resume | You need to verify season resolution before scoring |
| `docs/game/des-moines-diamond-cli-playtest-003.md` | Active-project countdown playtest for multi-season connector completion | You need to verify construction timing before score bands |
| `docs/game/des-moines-diamond-cli-playtest-004.md` | First score-command playtest for operational score versus publication hold | You need to verify the full G1-A CLI loop |
| `docs/game/des-moines-diamond-g2-plan.md` | Browser-prototype plan for the map-first Des Moines scenario board | You need to start or review G2-A UI work |
| `docs/game/des-moines-diamond-browser-playtest.md` | Human browser blind-playtest packet for the G2-A prototype | You need to run or review the browser comprehension/export/aha gate |
| `docs/game/des-moines-diamond-browser-playtest-001.md` | Simulated browser playtest baseline for the G2-A prototype | You need the reproducible browser baseline before a human run |
| `docs/game/browser/des-moines-diamond.html` | Static first G2-A browser prototype for fixture playback | You need to open or inspect the map-first Des Moines game board |
| `docs/game/browser/check-des-moines-browser.ps1` | Static fixture-contract check for the Des Moines browser prototype | You need to verify browser fixture copy and required regions without a browser harness |
| `docs/game/browser/des-moines-browser.spec.js` | Playwright desktop/mobile smoke check for the Des Moines browser prototype | You need screenshot-era browser regression coverage |
| `docs/game/route-game-cli-design.md` | G1 `route game` command, state, scoring, gate, and test design | You need to implement or review the CLI game layer |
| `data/game/campaign-spine.csv` | Map-backed Interstate Tycoon campaign sequence with lessons, evidence gates, publication gates, and next artifacts | You need to know what game scenario comes after Des Moines or verify `route game campaign --gate` |
| `data/game/t2-service-overlays.csv` | Game-facing T2 service-class overlay contract linking service standards to incident, upgrade, and restitch levers | You need to target T2 service classes in a scenario or verify `route game t2-overlays --gate` |
| `data/game/t2-scenario-hooks.csv` | Campaign-facing T2 hook ledger assigning T1/T2 scenarios to service classes, map ids, player decisions, and evidence holds | You need to see which campaign stops consume T2 overlays or verify `route game t2-hooks --gate` |
| `data/game/des-moines-diamond-session-fixture.csv` | Canonical G1-A score fixture for operational win with publication hold | You need stable score-output regression coverage |
| `data/game/des-moines-diamond-state-fixture.json` | Canonical G2 seed state showing completed connector with publication hold | You need browser/campaign state fixture data |
| `data/game/donner-weather-closure-session-fixture.csv` | Canonical Donner CLI seed fixture showing an operational winter win with SLA and publication still held | You need to verify the Mountain Pass game score path |
| `data/map-atlas.csv` | Map artifact manifest for national, schematic, and T1 regional maps | You need to regenerate or gate maps used by tier presentation and the game |
| `data/tier-promise-standards.csv` | Machine-readable promise-horizon doctrine: T1 is 48/36h national freight, T2 is 24/12h regional freight, T3 is 6h feeder access, and T4 is 1h local access | You need to understand which SLA promise windows are allowed to drive each tier selector |
| `data/t1-sla-candidate-universe.csv` | Candidate universe for national 48h/36h promise pairs with score inputs, drop hints, and coverage links | You need to add or rescore a possible T1 SLA pair before it reaches the selected portfolio |
| `data/t1-sla-candidate-pairs.csv` | Ranked T1 SLA promise-pair cut-line artifact generated by `route t1-sla-candidate-pairs --gate` | You need to answer why the selected top-25 promise portfolio beat pairs 26+ |
| `data/t1-sla-pairs.csv` | Top-25 national T1 SLA promise portfolio used by the T1 line selector | You need to see which 48h/36h freight promises force national T1 route selection |
| `data/t1-line-selector.csv` | T1 selector output balancing national SLA promises, route scores, top city stops, and route/stop budgets | You need to compare algorithmically selected T1 lines against the current Beck T1 map |
| `data/t1-feedback-docket.csv` | Conservative upward-feedback docket showing which T2/T3/T4 pressure rows can reopen T1 review, and which are held below T1 without named SLA/stop/topology dependency | You need to check whether lower-tier pressure actually changes the national promise spine |
| `data/t3-zone-access-obligations.csv` | T3 zone obligation table grouping lower-tier pressure into 6h feeder access and 24h upgrade-review obligations tied to T3 map ids | You need to build T3 zone route columns, explain why a zone map exists, or audit lower-tier pressure before map rendering |
| `data/t3-zone-route-columns.csv` | Route-level T3 zone selector output with selected 6h feeder columns, upward-review connectors, and below-threshold access-gap review rows | You need to see which lower-tier routes are selected for zone maps before stop selection or diagnostics |
| `data/t4-terminal-access-columns.csv` | T4 local access selector output with 1h terminal obligations, terminal-review rows, and explicit zone-assignment-needed gaps | You need to audit local access pressure before promoting routes to T3 or drawing local insets |
| `data/t3-t4-access-gaps.csv` | Access-gap repair ledger collecting below-threshold feeder, terminal-evidence, and zone-assignment gaps with upward pressure blocked | You need to triage unresolved T3/T4 pressure before map diagnostics or terminal/source enrichment |
| `data/t3-zone-map-diagnostics.csv` | Zone-map readiness diagnostics joining selected feeder columns, held access gaps, and T3 map atlas ids | You need to decide which T3 zone maps can render selected feeders and which need held-gap callouts |
| `data/t3-zone-render-board.csv` | Optimizer-backed board contract for T3 zone maps and game overlays, with zone summaries, selected routes, review connectors, held gaps, and unassigned backlog | You need the concrete route/gap rows a T3 renderer may display without falling back to static fixtures |
| `data/t3-zone-stop-placement.csv` | Zone-bounded stop-placement readiness for selected T3 render-board routes, distinguishing ready stop chains from stop-authoring gaps | You need to know which selected T3 routes can drive Beck geometry and which need more named stops first |
| `data/national-segment-registry.csv` | Segment identity registry merging segment ids, bundle ids, stitch groups, aliases, state scopes, layer coverage, and stop-layout readiness | You need a single auditable join surface for route segments before promotion, bundling, geometry, incidents, or game overlays |
| `docs/national-segment-identity-spec.md` | Stable segment identity, alias, state-scope, stitch-group, and route-bundle grammar for optimizer and renderer artifacts | You need to join, promote, rename, or bundle route segments without relying on ambiguous route labels |
| `data/t1-design-review.csv` | T1 design review joining promise-selected routes to Beck overlap diagnostics, cutline candidates, and score-backbone exceptions | You need to decide whether a selected T1 line is accepted, needs map policy, or should be demoted/replaced |
| `data/t1-design-policy-actions.csv` | T1 design policy action contract for accepted lines, shared-segment overlap, score-backbone exceptions, and cutline holds | You need to interpret `next_design_action` in `data/t1-design-review.csv` or verify `route t1-design-policy --gate` |
| `data/t1-score-exceptions.csv` | T1 score-backbone exception decisions for selected routes without current 48h/36h promise pairs | You need to see whether a score-only selected T1 route is justified, demoted, or replaced |
| `docs/t1-design-review.md` | Human-readable guide to the T1 design review roles, accepted routes, policy reviews, and current cutline candidates | You need the current design interpretation without reading the generated CSV |
| `data/beck-t1-diagnostics.csv` | T1 Beck backbone diagnostics for endpoint qualification and shared-segment overlap review | You need to audit the current hand-authored Beck T1 line chains before replacing them with selector output |
| `data/beck-t2-service-standards.csv` | Machine-readable T2 Beck service-class standards for diagnostics, schematic rendering, release gates, and game overlay semantics | You need to interpret `service_class` in `data/beck-t2-diagnostics.csv`, maintain `data/game/t2-service-overlays.csv`, or verify `route beck-t2-service-standards --gate` |
| `data/beck-t2-qualification-actions.csv` | Machine-readable T2 Beck qualification-action rules for duplicate-service review, demotion review, and map treatment | You need to interpret `service_action` in `data/beck-t2-diagnostics.csv` or verify `route beck-t2-qualification-actions --gate` |
| `data/standards-l1-inventory.csv` | L1 inventory/source ledger for standards blocked on asset or operations data | You need to know what source table must exist before a Planned standard can become testable |
| `data/pressure-test-scenarios.csv` | L2 scenario catalog and readiness/blocker labels | You need to know which adversity scenarios are real pressure tests versus named shells |
| `data/t1-intersection-failures.csv` | T1/T1 failure-rate, duration, throughput-retention, and reroute evidence ledger | You need to know whether T1/T1 resilience claims have empirical incident anchors |
| `data/t1-diamond-validation.csv` | T1/T1 diamond anchor manual-validation ledger | You need to know which curated T1/T1 anchors are recognized, manually validated, or still heuristic |
| `data/t1-failure-source-plan.csv` | Source acquisition plan for T1/T1 failure-rate and reroute fields | You need to know which DOT/FHWA data systems can fill failure evidence gaps |
| `data/t1-source-health.csv` | Source health/status ledger for T1/T1 evidence ingestion | You need to know whether a source is live, blocked, key-gated, or archive-only |
| `data/t1-snapshot-plan.csv` | Polling and accumulation plan for live T1/T1 snapshot feeds | You need to run or review the current-state feed accumulation cadence |
| `data/t1-failure-events.csv` | Normalized raw T1/T1 incident, closure, and work-zone observations | You need to compute empirical annual failure rates and duration percentiles |
| `specs/2026-05-06-route-design.md` | Core ROUTE method: corpus, dimensions, gap map, parliament, design proposals | You need the conceptual process |
| `specs/2026-05-06-route-rust-architecture.md` | Rust workspace architecture, CLI contracts, data model, output formats | You need implementation boundaries |
| `specs/2026-05-06-interstate-2-design.md` | Interstate 2.0 feature set, investment thesis, simulation toolkit, transit integration | You need the national design framework |
| `specs/2026-05-06-tier-standards.md` | T1/T2/T3/T4 tier standards and service expectations | You need tier definitions |
| `research/MODULE.md` | Tracks A-F, paper chain, quantification contracts, review history | You need the research program |
| `.roles/ROLE.md` | Parliament, stakeholder, editorial, and panel-review role index | You need review gates or role selection |
| `TRACKER.md` | Current status board | You need the live project state |

---

## Ownership Rules

1. If a claim describes what ROUTE is trying to do, update `specs/2026-05-06-route-design.md`.
2. If a claim describes how the Rust system works, update `specs/2026-05-06-route-rust-architecture.md`.
3. If a claim describes Interstate 2.0 as a build program, update `specs/2026-05-06-interstate-2-design.md`.
4. If a claim changes the roadmap, phase theme, or done criteria, update `docs/SYSTEM_PLAN.md` and `TRACKER.md`.
5. If a claim appears in a paper, make sure `research/MODULE.md` still names the dependency and quantification contract.
6. If a claim requires judgment, attach the relevant `.roles` pass before calling it validated.

---

## Claim Status Vocabulary

Use the same status words across specs, README, CLI help, and papers:

| Status | Meaning |
|---|---|
| Implemented | Code runs end-to-end and is testable |
| Heuristic | Code runs but uses a proxy, partial data, or simplified model |
| Stub | Interface exists but the analysis is not real yet |
| Planned | Specified but not implemented |
| Deprecated | Historical and no longer a current claim |

When in doubt, mark a claim lower. ROUTE is stronger when it is honest about what is measured versus what is proposed.
