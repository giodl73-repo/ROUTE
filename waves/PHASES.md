# ROUTE Wave Phases

> Find the first row with `status: active`; that is the work rail for
> `/route-wave next` and `/route-pulse`.

ROUTE still uses Mileposts for public phase meaning. Waves are the execution
record inside and across mileposts: each wave has a mission, pulses, gates,
reviews, close notes, and the commits that changed the system.

## Wave Index

| Date | Wave | Mission | Status |
|---|---|---|---|
| 2026-05-06 | Ground Survey | Bootstrap ROUTE specs, roles, Rust workspace, first scoring/build/fetch path, and research module frame. | done |
| 2026-05-07 | Research Module Sprint | Expand ROUTE into papers, tier candidates, relay/SLA simulations, maps, and panel/recheck workflow. | done |
| 2026-05-08 | Instrument Calibration | Make the v1.4 scorer evidence-labeled and boringly reproducible across all dimensions. | done |
| 2026-05-09 | Milepost Gates | Close Instrument, Atlas, Fault Lines, and Pressure Test gates with source, scenario, and primitive-test coverage. | done |
| 2026-05-10 | System Became Playable | Interstate Tycoon became the game-facing proof surface and map-backed campaign spine. | done |
| 2026-05-10 | Maps Became Contracts | Map atlas and Beck schematics became gated release/game artifacts instead of screenshots. | done |
| 2026-05-11 | Stops Bend the Map | Beck geometry shifted to stop-first rules: stops own bends, transfers, intersections, labels, and spacing. | done |
| 2026-05-12 | Promise Horizon | Tiering became a promise-horizon system: T1 48/36h, T2 24/12h, T3 6h, T4 1h. | done |
| 2026-05-12 | Optimizer Got a Constitution | SLA portfolio, Beck contract, T2/T3/T4 doctrine, manifest, and role reviews became optimizer doctrine. | done |
| 2026-05-13 | Constraint Ledger Spine | Normalize every blocker/debt/diagnostic source into the optimizer ledger and budget selectors consume. | done |
| 2026-05-13 | Constraint Ledger Blocker Burn-Down | Turn normalized constraint-budget blockers into explicit decisions, starting with the I-84 hard blocker and T4 zone-assignment backlog. | done |
| 2026-05-13 | T4 Terminal Contact Evidence | Convert zone-scoped T4 terminal evidence holds into route-to-terminal contact decisions and scenario-readiness candidates. | done |
| 2026-05-13 | Great Lakes Terminal Contact Sources | Turn the largest terminal-contact source-needed slice into a governed source-acquisition and proof docket. | active |

## Next Active Wave

`2026-05-13-great-lakes-terminal-contact-sources` is active.

## Operating Rules

1. A wave is not a milestone. A wave is a coherent execution arc that can cross
   docs, data, code, maps, and reviews.
2. A pulse is the smallest committable unit that changes behavior, a spec, a
   generated artifact, or a review status.
3. Every pulse plan must name deliverables, gates, non-goals, affected
   artifacts, and governing roles.
4. Reviews land in `waves/{active}/panels/` or `docs/reviews/` and must change
   a claim, gate, artifact, or next action to count.
5. Close a wave only after the wave card, pulse statuses, gates, and commit list
   agree.
