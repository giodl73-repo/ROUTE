---
name: International Hierarchy Iteration Playbook
slug: international-hierarchy-iteration-playbook
type: how-to
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/reviews/international-canada-hierarchy-iteration-001.md
  - data/international-canada-hierarchy-scorecard-001.csv
  - data/international-canada-candidate-hierarchy-v2.csv
---

# International Hierarchy Iteration Playbook

Use this loop for each country or region after an initial candidate hierarchy
map exists.

## Steps

1. Score the map on coverage, tier fit, freight value, resilience,
   rural/remote access, source readiness, and SLA plausibility.
2. Treat every score below 7 as a repair row, not a discussion topic.
3. Apply repairs to the candidate hierarchy data.
4. Render a revised map.
5. Record what changed and what remains held.

## Required Outputs

| Output | Purpose |
|---|---|
| scorecard CSV | makes criticism reusable |
| revised hierarchy CSV | makes repairs executable |
| revised SVG map | shows the result |
| iteration review | records decisions and claim boundaries |

## Red Lines

Do not call an iteration an official network, adopted route designation,
guaranteed SLA, construction plan, ROI case, eligibility finding, compliance
finding, endorsement, public-readiness gate, or external-readiness gate.
