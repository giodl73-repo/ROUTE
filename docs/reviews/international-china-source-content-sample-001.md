---
name: International China Source Content Sample 001
slug: international-china-source-content-sample-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_china_source_content_sample.py
  - tools/check_china_source_content_sample.py
  - data/international-china-source-content-sample-001.csv
  - data/international-china-source-payload-access-001.csv
---

# International China Source Content Sample 001

## Result

China now has a bounded source-content sample layer after payload-access and
dry-run-depth closeout.

The sample records ministry, plan, statistics, standards, and port/waterway
context as content candidates only. It does not accept source rows. It keeps
planning context separate from policy alignment, standards context separate
from design geometry, and statistics/port pages as table-inventory leads.

## Boundary

This is not source-row validation, fixture replacement, parsed-adapter
readiness, geometry acceptance, topology proof, map overlay, official Chinese
corridor designation, policy alignment, route designation, terminal
performance, node completeness, road-access proof, throughput proof, SLA proof,
ROI proof, construction readiness, public readiness, external readiness, or
internal adapter proof.

## Gate

Decision: **china_source_content_sampled; source_row_validation_blocked**

Run:

```powershell
npm run check:china:source-content-sample
```
