---
date: 2026-05-07
slug: post-b1-review
rubric_version: v1.1
---

# ROUTE Session Handoff — Post B.1 Review

> Superseded status note, 2026-05-09: this handoff is retained as session history.
> `data/scores-all.csv` has since been regenerated under rubric v1.4 for 386
> interstate and US-highway atlas candidates. Current milestone status lives in
> `TRACKER.md`; the current dimension registry lives in `docs/DIMENSIONS.md`.

## What just happened

Full session from project initialization through first paper review:

1. Project scaffolded: Cargo workspace (6 crates), CLAUDE.md, design spec, 12-dimension axis pool, parliament voices, stakeholder roles, data sources
2. HPMS data fetched from geo.dot.gov (160,969 records, 28/50 states)
3. `route fetch / build / score / score-all / flow / invest / coverage` all working
4. Coverage analysis: 66.5M Americans (20.4%) beyond 30-mile interstate threshold
5. A.1 (Arterials Tiering) — full paper written (draft stage, unreviewed)
6. B.1 (Missing Links) — full paper written, Round 1 review complete (revision stage)
7. Rubric bumped to v1.1: A3 IRI cap (10→5), B2 partial-graph explicit warning, C3 descriptive-only note

## Rubric version: v1.1

Historical note: at the time of this handoff, `data/scores-all.csv` contained v1.0 scores and needed a v1.1 refresh. That warning is now superseded; the tracked score ledger has been regenerated under v1.4 with confidence labels and current 160-point tier thresholds.

I-80 v1.1 scores: A1=3.0, A3=5.0 (was 10.0), B1=8.2, C2=4.0 — total will drop ~5 points.

## Paper status

| Paper | Stage | Key fact |
|---|---|---|
| A.1 Arterials Tiering | **draft** — not reviewed | Key finding: congestion-stress paradox; centrality-adjusted T1 = 8 trunk lines |
| B.1 Missing Links | **revision** — 4 P1 blockers | 66.5M gap; I-3 top corridor at 46K pop/$B |
| C.1 O-D Reliability | draft stub only | Sections not written; plan.md exists |
| All others | planned | No content yet |

## B.1 P1 blockers (must fix before recheck)

1. **P1.1** Quantify county centroid artifact for large-county states (CA, AZ, NV) — new analysis
2. **P1.2** Verify 1,465 interchange node count vs. FHWA data
3. **P1.3** Add equity paragraph: acknowledge construction-era community harm
4. **P1.4** Contextualize $292B against IIJA budget horizon

## Local infrastructure additions

- `.roles/panel-reviewer/` — 10 transportation domain reviewers (R-T1 through R-T10)
- `data/coverage-gaps.csv` — 1,510 continental US gap counties with population
- `data/cache/acs_county_pop_2022.csv` — Census ACS county population
- `data/cache/hpms_2018.csv` — 160,969 HPMS interstate records (28 states)
- `data/scores-all.csv` — historical note originally marked this stale; current tracked file is v1.4

## Next session priorities (ranked)

### 1. Re-run score-all v1.1 (completed/superseded)
This item was completed and superseded by v1.4. `route score-all` now scores 386 atlas candidates and writes confidence labels to `data/scores-all.csv`.

### 2. Address B.1 P1 revision items (1–2 sessions)
Four blockers before recheck. P1.1 (centroid artifact) requires a new analysis pass — identify the CA/AZ/NV large-county cases and compute a corrected headline number. P1.3 and P1.4 are single-paragraph additions.

### 3. Review A.1 (run panel:publication review A.1)
A.1 is at draft stage with complete text. The tier findings may change after score-all v1.1 runs — verify tier structure before review. Five reviewers from `.roles/panel-reviewer/` are now available (R-T1 transport geographer, R-T3 transport policy, R-T4 network science appropriate).

### 4. Write C.1 O-D Reliability (panel:publication author C.1)
The plan is complete. Needs: PTI data (not yet fetched — `route fetch-hpms` gets HPMS but not FHWA FPM), the max-flow analysis on NY-LA and HOU-CHI corridors, and the managed-lane scenario modeling. Can run `route flow I-80` and `route flow I-35` for current capacity baseline.

### 5. Write B.2 Bottlenecks (panel:publication author B.2)
Depends on score-all v1.1 for accurate A1/A2/A3 corpus distribution. After score-all: `route gap --type bottleneck` will identify V/C > 0.85 corridors. B.2 plan needs writing.

## Key data still missing

| Source | Status | How to get |
|---|---|---|
| FHWA Freight Performance Measures (PTI/TTI) | Not fetched | FHWA website or RITIS portal — needed for A3 real scores |
| NBI bridge data | URL not verified | https://www.fhwa.dot.gov/bridge/nbi/ascii.cfm — download national file |
| BEA county GDP | Not fetched | URL needs verification; `route fetch-acs` pattern |
| FEMA SFHA shapefile | Not fetched | In manifest, needs URL verification |
| USDA rural codes | Not fetched | xlsx format — needs conversion |
| HPMS for TX, TN, VA, WY + 22 other states | Partial | Re-run `route fetch-hpms` (server intermittent; retry for missing states) |

## Corpus state

- Historical snapshot: 227 interstates scored under v1.0 at the time of this handoff.
- Current state: 386 atlas candidates scored under v1.4; see `data/scores-all.csv` and `route calibrate`.
- Centrality-adjusted T1 (from A.1): I-5, I-10, I-35, I-40, I-75, I-80, I-90, I-95
- 1,510 continental US gap counties (30-mile standard) in `data/coverage-gaps.csv`
