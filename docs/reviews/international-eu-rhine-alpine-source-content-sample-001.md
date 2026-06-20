---
name: International EU Rhine-Alpine Source Content Sample 001
slug: international-eu-rhine-alpine-source-content-sample-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_source_content_sample.py
  - tools/check_eu_rhine_alpine_source_content_sample.py
  - data/international-eu-rhine-alpine-source-content-sample-001.csv
  - data/international-eu-rhine-alpine-source-payload-probe-001.csv
---

# International EU Rhine-Alpine Source Content Sample 001

## Result

EU now has a bounded source-content sample layer after payload probing.

The sample records official EU source context for the current European Transport
Corridors map library, TENtec, GISCO transport datasets, and the Rhine-Alpine
ERTMS/RALP context page. It also records a hard rebase warning: the current map
library is not a clean Rhine-Alpine road-service fixture input.

## Boundary

This is not source adapter validation, geometry acceptance, topology proof,
fixture replacement, official ROUTE corridor designation, member-state
approval, SLA proof, ROI proof, construction readiness, public readiness, or
external readiness.

## Gate

Decision: **eu_source_content_sampled; fixture_replacement_still_blocked**

Run:

```powershell
npm run check:eu:source-content-sample
```
