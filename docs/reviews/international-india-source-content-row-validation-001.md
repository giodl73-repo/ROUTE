---
name: International India Source Content Row Validation 001
slug: international-india-source-content-row-validation-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_india_source_content_row_validation.py
  - tools/check_india_source_content_row_validation.py
  - data/international-india-source-content-row-validation-001.csv
  - data/international-india-parser-extraction-candidates-001.csv
---

# International India Source Content Row Validation 001

## Result

India extraction candidates now match back to bounded source-content sample
rows.

This closes a content-row traceability step for one NHAI link-context row, three
major-port node rows, and one port-statistics publication lead. It does not
close source-row validation because the current inputs are sampled content
summaries, not accepted road-link, terminal, or statistics table rows.

## Boundary

This is not source-row validation, fixture replacement, parsed-adapter
readiness, geometry acceptance, topology proof, map overlay, official Indian
corridor designation, national approval, state approval, route designation,
terminal performance, node completeness, road-access proof, throughput proof,
SLA proof, ROI proof, construction readiness, public readiness, external
readiness, or internal adapter proof.

## Gate

Decision: **india_source_content_rows_matched; source_row_validation_blocked**

Run:

```powershell
npm run check:india:content-row-validation
```
