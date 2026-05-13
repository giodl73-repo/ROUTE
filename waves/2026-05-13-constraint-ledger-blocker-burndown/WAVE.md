---
wave: constraint-ledger-blocker-burndown
date_open: 2026-05-13
status: active
source: optimizer-constraint-budget
---

# Constraint Ledger Blocker Burn-Down

## Mission

Turn the normalized constraint ledger from a visibility surface into an action
queue. The wave starts with the only current hard blocker, `I84` under the T1
promise portfolio, then burns down the largest claim-blocker families exposed in
`data/optimizer-constraint-budget.csv`: T4 zone-assignment gaps, T4 terminal
evidence gaps, T2 game/bundle-binding holds, and map-publication diagnostics.

## Opening Rule

Do not delete or downgrade blockers to improve counts. A blocker is resolved only
when its owning artifact records a decision, the normalized ledger and budget
regenerate, and the relevant manifest/release gates still pass. If the decision
is to carry the hold, the hold must remain visible with a next artifact.

## Inputs Inherited

| Input | Source |
|---|---|
| Constraint ledger doctrine | `docs/optimizer-constraint-ledger-spec.md` |
| Constraint budget backlog | `data/optimizer-constraint-budget.csv` |
| T1 promise exception doctrine | `docs/sla-promise-portfolio.md`; `data/t1-score-exceptions.csv` |
| T3/T4 access doctrine | `docs/t3-t4-access-optimization.md` |
| Route/stop/decision vocabulary | `docs/route-stop-column-schema.md` |
| Manifest/release ownership | `docs/optimizer-artifact-manifest.md`; `data/release-manifest.csv` |
| Prior closeout | `waves/2026-05-13-constraint-ledger-spine/CLOSE.md` |

## Current Backlog Shape

At wave open, `data/optimizer-constraint-budget.csv` exposes:

| Backlog slice | Current signal | First owning artifact |
|---|---:|---|
| T1 hard blocker | 1 `promise_portfolio` hard blocker on `I84` | `data/t1-score-exceptions.csv` |
| T4 zone-assignment gaps | 63 `zone_assignment_gap` rows | `data/t3-zone-map-diagnostics.csv` |
| T4 terminal evidence gaps | 6 `terminal_access_evidence_gap` rows | `data/t3-zone-map-diagnostics.csv` |
| T2 game bundle-binding holds | 15 `game_ops_bundle_binding` rows plus one asset-condition overlap | `data/game/t2-service-overlays.csv`; `data/national-segment-bundles.csv` |
| Beck/map publication holds | T1/T2 label, transfer, geometry, and long-connector blockers | `data/t1-design-policy-actions.csv`; Beck diagnostics |

## Spec Decision

No new doctrine spec is required before Pulse 01. The wave has enough governing
contracts for the first decisions:

- `docs/optimizer-constraint-ledger-spec.md` owns blocker/debt semantics.
- `docs/sla-promise-portfolio.md` owns whether a route outside the selected T1
  promise portfolio may remain T1 by explicit exception.
- `docs/t3-t4-access-optimization.md` owns T3/T4 zone and local-access decisions.
- `docs/route-stop-column-schema.md` owns decision vocabulary and required
  selector fields.

A new spec is only needed if Pulse 02 discovers that T4 zone assignment needs a
new zone taxonomy or terminal source contract not already covered by
`docs/t3-t4-access-optimization.md`.

## Scenario Decision

Run one bounded counterfactual before changing the I-84 claim status: compare
`justify-as-national-relay` versus `demote-to-t2` using the existing selector,
manifest, and constraint-budget gates. Do not start broad traffic, game, or
investment scenarios before the blocker rows are classified; those scenarios
should follow specific T4 zone assignments or T2 game holds.

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - I-84 T1 hard blocker decision | done | I-84 kept as explicit national-relay exception; hard blockers now 0 |
| 02 - T4 zone-assignment queue | planned | classify 63 zone gaps |
| 03 - T4 terminal evidence holds | planned | terminal/source enrichment or explicit holds |
| 04 - T2 game and bundle-binding holds | planned | service-overlay/bundle binding repair |
| 05 - Beck and publication blocker cleanup | planned | map/publication claim holds |
| 06 - Wave close | planned | close after blocker counts and gates agree |

## Done Criteria

- The `I84` hard blocker is either resolved by an explicit T1 exception or carried
  as a deliberate demotion/hold with regenerated selector and manifest artifacts.
- T4 zone-assignment blockers are reduced or converted into explicit zone/local
  access decisions with next artifacts.
- T4 terminal evidence blockers name source/terminal obligations or remain held
  with explicit source actions.
- T2 game/bundle-binding blockers are repaired or carried in game/source
  artifacts without hidden side reports.
- Remaining blockers are summarized in the closeout with counts by tier and
  constraint class.
- `cargo test -p route`, `route optimizer-constraint-ledger --gate`,
  `route optimizer-constraint-budget --gate`, `route tier-optimize --all-tiers
  --gate`, `route optimizer-manifest --gate`, `route release-manifest --gate`,
  and `scripts/check-mileposts.ps1 -SkipTests` pass.

## Non-Goals

- Do not claim every blocker can be resolved in this wave.
- Do not invent route promotions without promise, contact, source, or zone
  witnesses.
- Do not run broad benefit/cost scenarios until a blocker decision names the
  claim being tested.
- Do not remove held rows from the ledger to make counts look better.

