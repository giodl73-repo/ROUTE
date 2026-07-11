# TRACKER — ROUTE

Status board for the ROUTE project. Updated as work happens.

---

## Milepost Status

ROUTE phases use the Milepost theme from `docs/SYSTEM_PLAN.md`.

| Milepost | Description | Status |
|---|---|---|
| 0 — Ground Survey | Repo, specs, roles, data inventory, and CLI scaffold | ✅ substantially complete |
| 1 — Instrument | 16-dimension scorer, calibration ledger, tests, truth labels | ✅ complete |
| 2 — Atlas | Reproducible existing-corridor corpus and tier map | ✅ complete |
| 3 — Fault Lines | Missing-link, bottleneck, resilience, port, and coverage gaps | ✅ complete |
| 4 — Pressure Test | Standards proof under flow, incident, relay, SLA, and investment simulations | ✅ complete; pressure gates pass, Blueprint/publication claims held |
| 5 — The Forum | Parliament, stakeholder, editorial, and panel review records | ✅ complete; review docket gates pass, owner playtests held |
| 6 — Blueprint | Interstate 2.0 feature packages and investment sequence | ✅ complete; package, evidence, cost, phase, and spec gates pass |
| 7 — Program | CI, release process, public corpus, maps, and papers | ✅ complete; release gate script, manifest, checklist, and CI workflow pass locally |
| 8 — Evidence Campaign | Work one release-visible hold through source acquisition, validation, review, and claim update | ✅ complete; T1/T1 failure evidence hold improved, not promoted |

---

## Current Execution Focus — I-80 Flagship Stabilization

Internal milepost completion means the relevant command and artifact gates
passed; it does not mean the project has a validated flagship corridor or an
externally reviewed investment recommendation.

| Task | Status | Evidence / Next Step |
|---|---|---|
| Freeze expansion outside the flagship | ✅ active | `GOAL.md`; no new geography, doctrine family, or placeholder-only ledger |
| Record the I-80 anchor baseline | ✅ done | `docs/anchors/i80-flagship-baseline.md` |
| Complete the I-80 corpus narrative and source audit | ⏳ planned | Remove annotation placeholders and reconcile every material number |
| Produce an I-80-specific gap diagnosis | ⏳ planned | Separate measured failures, source gaps, model limits, and geometry artifacts |
| Select one bounded treatment from evidence | ⏳ planned | No treatment is preselected in the baseline pulse |
| Run Parliament and editorial review | ⏳ planned | Seven voices plus citation, numeracy, and scope gates |
| Build the compact decision packet | ⏳ planned | Regenerable report, maps, evidence appendix, and ten-minute presentation path |
| Prepare external review | ⏳ planned | DOT/MPO, freight, and transportation-research reviewers |
| Harden the flagship software path | ⏳ planned | Dependency pinning, CI portability, and focused CLI decomposition |

Active wave:
`waves/2026-07-11-i80-flagship-stabilization/WAVE.md`.

---

## Current Sprint — Milepost 1 Instrument

Goal: make the 16-dimension scorer boringly reliable enough that Atlas work can depend on it.

| Task | Status | Evidence / Next Step |
|---|---|---|
| Calibration ledger emits score, tier, confidence, score-confidence, and review flags | ✅ done | `data/confidence-risks.csv`, `route calibrate` |
| Corridor ledger names the weak dimensions driving each low-confidence ranking | ✅ done | `risk_dimensions` column in `data/confidence-risks.csv` |
| Dimension risk summary separates broad confidence debt from tier-sensitive review risk | ✅ done | `data/confidence-risk-summary.csv` |
| FPM PTI/TTI can flow into graph edges for observed A3 scoring | ✅ done | `route build --fpm`, `build_graph_with_fpm` test |
| Dimension registry table exists in docs and is checked against code | ✅ done | `docs/DIMENSIONS.md`; `dimension_registry_doc_mentions_every_code_and_name` |
| L0/L1 tests cover missing-data behavior and anchor extremes for all 16 dimensions | ✅ done | `sparse_corridor_scores_all_dimensions_with_truth_labels`; `dimension_anchor_extremes_score_zero_and_ten` |
| Proxy and missing-data labels are consistent across score table, corpus report, and CSVs | ✅ done | Shared `confidence_label`; corpus and CSV outputs include labels |
| `route score-all` refreshed under current rubric and confidence columns | ✅ done | `data/scores-all.csv` regenerated with v1.4 confidence labels |
| Stale docs/handoff warnings about old A3/score-all outputs are reconciled | ✅ done | Historical handoff marked superseded; architecture/research docs updated |

Milestone 1 is done when every task above is ✅ and `cargo test --workspace` plus `route calibrate` pass from a clean worktree.

---

## Current Sprint — Milepost 2 Atlas

Goal: make the existing network corpus and tier map reproducible from commands.

| Task | Status | Evidence / Next Step |
|---|---|---|
| Current v1.4 score ledger exists for all atlas candidates | ✅ done | `data/scores-all.csv`, 386 corridors |
| Tier table can be regenerated from `route score-all` | ✅ done | `data/tier-table.csv`, `data/tier-table.md`; written by `route score-all` |
| Historical v1.2 candidate docs are clearly labeled as non-current | ✅ done | `data/t1-candidates.md`, `data/t2-candidates.md`, `data/t3-candidates.md` historical banners |
| Corpus report entries record command, rubric version, data version, confidence, and estimation flags | ✅ done | `route report I80` writes provenance and matches `data/scores-all.csv` score/confidence |
| Tier map can be regenerated from current score ledger | ✅ done | `route map all` reads `data/scores-all.csv` and regenerates `maps/all-tiers.png` |
| Core map atlas has a manifest and gate before game reuse | ✅ done | `data/map-atlas.csv` tracks the national tier map, Beck schematic, and T1 regional maps; `route map-atlas --gate` checks PNG existence, dimensions, and minimum size |
| Basemap claim is either implemented or downgraded explicitly | ✅ done | Spec now labels current map as projected network + state/city labels; polygon basemap deferred |

Milestone 2 is done when `route score-all`, `route calibrate`, and the tier map command regenerate tracked Atlas artifacts from a clean worktree.

---

## Current Sprint — Milepost 3 Fault Lines

Goal: separate true network gaps from source and geometry artifacts.

| Task | Status | Evidence / Next Step |
|---|---|---|
| Corridor geometry QA catches impossible termini, empty states, and carriageway-inflated mileage | ✅ done | I-80 report now has west longitudes, inferred states, and TIGER interstate centerline miles |
| Coverage gaps distinguish large-county centroid artifacts from true access gaps | ✅ done | `data/coverage-gaps.csv` includes gap class and artifact reason columns |
| `route gap --type ...` writes reproducible gap artifacts instead of planned-only output | ✅ done | `gaps/missing-link.md`, `gaps/bottleneck.md`, `gaps/resilience.md`, `gaps/intermodal.md` |
| Bottleneck findings separate congestion, capacity, and graph topology failures | ✅ done | `gaps/bottleneck.md` labels corridor stress, topology chokepoints, and flow-needed capacity seeds |
| Resilience and port-connector gap claims have source/confidence labels | ✅ done | `gaps/resilience.md` and `gaps/intermodal.md` include confidence labels |

Milestone 3 is done when each gap artifact names whether it is a true system gap, a data gap, or a geometry/source artifact, and `cargo test --workspace` passes.

---

## Current Sprint — Milepost 4 Pressure Test

Goal: make every Interstate 2.0 standard earn its place by proving which SLA, throughput, resilience, or access outcome it protects under adversity.

The central blocker is T1/T1 interchange resilience. The current system concentrates two national primary arteries into a single interchange node; Interstate 2.0 must prove that diamond zones, express freight flyovers, and alternate routing keep T1 freight moving when that node is stressed or partially closed.

| Task | Status | Evidence / Next Step |
|---|---|---|
| Standards proof ledger maps every T1/T2/T3/T4 standard to outcome, mechanism, stressor, acceptance gate, evidence source, and confidence level | ✅ done | `data/standards-proof-ledger.csv`, `docs/STANDARDS_EVALUATION.md` |
| T1/T1 diamond proof has explicit acceptance gates | ✅ done | Gate defined in `docs/STANDARDS_EVALUATION.md`: k >= 3 in the 50-mile zone, single connector/interchange failure does not collapse transfers, and 80% T1 throughput restoration within 4 hours is demonstrated or labeled unproven |
| T1 SLA model distinguishes freight-lane PTI, GP PTI, relay buffers, incident buffers, and shipper planning windows | ✅ done | `route_sim::sla_proof_table` emits shared heuristic proof rows for GP vs managed lanes, solo vs relay, p95/PTI, 48h share, and evidence label; direct NPMRDS/source validation is explicitly held for Blueprint/publication |
| Throughput proof separates congestion-binding bottlenecks from resilience-binding chokepoints | ✅ done | `data/throughput-proof-matrix.csv` and `route throughput-proof --gate` distinguish congestion-binding bottlenecks from resilience-binding chokepoints; calibrated demand and empirical sensitivity are labeled as next evidence, not Milepost 4 blockers |
| Adversity scenario library covers T1/T1 closure, corridor segment closure, port surge, weather/flood disruption, relay hub outage, EV/rest-area outage, and managed-lane sensitivity | ✅ done | `route pressure-scenarios --gate-l2` verifies required adversity-class coverage and `--gate-readiness` now passes; `route hub-outage` and `route ev-rest-outage` make the former Planned outage rows executable heuristic pressure tests |
| T1/T1 failure-rate and reroute evidence ledger separates modeled assumptions from empirical closure data | ✅ done | `route t1-failures --gate-evidence` enforces empirical/modeled/source_needed labels, confidence labels, artifacts, blockers, and next evidence steps; Des Moines has a low-confidence Iowa 511 snapshot-derived empirical event sample while annual-history/reroute validation remains a labeled publication blocker; `route t1-diamond-validation --gate-catalog` tracks all 15 curated anchors and `--priority A --docket --with-access` turns top-site blockers into source-linked tasks |
| Source acquisition plan identifies DOT/FHWA systems needed to fill T1/T1 failure evidence fields | ✅ done | `data/t1-failure-source-plan.csv`; official source targets identified for all 15 T1/T1 sites plus FHWA/NPMRDS cross-cutting sources |
| Source health ledger separates identified URLs from usable ingestion paths | ✅ done | `data/t1-source-health.csv`, `route t1-source-health --blockers`, and `route t1-access-docket` show live, blocked, key-gated, account-gated, access-gated, and archive-needed sources |
| Planned standards have L1 inventory/source rows before they feed Blueprint | ✅ done | `data/standards-l1-inventory.csv` and `route standards-inventory --gate --gate-planned` cover WIM, rest/truck parking, flyovers, spurs, C-D roads, bridges, T2 alternates, T3 operations, and T4 maintenance |
| T1 bridge standard has an L1 condition-coverage gate | ✅ done | `route standards-bridges --tier T1 --gate-l1` checks cached NBI bridge-condition coverage for generated T1 routes while keeping clearance/load-posting joins as unresolved source gaps |
| Normalized event observation table computes annual T1/T1 failure rates and duration percentiles | ✅ done | `data/t1-failure-events.csv` carries a small Iowa 511 normalized observation sample, passes `route t1-failure-events --gate-observations`, and can write the Des Moines summary into `data/t1-intersection-failures.csv`; Iowa 511/INDOT snapshot polling is planned and gated by `route t1-snapshot-plan --gate-plan`, and `--script` prints runnable fetch/import/accumulate commands; annual history remains a labeled publication blocker |
| Interstate Tycoon turns pressure-test proof into a playable standard-by-standard demo | ✅ done | Des Moines G1-A/G1-B CLI loop is stable and first G2-A browser prototype has fixture checks, Playwright checks, browser-local season mutation, CLI-compatible session-log display/download, after-action scoring, a browser blind-playtest packet, and a simulated browser pass; Donner now has a G1-A seed in `route game` with trapped-queue/source-observed checks and a score fixture; G0-C remains explicitly held for a human blind playtest or owner acceptance |
| L0 tests cover primitive invariants for max-flow, incident degradation, SLA arithmetic, relay timing, and k-connectivity | ✅ done | Incident degradation/restoration, max-flow bottleneck/parallel-path behavior, relay spacing/driver-mode behavior, SLA arithmetic, and k-connectivity edge-disjoint path behavior are covered |
| L1 tests verify generated pressure-test artifacts are reproducible from stable fixtures | ✅ done | Unit coverage gates canonical `data/pressure-test-scenarios.csv`, `data/throughput-proof-matrix.csv`, `data/t1-failure-events.csv`, `data/standards-proof-ledger.csv`, and Donner/Des Moines game fixtures; embedded scenario TOMLs parse and expose readiness warnings |
| L2 tests run representative scenarios and assert bounded outputs rather than headline-only claims | ✅ done | Synthetic T1 closure, NY-LA SLA, Houston-Chicago/I-69, Miami-NYC port-corridor, relay outage, EV/rest outage, and game-score fixtures bound representative outputs; calibrated demand/empirical sensitivity remains labeled next evidence |
| Unproven standards are labeled before Blueprint work consumes them | ✅ done | `route standards-proof --gate-blueprint` now rejects unknown evidence labels and unresolved non-Implemented standards; allowed labels are Implemented, Heuristic, Stub, Planned, or Deprecated |

Milestone 4 is done when every active standard has a proof record, T1/T1 interchange resilience has passed or been explicitly downgraded, and `cargo test --workspace` protects the L0/L1/L2 pressure-test path.

### Milepost 4 Finish Tasklist

Goal: close Pressure Test by turning every remaining partial item into either a passing gate or an explicitly downgraded, evidence-labeled claim.

| Order | Task | Status | Exit Gate / Artifact |
|---:|---|---|---|
| 1 | Run an owner/human review of the Donner CLI slice | ⚠️ held | No human/owner acceptance record attached; `docs/game/donner-weather-closure-cli-playtest-001.md` remains simulated/CLI evidence |
| 2 | Decide Donner next path: browser G2-A prototype versus deeper sim calibration first | ✅ done | Decision: calibrate `donner-closure` sim and alternate-capacity evidence before proof-grade browser promotion; logged in `docs/milepost-4-closeout.md` and campaign next artifact |
| 3 | Close Interstate Tycoon G0-C acceptance for Des Moines and Donner, or mark both held explicitly | ✅ done | Both scenario statuses now mark G0-C held in `data/game/campaign-spine.csv` / `docs/game/interstate-tycoon-plan.md` |
| 4 | Add L1/L2 fixture coverage for Donner game CLI scoring if the owner accepts the G1-A seed | ✅ done | `data/game/donner-weather-closure-session-fixture.csv` and `game::tests` protect trapped queue, source-observed copy, and publication hold |
| 5 | Reconcile Donner sim caveat with pressure-test claims | ✅ done | `docs/STANDARDS_EVALUATION.md`, `data/pressure-test-scenarios.csv`, CLI score output, and closeout note state the current synthetic no-delta limitation |
| 6 | Work one A-band T1/T1 evidence docket item far enough to prove the source-acquisition loop | ✅ done | Iowa 511/INDOT snapshot plan scripts pass; Iowa 511 normalized observations pass event and failure evidence gates |
| 7 | Tighten T1 SLA proof labels so every row says direct evidence, heuristic, source-needed, or downgraded | ✅ done | `route standards-proof --gate-pressure`, `route pressure-scenarios --coverage --gate-coverage`, and `route throughput-proof --gate` pass; Blueprint remains held |
| 8 | Run the full Milepost 4 gate bundle from a clean worktree | ✅ done | `cargo test --workspace`, `route standards-proof --gate-pressure`, `route pressure-scenarios --gate-l2 --gate-readiness`, `route pressure-scenarios --coverage --gate-coverage`, `route throughput-proof --gate`, `route game campaign --gate`, `route t1-failures --gate-evidence`, `route t1-failure-events --gate-observations`, and `route t1-snapshot-plan --gate-plan --script --priority A` pass; `route standards-proof --gate-blueprint` remains an expected Blueprint hold |
| 9 | Update Milepost 4 task statuses and handoff notes | ✅ done | `docs/milepost-4-closeout.md` added; remaining downgraded/held claims are named there |

Milepost 4 closeout points Milepost 5 toward the held claims rather than direct Blueprint promotion.

---

## Current Sprint — Milepost 5 The Forum

Goal: make disagreement productive and traceable before Blueprint work consumes the Pressure Test results.

The central rule for this stage is that review must change an artifact, a claim label, a docket status, or a next evidence step. A review that merely praises or criticizes without updating the work product is theater, not Forum evidence.

| Task | Status | Evidence / Next Step |
|---|---|---|
| Forum docket exists and gates complete review contracts | 🔄 partial | `data/forum-docket.csv`; `route forum --gate` |
| First parliament review attached to Milepost 4 held claims | ✅ done | `docs/forum/milepost-4-held-claims-parliament.md`; F5-01 complete |
| Des Moines and Donner G0-C owner/human acceptance remain explicitly held | ✅ done | F5-02/F5-03 in `data/forum-docket.csv`; campaign statuses mark G0-C held |
| Stakeholder pass reviews standards package before Blueprint feature packaging | ✅ done | F5-04 complete: `docs/forum/standards-stakeholder-pass.md`; Blueprint must classify standards by stakeholder class |
| Editorial gate checks Milepost 4 closeout scope, citations, and numeracy before validated status | ✅ done | F5-05 complete: `docs/forum/milepost-4-closeout-editorial.md`; closeout passes for Forum use |
| Panel recheck protects C.1 SLA/PTI claims before any Blueprint use | ✅ done | F5-06 complete: `research/publications/C.1+od-freight-reliability/reviews/MILEPOST5-RECHECK.md`; C.1 remains usable as heuristic research but SLA/PTI and reliability-dollar claims are held for Blueprint |
| At least three parliament reviews cover high-stakes proposals or held claims | ✅ done | 3/3 complete: F5-01 Milepost 4 held claims, F5-07 standards package, F5-08 no-delta scenarios |
| Forum outcomes feed tracker/spec changes | ✅ done | F5-01 keeps Blueprint gate locked and points Donner back to sim calibration; F5-04/F5-07 require stakeholder-classed Blueprint feature packages with mitigation/delivery fields; F5-06 keeps SLA/PTI and reliability-dollar claims heuristic for Blueprint; F5-08 bars no-delta scenarios from supporting benefit claims |

Milepost 5 is done when at least three adversarial review records exist, stakeholder/editorial/panel gates are attached to the active high-stakes claims, and `route forum --gate` plus the relevant pressure gates pass.

Status: ✅ complete. `docs/milepost-5-closeout.md` records the Forum decision, Blueprint intake rules, and explicit owner/human playtest holds.

---

## Current Sprint — Milepost 6 Blueprint

Goal: turn Forum-reviewed pressure-test output into feature packages, investment sequencing, and source-labeled design claims without laundering heuristic evidence into proof.

The central rule for this stage is that every package must carry its stakeholder class, evidence level, Forum constraint, delivery status, mitigation/exposure fields where needed, and next evidence step before it can enter the Interstate 2.0 design spec.

| Task | Status | Evidence / Next Step |
|---|---|---|
| Blueprint plan names the slices needed to finish Milepost 6 | ✅ done | `docs/blueprint/milepost-6-plan.md` |
| Feature package ledger exists and gates Forum intake rules | ✅ done | `data/blueprint-feature-packages.csv`; `route blueprint --gate --details` |
| Package taxonomy and briefs separate operational must-haves, source-gated must-haves, conditional expansion, and mitigation companions | ✅ done | `docs/blueprint/feature-packages.md` |
| Evidence downgrade map reconciles package claims with `standards-proof --gate-blueprint` | ✅ done | `data/blueprint-evidence-map.csv`; `route blueprint-evidence --gate --details` |
| Cost and lifecycle range ledger separates sourced costs from planning placeholders | ✅ done | `data/blueprint-cost-ranges.csv`; `route blueprint-costs --gate --details` |
| Phase sequence names prerequisites, blockers, and promotion gates | ✅ done | `data/blueprint-phase-sequence.csv`; `docs/blueprint/phase-sequence.md` |
| Interstate 2.0 design spec is amended to use package statuses instead of unsupported benefits | ✅ done | `specs/2026-05-06-interstate-2-design.md` §1A now inherits package/evidence/cost labels |
| Milepost 6 closeout runs the gate bundle | ✅ done | `docs/milepost-6-closeout.md`; `cargo test --workspace`, `route blueprint --gate`, `route blueprint-evidence --gate`, `route blueprint-costs --gate`, Forum, and pressure gates pass |

Milepost 6 is done when the package ledger, package briefs, evidence downgrade map, cost ledger, phase sequence, and design-spec amendment all agree on what is proven, heuristic, planned, held, or downgraded.

Status: ✅ complete. `docs/milepost-6-closeout.md` records the Blueprint package spine, gate bundle, remaining held claims, and Milepost 7 handoff.

---

## Current Sprint — Milepost 7 Program

Goal: make the Milepost 4-6 corpus reproducible enough that another person can clone the repo, run the gates, and understand which claims are public, held, internal, or source-needed.

The central rule for this stage is that release process must protect evidence labels. A publishable artifact can carry held claims, but only when the hold is visible and verified by the release manifest or gate bundle.

| Task | Status | Evidence / Next Step |
|---|---|---|
| Milepost 7 plan and tasklist exist | ✅ done | `docs/milepost-7-plan.md` |
| Scripted gate bundle covers Mileposts 4-6 and release hygiene | ✅ done | `scripts/check-mileposts.ps1` |
| Release manifest names artifact ownership, public status, and verification commands | ✅ done | `data/release-manifest.csv` |
| Release checklist defines public, held, internal, and source-needed policy | ✅ done | `docs/release/release-checklist.md` |
| CI-ready workflow runs the local gate script | ✅ done | `.github/workflows/ci.yml` |
| Local release gate bundle passes | ✅ done | `powershell -ExecutionPolicy Bypass -File scripts/check-mileposts.ps1` |
| Milepost 7 closeout records release decision and remaining holds | ✅ done | `docs/milepost-7-closeout.md` |

Milepost 7 is done when the release manifest, gate script, checklist, CI workflow, tracker, and closeout agree on the publishable release surface and the local release gate bundle passes.

Status: ✅ complete. `docs/milepost-7-closeout.md` records the Program release decision, passing gate bundle, release policy, and remaining held claims.

---

## Current Sprint — Milepost 8 Evidence Campaign

Goal: pick one visible hold from the Milepost 7 release surface and work it through source acquisition, normalization, validation, review, and claim update.

The central rule for this stage is that source acquisition is not proof by itself. A claim can only move after the evidence is normalized, bounded, reviewed, and propagated through every ledger that references it.

Recommended first target: T1/T1 failure evidence, because the repo already has a source plan, normalized event schema, one empirical seed site, and direct Blueprint dependency through the T1/T1 diamond recovery hold.

| Task | Status | Evidence / Next Step |
|---|---|---|
| Milepost 8 plan and checklist exist | ✅ done | `docs/milepost-8-plan.md` |
| Select one target hold and write decision rationale | ✅ done | `docs/evidence-campaigns/milepost-8-target.md`; selected T1/T1 failure evidence |
| Create source acquisition checklist for the target | ✅ done | `data/evidence-campaign-source-plan.csv` |
| Run or document source access attempt | ✅ done | `docs/evidence-campaigns/milepost-8-source-attempt.md`; Iowa fetch/import produced 25 rows, INDOT fetch/import produced 0 observation-grade rows |
| Normalize observations into an evidence ledger | ✅ done | `data/t1-failure-events.csv`; observation gate passes for Iowa rows |
| Update pressure and Blueprint claim references | ✅ done | `data/t1-intersection-failures.csv`, `data/blueprint-evidence-map.csv`, and `data/release-manifest.csv` point to the Milepost 8 source attempt |
| Attach review record | ✅ done | `docs/reviews/milepost-8-t1-failure-evidence-review.md`; decision is continued hold with better traceability |
| Run release gate bundle | ✅ done | `powershell -ExecutionPolicy Bypass -File scripts/check-mileposts.ps1` |
| Write Milepost 8 closeout | ✅ done | `docs/milepost-8-closeout.md` |

Milepost 8 is done when the selected hold has a source/evidence record, claim-status update, review decision, passing release gate bundle, and closeout.

Status: ✅ complete. `docs/milepost-8-closeout.md` records the T1/T1 failure evidence campaign, improved hold, source attempt results, review decision, and next evidence step.

---

## Current Sprint — Milepost 9 Evidence Operations

Goal: turn the Milepost 8 T1/T1 improved hold into a repeatable evidence operation with source-window metadata, archive or repeated-polling paths, and a guard that prevents snapshot-only rows from being treated as annual history.

The central rule for this stage is that repeatability matters more than volume. More live snapshots can strengthen a monitoring record, but they do not promote a recovery claim unless the capture windows, durations, and review decision support the specific claim.

Recommended first target: continue T1/T1 failure evidence for Iowa `T1X-I35-I80` and INDOT/OHGO `T1X-I80-I90`, because Milepost 8 already proved source access paths and blockers for those two sites.

| Task | Status | Evidence / Next Step |
|---|---|---|
| Milepost 9 plan and checklist exist | ✅ done | `docs/milepost-9-plan.md` |
| Add source-window fields or companion ledger for repeated T1/T1 evidence | ✅ done | `data/t1-evidence-windows.csv` and `route t1-evidence-windows --gate-windows` expose source, capture window, freshness, and snapshot-vs-history status |
| Implement or document Iowa 511 repeat-window path | ✅ done | `scripts/poll-t1-iowa511.ps1` and `docs/evidence-campaigns/milepost-9-iowa-repeat-window.md` define the repeated polling path for `T1X-I35-I80` |
| Implement or document INDOT/OHGO enrichment path | ✅ done | `docs/evidence-campaigns/milepost-9-indot-ohgo-enrichment.md` keeps `T1X-I80-I90` as `enrichment_blocker` until timed rows or archive history exist |
| Add a snapshot-history guard | ✅ done | `route t1-evidence-windows --gate-windows` and `docs/evidence-campaigns/milepost-9-snapshot-history-guard.md` fail promotion when only snapshot-only evidence exists |
| Update T1/T1 failure and Blueprint evidence references | ✅ done | `data/t1-intersection-failures.csv` and `data/blueprint-evidence-map.csv` now point to the evidence-window guard and preserve the hold |
| Attach Milepost 9 evidence-operations review | ✅ done | `docs/reviews/milepost-9-evidence-operations-review.md` continues the hold |
| Run release gate bundle and write closeout | ✅ done | `powershell -ExecutionPolicy Bypass -File scripts/check-mileposts.ps1`; `docs/milepost-9-closeout.md` |

Milepost 9 is done when the selected T1/T1 evidence operation has a repeatable acquisition path or explicit blocker, evidence-window metadata, a snapshot-history guard, updated claim references, review decision, passing release gate bundle, and closeout.

Status: ✅ complete. `docs/milepost-9-closeout.md` records the evidence-window guard, Iowa repeat path, INDOT/OHGO enrichment blocker, review decision, and continued hold.

---

## Corpus

### Existing corridors scored

| Designation | Name | Miles | Rubric ver | Status | Total score | Notes |
|---|---|---|---|---|---|---|
| I-80 | San Francisco Bay Area to New York region | 2,917 | v1.4 | draft | 89.8 / 160 | Medium confidence; generated record still contains human-annotation placeholders |

### Proposed corridors scored

| Slug | Termini | Rubric ver | Status | Total score | Notes |
|---|---|---|---|---|---|---|
| — | — | — | — | — | — |

---

## Parliament reviews

| Corridor | Round | Date | Earned | Refuted | Collisions | Axes changed | Notes |
|---|---|---|---|---|---|---|---|
| — | — | — | — | — | — | — | — |

---

## Gap analyses

| Slug | Gap type | Date | Status | Key finding |
|---|---|---|---|---|
| — | — | — | — | — |

---

## Design proposals

| Slug | Corridor | Date | Status | Key feature |
|---|---|---|---|---|
| — | — | — | — | — |

---

## Rubric changelog

| Version | Date | Change |
|---|---|---|
| v1.0 | 2026-05-06 | Initial 12-dimension pool |
| v1.4 | 2026-05-08 | Current 16-dimension scorer: adds trade, safety, military/strategic, agricultural export access |

---

## Session handoffs

| Date | Slug | Priorities carried forward |
|---|---|---|
| — | — | — |

---

## Pre-publication checklist

| Item | Status |
|---|---|
| LICENSE | ✅ |
| README `## License` | ✅ |
| Internal naming scrubbed | ✅ |
| `.gitignore` standard block | ✅ |
| Anchor corridor complete | ⏳ |
| ≥1 research paper | ⏳ |
| Research PDFs built | ⏳ |
