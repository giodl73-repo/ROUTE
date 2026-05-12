# SLA Promise Portfolio

## Purpose

The SLA promise portfolio decides which timed freight promises are important
enough to shape the tier system.

This is the upstream contract for T1/T2/T3/T4 selection. A route earns a tier
because it helps meet a promise horizon, not because the route is famous,
visually convenient, or merely high scoring.

## Promise Horizons

The canonical tier windows live in `data/tier-promise-standards.csv`.

| Tier | Promise windows | Meaning |
|---|---|---|
| T1 | 48h / 36h | National freight promises that can convert air, express, pharma, perishables, ecommerce, and high-value industrial lanes |
| T2 | 24h / 12h | Regional freight promises that feed T1 and connect mega-regions |
| T3 | 6h | One-shift feeder access from production zones, ports, smaller metros, and regional hubs |
| T4 | 1h | Terminal, port, rail-yard, warehouse, and last-mile freight access |

The T1 selector may only consume 48h and 36h promise pairs. Shorter windows
belong to lower tiers unless they expose a named T1 stop, topology, or SLA
repair witness.

## T1 Pair Selection Doctrine

`data/t1-sla-pairs.csv` is the current T1 promise portfolio. It should remain a
top-25 portfolio until a deliberate budget change is made.

A T1 pair qualifies when it satisfies several of these conditions:

1. It connects top national freight, population, port, border, industrial, or
   logistics markets.
2. It has a plausible industry-conversion story from air or slower freight to
   relay-backed highway service.
3. It forces a reusable national route segment rather than a one-off path.
4. It improves geographic coverage of the national spine.
5. It creates a useful relay, transfer, or resilience obligation for the T1
   graph.
6. It can be expressed as a 48h or 36h service promise with an auditable route
   and stop chain.

The portfolio should not simply choose the longest routes. A healthy national
set mixes coast-to-coast 48h promises with half-continent 36h promises that
force central, southern, northern, mountain, and Atlantic/Gulf/Pacific coverage.

## Cut Line

The cut line is not the 25th largest city pair. It is the point where another
promise pair no longer changes the selected national spine enough to justify
the route and stop budget it consumes.

A dropped pair should have a reason:

- already covered by stronger selected promises;
- lower-tier horizon is more appropriate;
- route is incomplete or source-gated;
- stop budget impact is too high;
- does not change T1 topology;
- lacks a credible freight conversion story.

The "11th route" or "26th pair" should therefore be reviewable as a marginal
benefit decision, not a hidden manual omission.

## Required Fields

Every T1 SLA pair row must include:

- `pair_id`
- `origin_id`
- `dest_id`
- `target_hours`
- `priority`
- `market_class`
- `required_routes`
- `required_stops`
- `evidence_basis`

`required_routes` and `required_stops` are audit claims. If they change, the T1
line selector, stop selector, Beck alignment, and feedback docket must be
regenerated.

## Route Selection Relationship

The promise portfolio does not directly say "draw this line." It says:

```text
promise pair -> required route candidates -> selected route/stop columns -> topology gate
```

Routes in `required_routes` receive priority in the T1 line selector. A route
outside the portfolio may still enter T1 only through an explicit exception:

- score-backbone exception;
- resilience or relay exception;
- source-backed topology repair;
- accepted upward feedback from `data/t1-feedback-docket.csv`.

Score alone is insufficient.

## Lower-Tier Feedback

T2/T3/T4 pressure can ask T1 to reconsider a line or stop only when it names a
T1 dependency:

1. a 48h/36h pair that improves or becomes feasible;
2. a selected T1 stop or transfer that must be added or moved;
3. a topology repair that protects the accepted T1 graph.

`data/t1-feedback-docket.csv` is the current audit surface for that rule.

## Evidence Labels

The current portfolio is design-grade and heuristic where direct PTI/FPM
validation is absent. It may drive optimizer selection, map design, and game
scenarios, but publication-grade SLA claims remain held until the evidence
campaign provides observed reliability data or a validated model.

## Open Work

Near-term missing pieces:

- a ranked candidate table for pairs 26+ with drop reasons;
- an explicit T2 24h/12h promise-pair portfolio;
- T3/T4 access promise ledgers by zone;
- sensitivity runs showing how the T1 spine changes under top-10, top-25, and
  top-40 promise budgets;
- direct travel-time/relay evidence joins for proof-grade SLA claims.
