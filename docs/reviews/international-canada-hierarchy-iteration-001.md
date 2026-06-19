---
name: International Canada Hierarchy Iteration 001
slug: international-canada-hierarchy-iteration-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - data/international-canada-candidate-hierarchy.csv
  - data/international-canada-hierarchy-scorecard-001.csv
  - data/international-canada-candidate-hierarchy-v2.csv
  - tools/render_canada_candidate_hierarchy_v2_map.py
  - maps/international/canada-candidate-hierarchy-v2.svg
  - docs/reviews/international-canada-candidate-hierarchy-map-001.md
---

# International Canada Hierarchy Iteration 001

## Result

This iteration scores the first Canada candidate hierarchy and produces a
repaired v2 map:

`maps/international/canada-candidate-hierarchy-v2.svg`

## Scorecard

| Dimension | Score | Repair |
|---|---:|---|
| Coverage | 6 | add Ottawa, Regina, Saskatoon, and stronger border logic |
| Tier fit | 5 | up-tier Toronto-Windsor; demote Winnipeg-Thompson pending evidence |
| Freight value | 6 | add border and prairie production connectors |
| Resilience | 6 | keep resilience feeders but require constraints |
| Rural/northern access | 5 | keep northern access visible as T3 until stronger evidence closes |
| Source readiness | 4 | keep graph/need parsing future; hold nodes and constraints |
| SLA plausibility | 6 | keep target classes, not guaranteed SLAs |

## V2 Changes

| Change | Reason |
|---|---|
| Toronto-Windsor moved from T3 to T2. | Border and industrial access looked under-tiered. |
| Winnipeg-Thompson moved from T2 to T3. | Northern access is visible, but source evidence is not enough for T2. |
| Regina and Saskatoon added. | Prairie production and regional access were underrepresented. |
| Ottawa and Quebec City added. | Capital-region and St. Lawrence regional coverage were underrepresented. |
| Fort McMurray rerouted through Edmonton. | The v1 Calgary-Fort McMurray schematic jump was too coarse. |

## Reusable Process

1. Generate a first candidate hierarchy map.
2. Score coverage, tier fit, freight value, resilience, rural/northern access,
   source readiness, and SLA plausibility.
3. Convert every low score into a row-level repair.
4. Generate a revised map.
5. Keep source-bound parsing, official network, SLA, construction, ROI,
   eligibility, compliance, endorsement, public-readiness, and external-readiness
   claims held until evidence closes.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Renderer run | `python tools\render_canada_candidate_hierarchy_v2_map.py` | pass | v2 SVG map generated |
| Output inspection | compare scorecard, v2 rows, map labels, tier counts, and held claims | pass | v2 rows implement scorecard repairs and map labels preserve held claims |
| Prohibited-claim scan | scan iteration package for promoted prohibited claims | pass | hits are guardrail, held, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_hierarchy_iteration_v2_generated; validation_held**

Rationale: The iteration produces a better Canada candidate hierarchy map and a
repeatable score-repair-render process for other regions. It does not validate
the Canada network or promote official-plan, SLA, construction, ROI,
eligibility, compliance, endorsement, public-readiness, or external-readiness
claims.
