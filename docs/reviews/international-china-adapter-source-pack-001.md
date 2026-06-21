---
name: International China Adapter Source Pack 001
slug: international-china-adapter-source-pack-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_china_adapter_source_pack.py
  - tools/check_china_adapter_source_pack.py
  - data/international-china-adapter-source-pack-001.csv
  - data/international-china-candidate-hierarchy-v2.csv
---

# International China Adapter Source Pack 001

## Result

China now has a source-pack preflight for the generic international proof
kernel.

The pack identifies transport-ministry context, State Council transport-plan
context, National Bureau of Statistics portal context, highway-standards
context, port/waterway context, the existing ROUTE hierarchy fixture, and held
service-target assumptions.

## Boundary

This is source custody preflight only. It does not claim official Chinese
corridor designation, policy alignment, route designation, source-row
validation, fixture replacement, parsed-adapter readiness, geometry acceptance,
topology proof, map overlay, terminal performance, SLA, ROI, construction
readiness, public readiness, external readiness, or validation.

## Gate

Decision: **china_source_pack_preflight_ready; promotion_held**

Run:

```powershell
npm run check:china:source-pack
```
