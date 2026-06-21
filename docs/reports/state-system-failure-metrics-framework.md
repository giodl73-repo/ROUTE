---
name: State System Failure Metrics Framework
slug: state-system-failure-metrics-framework
type: report
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-system-failure-metric-menu.csv
  - data/state-system-failure-scorecard-template.csv
  - data/throughput-proof-matrix.csv
  - data/t1-snapshot-plan.csv
  - docs/briefs/iowa-state-service-network-goals.md
  - docs/briefs/texas-state-service-network-goals.md
  - docs/reports/state-market-system-value-add-report.md
---

# State System Failure Metrics Framework

## Point

The state product should not stop at "here are your important corridors." A
state already knows that. ROUTE should show the failure modes that are hard to
organize across agencies, maps, project lists, and political priorities:

- overreliance on a few interstate links and major interchanges;
- weak alternate-route service when an interstate, bridge, pass, work zone, or
  urban junction fails;
- underuse of the state highway system as redundancy, access, and load-shedding
  infrastructure;
- last-mile terminal and institutional access failures;
- rural access isolation during disruptions;
- missing recovery-time evidence before anyone can promise an SLA.

These are metric definitions and diagnostic questions. They are not observed
state failure claims until source rows, operating history, graph validation, and
external review are complete.

## Metric Menu

| Metric | What it reveals | Why it matters |
|---|---|---|
| Interchange single-point exposure | Which junction failures can break a statewide promise. | Converts interchange work from isolated projects into service-continuity risk. |
| Alternate-route service penalty | How bad the best fallback becomes after a closure. | A route is not resilient just because a detour exists. |
| Interstate overreliance ratio | Which promises depend too heavily on interstate links. | Shows where state highways may need explicit T2/T3/R roles. |
| State-system redundancy coverage | Which city pairs have credible non-interstate or mixed-system fallback. | Makes state highway value visible without pretending every road is a trunk. |
| Terminal access friction | Which last-mile links make larger corridors underperform. | Keeps ports, airports, yards, hospitals, campuses, and industrial parks inside the service network. |
| Rural access isolation | Which regions become practically disconnected under disruption. | Turns rural access from a political slogan into a service-risk surface. |
| Recovery-time evidence gap | Whether failures have enough timestamp evidence for recovery expectations. | Blocks fake SLA claims and creates the source docket. |
| Service promise mismatch | Where road class and needed service role diverge. | Lets clients debate purpose, not just jurisdiction. |

The machine-readable menu is `data/state-system-failure-metric-menu.csv`. The
starter scorecard template is
`data/state-system-failure-scorecard-template.csv`.

## What This Shows That A State May Not Already See

States usually know the roads. ROUTE should help them see the mismatch between
roads and promises.

| State knowledge | ROUTE diagnostic conversion |
|---|---|
| "This interstate is important." | "Which service promises fail if this interstate is closed, and what is the fallback penalty?" |
| "This interchange is congested." | "Is this a single-point exposure for multiple statewide promises?" |
| "This state route is secondary." | "Does it provide the only credible redundancy for a T1/T2 promise?" |
| "This rural area needs access." | "Which disruption makes it isolated, and which tier should own the recovery expectation?" |
| "This port or airport has access issues." | "Which T4 terminal feeder breaks a trunk promise if ignored?" |
| "We need resiliency." | "Which failure mode, recovery target, alternate path, and evidence source define resiliency?" |

## Example Diagnostic Questions

Colorado:

- If the I-70 mountain corridor fails, what is the best service-preserving
  alternate, and what penalty does it impose?
- Does the Front Range spine have enough state-system redundancy, or is too much
  of the promise loaded onto a narrow interstate set?
- Which airport and Western Slope access claims are terminal/access promises
  rather than trunk promises?

Tennessee:

- If a major Nashville or Chattanooga interchange fails, which east-west or
  southeast-to-middle promises lose practical continuity?
- Is west Tennessee coverage dependent on one interstate spine, or can state
  routes carry a defined resilience role?
- Which Memphis river/terminal access failures would make a statewide freight
  promise underperform?

Missouri:

- If the I-70 spine or a St. Louis/Kansas City gateway fails, what service role
  can the state highway system realistically carry?
- Does Springfield/Ozarks access provide redundancy or only regional coverage?
- Which Mississippi/Missouri river access points need terminal-feeder evidence
  before being promoted?

## Product Implication

The next state-facing deliverable should be a scorecard, not another static map:

1. Priority promises from the state map.
2. Failure modes that can break each promise.
3. Alternate-route penalty and state-system redundancy questions.
4. Interchange, terminal, bridge, pass, work-zone, and rural isolation flags.
5. Evidence posture for each metric: observed, source-needed, heuristic-held, or
   blocked.
6. A revised package sequence: operations, asset repair, interchange recovery,
   state-route redundancy, terminal access, rural access, and long-range capital.

The first implementation should fill the scorecard template with no numeric
scores unless the source posture permits it. A useful first client packet can
still be valuable with `not-scored` rows when those rows identify the exact
source, workshop decision, or operating history needed to score the failure.

## Held Claims

This report does not claim that any state has failed a metric, that a given
interchange is objectively deficient, that a state highway must be promoted, or
that ROUTE has validated observed failure rates, legal SLAs, costs, numeric ROI,
funding eligibility, construction readiness, endorsement, external validation,
public readiness, or state approval.
