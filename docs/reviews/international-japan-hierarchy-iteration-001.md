---
name: International Japan Hierarchy Iteration 001
slug: international-japan-hierarchy-iteration-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - data/international-japan-candidate-hierarchy.csv
  - data/international-japan-scorecard-001.csv
  - data/international-japan-candidate-hierarchy-v2.csv
  - tools/render_japan_hierarchy_v2_map.py
  - maps/international/japan-candidate-hierarchy-v2.svg
  - docs/how-to/international-hierarchy-iteration-playbook.md
---

# International Japan Hierarchy Iteration 001

## Result

This applies the score-repair-render loop to Japan and produces:

`maps/international/japan-candidate-hierarchy-v2.svg`

## Scorecard

| Dimension | Score | Repair |
|---|---:|---|
| Coverage | 6 | extend trunk continuity west and add Niigata resilience branch |
| Tier fit | 5 | repair Osaka-Kobe from fixture T4 to T1 |
| Freight value | 7 | preserve Pacific Belt trunk hypothesis |
| Resilience | 6 | add Sendai/Niigata branches with hazard holds |
| Rural/regional access | 5 | keep Kyoto and add alternate-coast access |
| Source readiness | 3 | hold source-bound MLIT, road graph, hazard, and terminal promotion |
| SLA plausibility | 6 | keep high-reliability targets as assumptions |

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Renderer run | `python tools\render_japan_hierarchy_v2_map.py` | pass | v2 SVG map generated |
| Output inspection | compare scorecard, v2 rows, map labels, tier counts, and held claims | pass | v2 rows implement scorecard repairs and map labels preserve held claims |
| Prohibited-claim scan | scan iteration package for promoted prohibited claims | pass | hits are guardrail, held, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **japan_hierarchy_v2_generated; validation_held**

Rationale: The iteration exposes and repairs a trunk-versus-terminal tiering
error while adding resilience branches. It preserves all official-corridor,
ministry approval, guaranteed-SLA, disaster-readiness, construction, ROI,
eligibility, compliance, endorsement, public-readiness, and external-readiness
holds.
