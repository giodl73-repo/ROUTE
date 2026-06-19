---
name: International EU Rhine-Alpine Hierarchy Iteration 001
slug: international-eu-rhine-alpine-hierarchy-iteration-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - data/international-eu-rhine-alpine-candidate-hierarchy.csv
  - data/international-eu-rhine-alpine-scorecard-001.csv
  - data/international-eu-rhine-alpine-candidate-hierarchy-v2.csv
  - tools/render_eu_rhine_alpine_hierarchy_v2_map.py
  - maps/international/eu-rhine-alpine-candidate-hierarchy-v2.svg
  - docs/how-to/international-hierarchy-iteration-playbook.md
---

# International EU Rhine-Alpine Hierarchy Iteration 001

## Result

This applies the Canada score-repair-render loop to a second region and
produces:

`maps/international/eu-rhine-alpine-candidate-hierarchy-v2.svg`

## Scorecard

| Dimension | Score | Repair |
|---|---:|---|
| Coverage | 6 | add Lyon and Strasbourg alternate access rows |
| Tier fit | 6 | keep Alpine T1 candidates but mark constraint risk |
| Freight value | 7 | keep port cluster and Rhine industrial spine |
| Resilience | 5 | add alternate access and keep constraints source-needed |
| Governance complexity | 5 | add governance hold; do not imply corridor approval |
| Source readiness | 3 | hold source-bound promotion |
| SLA plausibility | 6 | keep target classes as assumptions |

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Renderer run | `python tools\render_eu_rhine_alpine_hierarchy_v2_map.py` | pass | v2 SVG map generated |
| Output inspection | compare scorecard, v2 rows, map labels, tier counts, and held claims | pass | v2 rows implement scorecard repairs and map labels preserve held claims |
| Prohibited-claim scan | scan iteration package for promoted prohibited claims | pass | hits are guardrail, held, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **eu_rhine_alpine_hierarchy_v2_generated; validation_held**

Rationale: The iteration confirms the hierarchy loop can be reused outside
Canada. It does not validate an EU corridor, member-state agreement, SLA,
construction priority, ROI, eligibility, compliance, endorsement,
public-readiness, or external-readiness claim.
