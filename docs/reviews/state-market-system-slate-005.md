---
name: State Market-System Slate 005
slug: state-market-system-slate-005
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/international-market-system-map-v1.csv
  - data/international-market-system-map-export-001.csv
  - tools/render_international_market_system_maps.py
  - maps/state/colorado-market-system-v1.svg
  - maps/state/tennessee-market-system-v1.svg
  - maps/state/missouri-market-system-v1.svg
  - docs/reports/state-market-system-value-add-report.md
---

# State Market-System Slate 005

## Scope

This slate adds Colorado, Tennessee, and Missouri to the state market-system map
set.

## State Roles

| State | Product stress test |
|---|---|
| Colorado | Front Range growth, Denver hub, airport feeder, I-70 mountain-pass resilience, Western Slope, plains freight, and southern/southwest rural access |
| Tennessee | Memphis freight gateway, Nashville hub, Knoxville/Chattanooga east spine, Tri-Cities/Appalachian access, river terminal, and west-state coverage |
| Missouri | Kansas City/St. Louis gateways, I-70 spine, Columbia/capital hub, Springfield/Ozarks, Joplin, Mississippi river access, and southeast rural coverage |

## Value-Add Pressure Test

These maps do not assume ROUTE is telling a state where its obvious corridors
are. The value proposition is that ROUTE converts known places into an editable
service portfolio:

- which city pairs are treated as trunk promises, connectors, or access
  feeders;
- which corridors are framed as reliability, resilience, rural access, freight,
  port, border, terminal, or institutional promises;
- which claims remain held until source rows, operating evidence, costs, legal
  authority, and external review exist;
- which tradeoffs a state can change in a workshop before the system generates a
  revised map, SLA slate, and proof docket.

See `docs/reports/state-market-system-value-add-report.md`.

## Held Claims

The maps do not claim official state priority, legal SLA, construction
readiness, cost, numeric ROI, funding eligibility, compliance, endorsement,
external validation, or public readiness.

## Gate

Decision: **state_market_system_slate_005_expanded; validation_held**
