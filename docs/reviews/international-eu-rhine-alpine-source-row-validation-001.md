---
name: International EU Rhine-Alpine Source Row Validation 001
slug: international-eu-rhine-alpine-source-row-validation-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_source_row_validation.py
  - tools/check_eu_rhine_alpine_source_row_validation.py
  - data/international-eu-rhine-alpine-source-row-validation-001.csv
  - data/eu_rhine_alpine_source_link_candidates.csv
  - data/eu_rhine_alpine_source_need_candidates.csv
  - data/eu_rhine_alpine_source_node_candidates.csv
  - data/eu_rhine_alpine_service_target_candidates.csv
---

# International EU Rhine-Alpine Source Row Validation 001

## Result

This validates only the internal consistency of EU dry-run rows against the
field-inventory posture.

The result is explicitly bounded: metadata candidate rows are matched for
internal parser inspection only; node rows remain source-needed; target rows
remain held assumptions. This does not validate an EU source adapter, accept
geometry, replace fixtures, prove topology, or promote official-corridor,
member-state approval, SLA, ROI, construction, validation, public-readiness,
external-readiness, or external-validation claims.

## Gate

Decision: **eu_source_rows_bounded; fixture_replacement_held**

Run:

```powershell
npm run check:eu:source-row-validation
```
