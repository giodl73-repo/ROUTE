---
name: International China Hierarchy Iteration 001
slug: international-china-hierarchy-iteration-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - data/international-china-candidate-hierarchy.csv
  - data/international-china-scorecard-001.csv
  - data/international-china-candidate-hierarchy-v2.csv
  - tools/render_china_hierarchy_v2_map.py
  - maps/international/china-candidate-hierarchy-v2.svg
  - docs/how-to/international-hierarchy-iteration-playbook.md
---

# International China Hierarchy Iteration 001

## Result

This applies the score-repair-render loop to China and produces:

`maps/international/china-candidate-hierarchy-v2.svg`

## Scorecard

| Dimension | Score | Repair |
|---|---:|---|
| Coverage | 6 | repair metro-port continuity and add inland candidates |
| Tier fit | 5 | repair Beijing-Tianjin and Guangzhou-Shenzhen fixture downgrades |
| Freight value | 7 | preserve coastal and Yangtze trunk candidates |
| Resilience | 5 | add inland access candidates with constraint holds |
| Regional access | 5 | add Zhengzhou and Chengdu candidates without promotion |
| Source readiness | 2 | hold source-bound road graph, policy, port, and logistics promotion |
| SLA plausibility | 5 | keep long-haul targets as assumptions |

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Renderer run | `python tools\render_china_hierarchy_v2_map.py` | pass | v2 SVG map generated |
| Output inspection | compare scorecard, v2 rows, map labels, tier counts, and held claims | pass | v2 rows implement scorecard repairs and map labels preserve held claims |
| Prohibited-claim scan | scan iteration package for promoted prohibited claims | pass | hits are guardrail, held, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **china_hierarchy_v2_generated; validation_held**

Rationale: The iteration exposes and repairs port/export downgrades while
adding inland access candidates. It preserves all official-corridor, policy
alignment, guaranteed-SLA, construction, ROI, eligibility, compliance,
endorsement, public-readiness, and external-readiness holds.
