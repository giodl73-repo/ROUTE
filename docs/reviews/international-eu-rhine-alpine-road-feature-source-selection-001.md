---
name: International EU Rhine-Alpine Road Feature Source Selection 001
slug: international-eu-rhine-alpine-road-feature-source-selection-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_road_feature_source_selection.py
  - tools/check_eu_rhine_alpine_road_feature_source_selection.py
  - data/international-eu-rhine-alpine-road-feature-source-selection-001.csv
  - docs/reviews/international-eu-rhine-alpine-current-corridor-rebase-001.md
---

# International EU Rhine-Alpine Road Feature Source Selection 001

## Result

EU now has selected next-probe source families for the two Canada-parity
blockers:

- GISCO transport networks for a road-feature metadata probe.
- GISCO Ports 2013 for a port-node metadata probe.

The current European Transport Corridors page remains scope-rebase context.
The Rhine-Alpine page remains legacy corridor context only.

## Boundary

This does not accept geometry, validate road features, replace fixtures, prove a
node catalog, prove terminal performance, authorize an official corridor, prove
SLA or ROI, or promote internal adapter proof.

## Gate

Decision: **eu_next_probe_sources_selected; replacement_still_blocked**

Run:

```powershell
npm run check:eu:road-source-selection
```
