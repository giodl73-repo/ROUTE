---
name: International China Dry Run Role Review 001
slug: international-china-dry-run-role-review-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_china_dry_run_role_review.py
  - tools/check_china_dry_run_role_review.py
  - data/international-china-dry-run-role-review-001.csv
  - data/china_source_link_candidates.csv
  - data/china_adapter_evidence_labels.csv
---

# International China Dry Run Role Review 001

## Result

The China parser dry-run outputs pass internal role review only with holds.

The review accepts the rows for internal parser dry-run planning review because
the dry-run tables preserve source custody, evidence labels, and blocked claims.
It does not treat them as accepted source rows, source-row validation, fixture
replacement, or adapter promotion.

## Boundary

This review does not claim source-row validation, fixture replacement, parsed
adapter readiness, geometry, topology, map overlay, terminal performance, node
completeness, road access proof, throughput, official corridor designation,
policy alignment, SLA, ROI, validation, public readiness, external readiness,
or internal adapter proof.

## Gate

Decision: **china_dry_run_role_review_pass_with_holds; source_row_validation_blocked**

Run:

```powershell
npm run check:china:dry-run-role-review
```
