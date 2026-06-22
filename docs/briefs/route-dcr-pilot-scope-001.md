---
name: ROUTE DCR Pilot Scope 001
slug: route-dcr-pilot-scope-001
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-22
updated: 2026-06-22
sources:
  - docs/briefs/route-dcr-package.md
  - docs/briefs/route-dcr-renewal-gate-001.md
  - docs/briefs/route-dcr-tabletop-scope-001.md
  - docs/briefs/route-operating-layer-value.md
  - data/route-dcr-pilot-phases-001.csv
  - data/route-dcr-pilot-cadence-001.csv
  - data/route-dcr-pilot-deliverables-001.csv
  - data/route-dcr-pilot-acceptance-001.csv
  - data/route-dcr-pilot-nonclaims-001.csv
---

# ROUTE DCR Pilot Scope 001

## Offer

Run a 30-90 day Decision Control Room pilot for one corridor, terminal,
managed lane, gateway, or state network slice.

The pilot turns a tabletop or diagnostic into a retained operating loop:
monitor service-promise drift, simulate resilience switches, review
signage/routing and EV support posture, maintain evidence boundaries, and
export claim-safe executive readouts.

## Buyer Commitment

The buyer must provide a sponsor, one bounded network scope, one service
promise or unacceptable failure mode, source surfaces or approved manual
fixtures, an operator or owner who can review switch options, and acceptance
that ROUTE remains advisory unless separately authorized.

## Pilot Loop

| Step | Work | Output |
|---|---|---|
| 1 | Lock scope, promise, held claims, and review authority | Pilot scope and promise-at-risk board |
| 2 | Connect approved source surfaces or manual fixtures | Source inventory and hold ledger |
| 3 | Monitor exceptions against service promise | Monitored signal ledger |
| 4 | Run monthly and event-driven simulations | Simulation comparison packets |
| 5 | Review reroute, signage, EV support, recovery, access, asset, communication, and investment postures | Switch playbook register |
| 6 | Export sponsor-safe updates | Executive readout and renewal recommendation |

## Cadence

The retained cadence is in `data/route-dcr-pilot-cadence-001.csv`.

At minimum, the pilot includes weekly exception review, monthly simulation,
event-driven review when a disruption or leadership concern appears, operator
review for switch options, monthly executive readout, and a closeout renewal
decision.

## Deliverables

The delivery ledger is in `data/route-dcr-pilot-deliverables-001.csv`.

Core outputs include a monitored signal ledger, promise-at-risk board,
simulation comparison packets, switch playbook register, signage/routing
advisory queue, EV support queue, evidence boundary ledger, executive readout,
and renewal recommendation.

The renewal recommendation must use
`docs/briefs/route-dcr-renewal-gate-001.md` so continuation is tied to observed
operating memory, repeated decisions, source gates, event triggers, claim
boundaries, or operator review rather than a generic platform pitch.

## Acceptance Gate

The pilot is ready for buyer review when:

- sponsor, scope, service promise, and unacceptable failure mode are named;
- source access or approved manual fixtures exist for every monitored signal;
- operator boundary and review authority are explicit;
- a tabletop scenario, diagnostic failure mode, or buyer event justifies the
  retained cadence;
- weekly exception, monthly simulation, event-driven, operator review,
  executive readout, and closeout cadences are accepted;
- non-claims block traffic control, legal detours, incident command, EV
  availability, guaranteed SLA, ROI, construction, endorsement, public
  readiness, procurement readiness, live integration, and automated control;
- closeout preserves evidence labels and makes renewal optional.

## Boundary

This is a 30-90 day advisory pilot, not a live operations integration,
traffic-control system, emergency command process, legal detour plan,
guaranteed SLA, EV availability guarantee, construction recommendation, ROI
proof, endorsement, procurement-readiness proof, automated-control system, or
public-readiness packet.

ROUTE can help guide signage and routing posture, EV support posture, and
resilience switching decisions, but operator authority stays with the buyer.
