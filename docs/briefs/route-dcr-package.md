---
name: ROUTE Decision Control Room Package
slug: route-dcr-package
type: brief
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-22
updated: 2026-06-22
sources:
  - docs/briefs/route-operating-layer-value.md
  - docs/briefs/route-dcr-pilot-scope-001.md
  - docs/briefs/route-dcr-renewal-gate-001.md
  - docs/briefs/state-network-planner-ui-brief.md
  - docs/reports/route-business-model-report.md
  - docs/reports/route-competitive-landscape-report.md
  - docs/reports/route-evidence-posture.md
---

# ROUTE Decision Control Room Package

## Definition

DCR means Decision Control Room.

The ROUTE DCR is the buyer-facing operating surface that turns the service
network into monitored decisions. It does not replace a traffic management
center, DOT authority, emergency command, dispatch system, or field traffic
control. It gives the authorized operator a structured place to see service
risk, simulate switch options, review evidence boundaries, and export the next
decision.

## Why It Exists

The first proposal answers:

> What should the network promise?

The DCR answers the recurring operating question:

> Is the network still able to keep that promise, and what should we do when it
> is not?

That is why a buyer pays after seeing the initial proposal. The value moves
from a static readout to a maintained decision environment.

## Modules

| Module | Purpose | Example Output |
|---|---|---|
| Service promise board | Shows T1/T2/T3/T4/R promises, holds, and affected routes/nodes | Promise-at-risk list |
| Live or staged signal board | Tracks incident, weather, closure, asset, terminal, EV, charging, and traffic signals | Monitoring exception row |
| Simulation bench | Replays closure, EV range, detour, signage, terminal, and package scenarios | Option comparison table |
| Resilience switchboard | Converts triggers into governed reroute, recovery, access, EV, signage, communications, or investment posture | Recommended switch packet |
| Signage and routing queue | Helps decide what drivers, truckers, EV users, or local access users should see before a bad route choice | Message-board and route-split advisory |
| EV support queue | Flags range, charger, rest-area, staging, queue, and utility risks | EV-sensitive reroute or support note |
| Evidence boundary ledger | Shows source-backed, heuristic, source-needed, held, expired, and unsafe-to-repeat claims | Claim-safe executive language |
| Executive readout exporter | Produces board, authority, sponsor, or leadership summaries | Bounded decision memo |

## DCR Decisions

| Decision Type | Trigger | DCR Output |
|---|---|---|
| Reroute | Closure, severe degradation, route restriction, shared hazard, or terminal access failure | Preferred alternate, blocked alternates, evidence label, and operator approval need |
| Signage | Drivers need earlier lane, route, charge, staging, or detour choice | Signage location theme, message intent, timing, and non-claim boundary |
| EV routing | Detour distance, charger outage, weather range loss, queue risk, or heavy-duty charging gap | EV-safe path, charging/staging note, and source-needed rows |
| Recovery | Incident duration or queue risk exceeds recovery target | Escalation path, staging option, clearance priority, and communication posture |
| Asset | Pavement, bridge, rest, charging, parking, or WIM issue threatens tier promise | Repair/funding evidence queue update |
| Terminal/access | Port, rail, airport, border, warehouse, hospital, campus, or rural access path fails | Local access posture and package-sequence change |
| Communication | Leadership promise is at risk or a claim expires | Claim-safe language and holds to preserve |
| Investment | Recurring monitored failures outrank current package sequence | Re-ranked operations, asset, access, resilience, or capital package |

## Inputs

The DCR can start with staged scenarios and mature into live feeds.

| Input Class | Early Mode | Later Mode |
|---|---|---|
| Incidents and closures | Scenario fixture or manual event row | DOT/511/API incident feed |
| Weather and hazard | Staged closure or hazard overlay | Weather, flood, fire, winter, or emergency feed |
| Traffic and reliability | Heuristic or historical sample | NPMRDS, probe, toll, detector, or operator data |
| EV and charging | Known station/rest-area inventory and outage assumptions | Charger status, queue, utility, fleet, or station telemetry |
| Signage and routing | Candidate message-board and detour playbook | Operator signage system or traveler-info integration |
| Asset condition | Pavement, bridge, rest, parking, charging, and maintenance ledger | Agency asset system or inspection refresh |
| Terminal and access | Source-backed terminal/access rows or held proof tasks | Port, rail, airport, border, warehouse, or local access feed |

## Sellable Packages

| Package | Buyer | Deliverable |
|---|---|---|
| DCR tabletop | Any first buyer | One-day simulation using staged events and the service-network diagnostic. |
| DCR pilot | Toll, port, managed-lane, state, or MPO buyer | 30-90 day monitored pilot with one corridor, terminal, managed lane, gateway, or state package. |
| DCR quarterly review | Retainer buyer | Monitoring exceptions, simulations, switch playbooks, evidence updates, and executive readout. |
| DCR event review | Buyer after a disruption | Post-event replay, missed promise, switch option, signage/routing, EV, and package implications. |
| DCR workbench | Mature buyer | Hosted or local decision surface with scenarios, exports, and evidence ledger. |

## Renewal Logic

The DCR should renew only when the pilot creates operating memory worth
maintaining: changed service hierarchy rows, repeated decision classes,
monitoring history, reusable simulations, source gates, switch playbooks,
evidence-boundary changes, or executive cadence.

The renewal gate is `docs/briefs/route-dcr-renewal-gate-001.md`. It routes the
buyer to quarterly review, event review, workbench buildout, source integration,
signage/routing advisory, EV support advisory, or stop/hold.

## Boundary

The DCR recommends and documents decisions. It does not command field devices,
override operator judgment, issue legal detours, guarantee EV charging
availability, guarantee service windows, prove ROI, prove construction
readiness, or replace emergency authority.

Every DCR output must preserve the distinction between advisory simulation,
operator-approved action, and evidence-backed claim.
