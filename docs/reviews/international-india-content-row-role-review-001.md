---
name: International India Content Row Role Review 001
slug: international-india-content-row-role-review-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_india_content_row_role_review.py
  - tools/check_india_content_row_role_review.py
  - data/international-india-content-row-role-review-001.csv
  - data/international-india-source-content-row-validation-001.csv
---

# International India Content Row Role Review 001

## Result

The India content-row validation outputs pass internal role review only with
holds.

The review accepts the rows for internal content-row planning review because the
extraction candidates trace back to sampled content. It does not treat them as
accepted source rows, source-row validation, fixture replacement, or adapter
promotion.

## Boundary

This review does not claim source-row validation, fixture replacement, parsed
adapter readiness, geometry, topology, map overlay, terminal performance, node
completeness, road access proof, throughput, official corridor, national/state
approval, SLA, ROI, validation, public readiness, external readiness, or
internal adapter proof.

## Gate

Decision: **india_content_row_role_review_pass_with_holds; source_row_validation_blocked**

Run:

```powershell
npm run check:india:content-row-role-review
```
