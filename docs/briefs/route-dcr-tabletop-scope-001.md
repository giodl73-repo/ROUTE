---
name: ROUTE DCR Tabletop Scope 001
slug: route-dcr-tabletop-scope-001
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-22
updated: 2026-06-22
sources:
  - docs/briefs/route-dcr-package.md
  - docs/briefs/route-operating-layer-value.md
  - data/route-dcr-tabletop-scenarios-001.csv
  - data/route-dcr-tabletop-inputs-001.csv
  - data/route-dcr-tabletop-outputs-001.csv
  - data/route-dcr-tabletop-nonclaims-001.csv
---

# ROUTE DCR Tabletop Scope 001

## Offer

Run a one-day Decision Control Room tabletop using staged events and the
buyer's service-network diagnostic.

The tabletop rehearses how the buyer would monitor a promise at risk, simulate
switch options, review signage/routing and EV support posture, preserve
evidence boundaries, and export a bounded decision memo.

## Buyer Commitment

The buyer must bring one bounded network scope, one service promise or failure
mode, available source surfaces, the operator or owner who can review switch
options, and acceptance that ROUTE is advisory until authorized operator review.

## Tabletop Flow

| Step | Work | Output |
|---|---|---|
| 1 | Confirm service promise and held claims | Promise-at-risk board |
| 2 | Load staged incident, closure, EV, signage, terminal, or asset event | Monitoring exception row |
| 3 | Simulate base case, switch option, and blocked option | Simulation comparison |
| 4 | Review reroute, signage, EV support, recovery, access, asset, or investment posture | Switch option packet |
| 5 | Export leadership-safe readout | Bounded decision memo |

## First Tabletop Scenarios

The first scenario menu is in
`data/route-dcr-tabletop-scenarios-001.csv`. It includes winter closure and EV
range stress, terminal access disruption, managed-lane incident recovery, asset
drift, and recurring freight bottleneck cases.

## Acceptance Gate

The tabletop is ready for buyer review when:

- every scenario names trigger, simulated decision, DCR output, evidence status,
  operator boundary, and held claims;
- required inputs identify early mode, later mode, and hold behavior;
- outputs distinguish advisory simulation from operator-approved action;
- non-claims block traffic-control, legal-detour, SLA, EV availability,
  incident-command, construction, ROI, endorsement, and public-readiness claims.

## Boundary

This is a tabletop scope, not a live operations integration, traffic-control
system, emergency command process, legal detour plan, guaranteed SLA, EV
availability guarantee, construction recommendation, ROI proof, endorsement, or
public-readiness packet.

EV availability guarantee remains a blocked claim unless an authorized operator
and source-backed charging evidence explicitly close that gate.
