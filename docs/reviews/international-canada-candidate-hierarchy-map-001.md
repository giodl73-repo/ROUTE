---
name: International Canada Candidate Hierarchy Map 001
slug: international-canada-candidate-hierarchy-map-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - data/international-canada-candidate-hierarchy.csv
  - data/international-canada-source-adapter-readiness.csv
  - data/international-canada-source-adapter-gap-backlog.csv
  - tools/render_canada_candidate_hierarchy_map.py
  - maps/international/canada-candidate-hierarchy.svg
  - docs/reviews/international-canada-source-adapter-readiness-001.md
---

# International Canada Candidate Hierarchy Map 001

## Result

ROUTE now has a Canada candidate service hierarchy result map:

`maps/international/canada-candidate-hierarchy.svg`

The map shows:

| Tier | Candidate Role | Count |
|---|---|---|
| T1 | national spine | 4 |
| T2 | regional connector | 3 |
| T3 | access feeder | 5 |

## Boundary

This is a candidate hierarchy map, not an official Canadian network. It does
not claim Transport Canada, provincial, port, or external review; guaranteed
SLA; construction readiness; ROI; funding eligibility; compliance;
endorsement; public readiness; or external readiness.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Renderer run | `python tools\render_canada_candidate_hierarchy_map.py` | pass | SVG map generated |
| Output inspection | compare candidate hierarchy rows, map labels, tier counts, and readiness posture | pass | map shows 4 T1, 3 T2, and 5 T3 candidate links with held-readiness notes |
| Prohibited-claim scan | scan result package for promoted prohibited claims | pass | hits are guardrail, held, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_candidate_hierarchy_map_generated; validation_held**

Rationale: This creates the result artifact: a Canada T1/T2/T3 candidate
hierarchy map. The result remains heuristic-held and source-gated.
