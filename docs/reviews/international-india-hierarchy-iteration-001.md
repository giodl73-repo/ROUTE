---
name: International India Hierarchy Iteration 001
slug: international-india-hierarchy-iteration-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - data/international-india-candidate-hierarchy.csv
  - data/international-india-scorecard-001.csv
  - data/international-india-candidate-hierarchy-v2.csv
  - tools/render_india_hierarchy_v2_map.py
  - maps/international/india-candidate-hierarchy-v2.svg
  - docs/how-to/international-hierarchy-iteration-playbook.md
---

# International India Hierarchy Iteration 001

## Result

This applies the score-repair-render loop to India and produces:

`maps/international/india-candidate-hierarchy-v2.svg`

## Scorecard

| Dimension | Score | Repair |
|---|---:|---|
| Coverage | 6 | add Lucknow and Bengaluru connectors |
| Tier fit | 5 | repair Ahmedabad-Mumbai from fixture T4 to T1 |
| Freight value | 7 | separate port feeders from trunk spine |
| Resilience | 5 | hold monsoon/resilience constraints as source-needed |
| Rural/regional access | 5 | add north/south regional connectors |
| Source readiness | 3 | hold source-bound promotion |
| SLA plausibility | 6 | keep target classes as assumptions |

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Renderer run | `python tools\render_india_hierarchy_v2_map.py` | pass | v2 SVG map generated |
| Output inspection | compare scorecard, v2 rows, map labels, tier counts, and held claims | pass | v2 rows implement scorecard repairs and map labels preserve held claims |
| Prohibited-claim scan | scan iteration package for promoted prohibited claims | pass | hits are guardrail, held, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **india_hierarchy_v2_generated; validation_held**

Rationale: The iteration exposes and repairs a tiering error from the fixture
while preserving all official-corridor, national/state approval, guaranteed-SLA,
construction, ROI, eligibility, compliance, endorsement, public-readiness, and
external-readiness holds.
