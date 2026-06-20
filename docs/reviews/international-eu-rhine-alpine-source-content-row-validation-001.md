---
name: International EU Rhine-Alpine Source Content Row Validation 001
slug: international-eu-rhine-alpine-source-content-row-validation-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_source_content_row_validation.py
  - tools/check_eu_rhine_alpine_source_content_row_validation.py
  - data/international-eu-rhine-alpine-source-content-row-validation-001.csv
  - data/international-eu-rhine-alpine-parser-extraction-candidates-001.csv
---

# International EU Rhine-Alpine Source Content Row Validation 001

## Result

EU extraction candidates validate against bounded source-content sample rows,
not against road-feature rows.

This closes the source-content validation layer and leaves the road-feature,
current-corridor rebase, fixture replacement, target posture, and internal proof
layers blocked.

## Gate

Decision: **source_content_rows_validated; road_feature_rows_blocked**

Run:

```powershell
npm run check:eu:content-row-validation
```
