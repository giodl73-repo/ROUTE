---
name: International EU Rhine-Alpine Current Corridor Rebase 001
slug: international-eu-rhine-alpine-current-corridor-rebase-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_current_corridor_rebase.py
  - tools/check_eu_rhine_alpine_current_corridor_rebase.py
  - data/international-eu-rhine-alpine-current-corridor-rebase-001.csv
  - data/international-eu-rhine-alpine-source-content-sample-001.csv
---

# International EU Rhine-Alpine Current Corridor Rebase 001

## Result

EU cannot honestly move from Rhine-Alpine context rows into fixture replacement
until scope is rebased or explicitly frozen as legacy context.

The current European Transport Corridors map-library source and the legacy
Rhine-Alpine RALP context source are both useful, but they are not the same
proof surface for a road-service fixture.

## Gate

Decision: **current_corridor_rebase_required_before_replacement**

Run:

```powershell
npm run check:eu:current-rebase
```
