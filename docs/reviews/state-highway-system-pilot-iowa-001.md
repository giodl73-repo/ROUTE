---
name: State Highway System Pilot Iowa 001
slug: state-highway-system-pilot-iowa-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_iowa_state_highway_pilot.py
  - tools/check_iowa_state_highway_pilot.py
  - data/state-highway-system-pilot-iowa-001.csv
  - docs/briefs/state-value-brief.md
  - docs/evidence-campaigns/milepost-9-iowa-repeat-window.md
  - data/game/des-moines-diamond-state-fixture.json
  - data/game/des-moines-diamond-session-fixture.csv
---

# State Highway System Pilot Iowa 001

## Result

ROUTE can run a bounded state-highway-system pilot for Iowa.

The pilot translates the state-facing value brief, Des Moines I-35/I-80
operating evidence path, statewide trunk/connector hypothesis, scenario game
fixture, and state-to-regional packet into five reviewable surfaces. This is a
domestic state-use version of the same proof discipline used in the
international package: service-network roles are candidates until source rows,
review, and delivery gates close.

## Boundary

This is not an official Iowa state plan, Iowa DOT endorsement, FHWA approval,
route designation, source-row validation, geometry acceptance, topology proof,
map overlay, construction readiness, funding commitment, guaranteed SLA,
travel-time proof, delivery commitment, numeric ROI, environmental clearance,
right-of-way clearance, maintenance commitment, public readiness, external
readiness, or validation.

## Gate

Decision: **state_highway_system_pilot_ready; state_authority_and_validation_held**

Run:

```powershell
npm run check:state:iowa-pilot
```
