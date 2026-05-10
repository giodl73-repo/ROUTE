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
| 4 — Pressure Test | Standards proof under flow, incident, relay, SLA, and investment simulations | 🔄 partial; proof ledger and scenario gates needed |
| 5 — The Forum | Parliament, stakeholder, editorial, and panel review records | 🔄 partial; formal gates needed |
| 6 — Blueprint | Interstate 2.0 feature packages and investment sequence | 🔄 partial; claims need evidence labels |
| 7 — Program | CI, release process, public corpus, maps, and papers | ⏳ not started |

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
| T1 SLA model distinguishes freight-lane PTI, GP PTI, relay buffers, incident buffers, and shipper planning windows | 🔄 partial | `route_sim::sla_proof_table` now emits shared heuristic proof rows for GP vs managed lanes, solo vs relay, p95/PTI, 48h share, and evidence label; direct NPMRDS/source validation remains |
| Throughput proof separates congestion-binding bottlenecks from resilience-binding chokepoints | 🔄 partial | `data/throughput-proof-matrix.csv` and `route throughput-proof --gate` now distinguish congestion-binding bottlenecks from resilience-binding chokepoints; remaining work is calibrated demand and empirical sensitivity |
| Adversity scenario library covers T1/T1 closure, corridor segment closure, port surge, weather/flood disruption, relay hub outage, EV/rest-area outage, and managed-lane sensitivity | ✅ done | `route pressure-scenarios --gate-l2` verifies required adversity-class coverage and `--gate-readiness` now passes; `route hub-outage` and `route ev-rest-outage` make the former Planned outage rows executable heuristic pressure tests |
| T1/T1 failure-rate and reroute evidence ledger separates modeled assumptions from empirical closure data | 🔄 partial | `route t1-failures --gate-evidence` now enforces empirical/modeled/source_needed labels, confidence labels, artifacts, blockers, and next evidence steps; most sites still need empirical sources |
| Source acquisition plan identifies DOT/FHWA systems needed to fill T1/T1 failure evidence fields | ✅ done | `data/t1-failure-source-plan.csv`; official source targets identified for all 15 T1/T1 sites plus FHWA/NPMRDS cross-cutting sources |
| Source health ledger separates identified URLs from usable ingestion paths | ✅ done | `data/t1-source-health.csv`, `route t1-source-health --blockers`, and `route t1-access-docket` show live, blocked, key-gated, account-gated, access-gated, and archive-needed sources |
| Normalized event observation table computes annual T1/T1 failure rates and duration percentiles | 🔄 partial | `data/t1-failure-events.csv` now carries a small Iowa 511 normalized observation sample and passes `route t1-failure-events --gate-observations`; Iowa 511/MDOT/INDOT ingestion, TDOT SmartWay importer scaffolding, `route t1-accumulate-events`, and `--write-ledger` are ready; annual history still needs accumulation |
| L0 tests cover primitive invariants for max-flow, incident degradation, SLA arithmetic, relay timing, and k-connectivity | ✅ done | Incident degradation/restoration, max-flow bottleneck/parallel-path behavior, relay spacing/driver-mode behavior, SLA arithmetic, and k-connectivity edge-disjoint path behavior are covered |
| L1 tests verify generated pressure-test artifacts are reproducible from stable fixtures | 🔄 partial | Unit coverage now gates canonical `data/pressure-test-scenarios.csv`, `data/throughput-proof-matrix.csv`, and `data/t1-failure-events.csv`; `route standards-proof` parses `data/standards-proof-ledger.csv`; embedded scenario TOMLs parse and expose readiness warnings |
| L2 tests run representative scenarios and assert bounded outputs rather than headline-only claims | 🔄 partial | Synthetic T1 closure, NY-LA SLA, Houston-Chicago/I-69, and Miami-NYC port-corridor fixtures now bound representative outputs; remaining work is calibrated demand/empirical sensitivity |
| Unproven standards are labeled before Blueprint work consumes them | ✅ done | `route standards-proof --gate-blueprint` now rejects unknown evidence labels and unresolved non-Implemented standards; allowed labels are Implemented, Heuristic, Stub, Planned, or Deprecated |

Milestone 4 is done when every active standard has a proof record, T1/T1 interchange resilience has passed or been explicitly downgraded, and `cargo test --workspace` protects the L0/L1/L2 pressure-test path.

---

## Corpus

### Existing corridors scored

| Designation | Name | Miles | Rubric ver | Status | Total score | Notes |
|---|---|---|---|---|---|---|
| — | — | — | — | — | — | — |

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
