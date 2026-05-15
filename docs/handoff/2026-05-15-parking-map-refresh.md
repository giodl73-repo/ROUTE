---
name: Parking map refresh handoff
slug: parking-map-refresh
type: plan
status: reviewed
rubric_version: v1.0
author: copilot
created: 2026-05-15
updated: 2026-05-15
sources:
  - data/optimizer-residual-blocker-backlog.csv
  - data/map-atlas.csv
  - data/map-publication-readiness.csv
---

# Parking Map Refresh Handoff

ROUTE is parked with no active wave. The 17 atlas maps have been regenerated and
the map-publication gates pass with zero publication blockers.

## Current residuals

| Family | Residual |
|---|---|
| T2 asset-condition repair debt | 2 rows / $30.0M: LA I-220 at $25.0M and CA I-110 at $5.0M |
| T1 live-event source guard | 1 evidence guard for `t1-live-event-snapshots`; Iowa 511 remains `snapshot_only` |

## Recent source decisions

- LA I-110 was removed from priced repair debt by accepting Louisiana DOTD STIP
  project H.010319 as full-cost repair funding.
- LA I-220 and CA I-110 were checked against official program sources and
  preserved as holds because no full-cost pavement repair funding row was found.
- Downstream priority-A repair package artifacts now contain only LA I-220.

## Resume candidates

1. Continue T1 evidence work by obtaining a true historical/repeated window for
   live-event snapshots.
2. Continue T2 repair debt only if official funding, downgrade, or exclusion
   evidence is found for LA I-220 or CA I-110.
3. Keep maps in publication-ready-held-claims status until evidence/SLA/transit/
   upgrade claims are actually promoted.
