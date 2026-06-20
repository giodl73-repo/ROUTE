---
name: International EU Rhine-Alpine Parser Extraction Candidates 001
slug: international-eu-rhine-alpine-parser-extraction-candidates-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_parser_extraction_candidates.py
  - tools/check_eu_rhine_alpine_parser_extraction_candidates.py
  - data/international-eu-rhine-alpine-parser-extraction-candidates-001.csv
  - data/international-eu-rhine-alpine-source-content-sample-001.csv
---

# International EU Rhine-Alpine Parser Extraction Candidates 001

## Result

EU now has no-geometry source-content extraction candidates analogous to the
Canada extraction-candidate layer, but they are weaker than Canada road-graph
feature rows.

The rows preserve bounded Rhine-Alpine context and TENtec layer context. They do
not replace `data/eu_rhine_alpine_source_link_candidates.csv`, do not accept a
road graph, and do not promote an internal adapter proof.

## Gate

Decision: **eu_extraction_candidates_ready; road_feature_replacement_blocked**

Run:

```powershell
npm run check:eu:extract
```
