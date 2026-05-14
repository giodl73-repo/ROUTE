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
| 2026-05-13 | Great Lakes Terminal Contact Sources | Turn the largest terminal-contact source-needed slice into a governed source-acquisition and proof docket. | done |
| 2026-05-13 | Columbus South Terminal Contact Proof | Run the first route-to-terminal proof pilot on the largest Great Lakes district slice. | done |
| 2026-05-13 | Terminal Contact Source Acquisition Spine | Build the manual/cached proof artifact loop that can promote terminal-contact rows without seed laundering. | done |
| 2026-05-13 | T2 Game/Ops Binding Burn-Down | Turn T2 game/ops bundle-binding blockers into explicit bundle decisions before game or incident use. | done |
| 2026-05-13 | T2 Bundle Overlay Repair Spine | Repair or explicitly preserve held T2 bundle-overlay blockers before game/ops claims can pass. | done |
| 2026-05-13 | T2 Service Overlay Diagnostic Binding | Bind service-overlay repair rows to explicit Beck diagnostic decisions before service classes can pass. | done |
| 2026-05-13 | T2 Local Zone Overlay Handoff | Bind local-zone repair rows to explicit T3 zone handoff decisions before national game overlay claims can pass. | done |
| 2026-05-13 | T2 Bundle Readiness Repair Docket | Turn repair-needed bundle-readiness rows into explicit stop-chain, stitched-member, and terminal-stop repair tasks. | done |
| 2026-05-13 | T2 Bundle Readiness Repair Evidence | Probe readiness repair tasks against downstream artifacts before any bundle-readiness replay can promote claims. | done |
| 2026-05-13 | T2 Bundle Readiness Evidence Replay | Convert readiness evidence probes into explicit replay decisions without promoting unresolved game/ops claims. | done |
| 2026-05-13 | T2 National Bundle Readiness Audit | Audit readiness replay decisions against national segment bundles before any structural replay can promote claims. | done |
| 2026-05-13 | T2 Stitched Member Registry Handoff | Bind stitched-member readiness blockers to registry/candidate evidence before bundle membership repair. | done |
| 2026-05-13 | T2 Stitched Member Candidate Scope Review | Separate route-level stitched candidate evidence from blocked bundle ids before membership repair. | done |
| 2026-05-13 | T2 Stitched Member Decision Docket | Turn stitched candidate scope findings into split, merge, or expand decisions before membership repair. | done |
| 2026-05-13 | T2 Stitched Member Split Plan | Convert split decisions into state-scoped candidate bundle choices before membership mutation. | done |
| 2026-05-13 | T2 Stitched Member Selection Docket | Classify state-scoped stitched-member candidate bundles before registry mutation. | done |
| 2026-05-13 | T2 Stitched Member Evidence Contract | Define proof requirements for stitched-member selection decisions before candidate promotion. | done |
| 2026-05-14 | T2 Stitched Member Evidence Acquisition | Turn source-needed stitched-member proof contracts into concrete acquisition targets. | done |
| 2026-05-14 | T2 Stitched Member Source Access Policy | Classify stitched-member DOT source targets before evidence collection. | done |
| 2026-05-14 | T2 Stitched Member Proof Intake | Define manual/cached evidence artifact fields before collection. | done |
| 2026-05-14 | T2 Stitched Member Proof Source Capture | Record source-capture placeholders before any proof acceptance or membership edit. | done |
| 2026-05-14 | T2 Stitched Member Proof Artifact Attachment | Record artifact-attachment placeholders before proof review or membership edit. | done |
| 2026-05-14 | T2 Stitched Member Proof Review Docket | Review artifact-attachment placeholders before candidate disposition or membership edit. | done |

## Next Active Wave

No active wave.

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
