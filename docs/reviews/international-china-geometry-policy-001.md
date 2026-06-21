---
name: International China Geometry Policy 001
slug: international-china-geometry-policy-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_china_geometry_policy.py
  - tools/check_china_geometry_policy.py
  - data/international-china-geometry-policy-001.csv
  - data/international-china-fixture-blocker-001.csv
---

# International China Geometry Policy 001

## Result

China dry-run link and node candidates remain no-geometry internal rows.
Geometry has not been requested, fetched, accepted, joined, or QA-reviewed.

The policy blocks map overlay, topology proof, fixture replacement,
parsed-adapter promotion, operating claims, official-network claims,
policy-alignment claims, terminal-performance claims, SLA, ROI, validation,
public readiness, external readiness, and internal adapter proof.

## Boundary

This policy does not claim source-row validation, fixture replacement, parsed
adapter readiness, geometry, topology, map overlay, official Chinese corridor,
policy alignment, terminal performance, road access, throughput, SLA, ROI,
validation, public readiness, external readiness, or internal adapter proof.

## Gate

Decision: **china_geometry_rejected_for_current_candidates**

Run:

```powershell
npm run check:china:geometry-policy
```
