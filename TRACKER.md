# TRACKER — ROUTE

Status board for the ROUTE project. Updated as work happens.

---

## Milepost Status

ROUTE phases use the Milepost theme from `docs/SYSTEM_PLAN.md`.

| Milepost | Description | Status |
|---|---|---|
| 0 — Ground Survey | Repo, specs, roles, data inventory, and CLI scaffold | ✅ substantially complete |
| 1 — Instrument | 16-dimension scorer, calibration ledger, tests, truth labels | ✅ complete |
| 2 — Atlas | Reproducible existing-corridor corpus and tier map | 🔄 partial; refresh needed |
| 3 — Fault Lines | Missing-link, bottleneck, resilience, port, and coverage gaps | 🔄 partial; artifact checks needed |
| 4 — Pressure Test | Flow, incident, relay, SLA, and investment simulations | 🔄 partial; test coverage needed |
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
