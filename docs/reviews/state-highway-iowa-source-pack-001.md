---
name: State Highway Iowa Source Pack 001
slug: state-highway-iowa-source-pack-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_iowa_state_source_pack.py
  - tools/check_iowa_state_source_pack.py
  - data/state-highway-iowa-source-pack-001.csv
  - docs/reviews/state-highway-system-pilot-iowa-001.md
  - docs/evidence-campaigns/milepost-9-iowa-repeat-window.md
---

# State Highway Iowa Source Pack 001

## Result

Iowa now has a state-highway source-pack preflight.

The pack names seven bounded source families: state roadway inventory,
state freight/economic context, Iowa 511 operating events, state program and
delivery context, state asset/maintenance context, the Des Moines scenario
fixture, and held state service targets.

## Boundary

This is source custody preflight only. It is not an official Iowa state plan,
Iowa DOT endorsement, FHWA approval, route designation, source-row validation,
geometry acceptance, topology proof, map overlay, construction readiness,
funding commitment, guaranteed SLA, travel-time proof, delivery commitment,
numeric ROI, benefit-cost proof, environmental clearance, right-of-way
clearance, maintenance commitment, public readiness, external readiness, or
validation.

## Gate

Decision: **iowa_state_source_pack_preflight_ready; promotion_held**

Run:

```powershell
npm run check:state:iowa-source-pack
```
