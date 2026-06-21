---
name: International China Fixture Blocker 001
slug: international-china-fixture-blocker-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_china_fixture_blocker.py
  - tools/check_china_fixture_blocker.py
  - data/international-china-fixture-blocker-001.csv
  - data/international-china-dry-run-role-review-001.csv
---

# International China Fixture Blocker 001

## Result

China fixture replacement remains blocked.

The current rows are dry-run context-only standards rows, source-candidate
context rows, a heuristic-held hierarchy carry-forward row, and held service
target assumptions. Role review passes only with holds, and geometry remains
not accepted for the current dry-run rows.

## Boundary

This blocker does not claim source-row validation, fixture replacement, parsed
adapter readiness, geometry, topology, map overlay, official Chinese corridor,
policy alignment, terminal performance, road access, throughput, SLA, ROI,
validation, public readiness, external readiness, or internal adapter proof.

## Gate

Decision: **china_fixture_replacement_blocked**

Run:

```powershell
npm run check:china:fixture-blocker
```
