# T2 Regional Treatment

## Purpose

T2 is not a national list of thin decorative lines. T2 is a set of regional
service treatments solved inside the accepted T1 graph.

This document owns the doctrine for selecting, reviewing, demoting, coloring,
and repairing T2 routes after T1 promise routes and stops are fixed.

## Role

T2 serves the 24h/12h promise horizon:

- connect regional freight markets to the T1 spine;
- provide T1 relief where there is source-backed bottleneck or resilience value;
- connect secondary metros, ports, border regions, and logistics clusters;
- create real transfer service between T1 trunks;
- expose regional gaps that should become T3/T4 access work instead of national
  map clutter.

T2 does not exist to draw every known useful highway. A route that lacks
regional service value, real contacts, or terminal-worthy exceptions should fall
to T3/T4.

## Inputs

Current T2 treatment consumes:

| Artifact | Use |
|---|---|
| `data/tier-candidate-columns.csv` | Route-service candidate columns and repair lineage |
| `data/t2-regionalizer.csv` | First-pass regional treatment rows |
| `data/t2-service-selection.csv` | Service selection, Beck diagnostics, duplicate checks, and parent-trunk lineage |
| `data/t2-service-diagnostic-queue.csv` | Bundle-ready service rows still missing Beck/service diagnostics before map or game binding |
| `data/t2-parallel-service-queue.csv` | Close-parallel Beck service rows that need spacing, split, merge, or promotion review |
| `data/t2-contact-resolutions.csv` | Contact resolution and demotion decisions |
| `data/t2-blocker-closure.csv` | Consolidated held blocker dispositions joined to bundle identity/status |
| `data/t2-bundle-repair-queue.csv` | Bundle-blocked candidate-review rows and the exact registry/stop repair needed before T2 re-entry |
| `data/t3-t4-pressure-intake.csv` | Lower-tier intake after T2 demotion or upgrade pressure |
| `data/t1-feedback-docket.csv` | Conservative upward feedback from lower tiers to T1 |

Future T2 selectors should also consume a T2 24h/12h promise-pair portfolio once
that artifact exists.

## Treatment States

| State | Meaning | Allowed next action |
|---|---|---|
| `selected-treatment` | Route has enough regional service/contact evidence to remain T2 for this pass | render, map hook, game hook |
| `review-treatment` | Route may be T2, but parent region, diagnostic, contact, or service proof is incomplete | review, repair, split, or demote |
| `blocked` | Route cannot be claimed as T2 without source/contact/endpoint proof | repair or demote |
| `lower-tier-pressure` | Route is useful but belongs in T3/T4 until evidence changes | T3/T4 intake |

Review is not failure. Review is the mechanism that prevents the map from
pretending the graph is cleaner than it is.

## Bundle Closure

T2 blocker closure rows must carry bundle posture. A held route is not fully
diagnosed until `data/t2-blocker-closure.csv` records its `segment_bundle_id`
when one exists, its `bundle_status`, and the bundle action from
`data/national-segment-bundles.csv`.

This keeps three problems separate:

- contact proof missing even though a bundle is ready;
- service geometry waiting on stop-chain or terminal-stop work;
- route-family ambiguity where no bundle can be attached yet.

The optimizer may demote, review, or split a route only after that bundle
posture is explicit.

Candidate-column closure is also bundle-gated. A row with accepted contact
evidence can re-enter regionalizer review only when the associated blocker
closure has a ready bundle. If the blocker closure says `bundle-missing`,
`needs-stop-chain`, or `needs-terminal-stop`, the candidate remains blocked
until the bundle registry catches up.

Those blocked rows are emitted as `data/t2-bundle-repair-queue.csv`. That queue
is the repair contract between T2 optimization and the core bundle registry: it
names the route, blocker class, current bundle posture, required artifact, next
artifact, and optimizer effect before the route can re-enter regional treatment.
When no candidate-review rows remain bundle-blocked, the queue emits a single
clearance row and points the optimizer to `data/t2-service-selection.csv`.

Bundle-cleared rows may still require a service diagnostic before they can
receive a map/game treatment. Segment candidate generation keeps those rows
bundle-addressable so the registry, pavement docket, game overlays, and future
diagnostic work can refer to the same `segment_bundle_id` without treating the
route as selected T2 service prematurely.

`data/t2-service-diagnostic-queue.csv` owns that next handoff. It is emitted for
bundle-ready rows that need `data/beck-t2-diagnostics.csv` before they can
receive a service class, map treatment, or game overlay. When no rows are
missing diagnostics, it emits a single `service-diagnostic-clear` row.

When the missing diagnostic is caused by a multi-state three-digit route label,
the queue points back to the route-family lane instead of asking for a Beck row
against the unsplit label. `route t2-route-family-splits --gate` consumes those
service-diagnostic rows and emits `split-numbered-service-family` actions, so
national T2 rendering waits for represented segment families rather than
treating every I-295 or I-275 as one interchangeable service.

The split is materialized upstream in `route tier-segment-candidates`: T2 routes
with `split-numbered-service-family` rows receive state-scoped
`segment_bundle_id` and `stitch_group_id` values. Once those bundles exist, the
route-family docket keeps a `segment-family-split-complete` row so future
regeneration does not collapse the family back into a single route-label bundle.

## Contact Rules

A T2 route normally needs at least two valid system contacts:

1. real T1/T2 graph contact;
2. selected stop or transfer node, not a near miss;
3. parent-trunk lineage where color or service inheritance is claimed;
4. source-backed exception if the route is one-ended but terminal-worthy.

A T2 route may have one contact only when it has a terminal-worthy exception,
such as a major port, border, logistics hub, or endpoint role with source-backed
regional value.

No T2 line may terminate visually near T1 without a selected contact or terminal
exception.

## Parent-Trunk And Color Lineage

T2 color follows service lineage.

| Case | Treatment |
|---|---|
| One parent trunk | Use parent color only when contact and service lineage are proven |
| Two parent trunks | Split color at a selected transfer or use audited gradient lineage |
| More than two parent trunks | Prefer service split, branch treatment, or regional inset |
| Missing parent trunk | Hold for parent-region review or render in neutral review style |

The map cannot use color to imply a parent relationship that the service column
does not record.

## Duplicate And Parallel Service

T2 routes are allowed to be parallel only when they provide distinct service.

Distinct service can come from:

- different parent-trunk pair;
- unique stops or terminals;
- source-backed relief value;
- resilience route under an incident or closure scenario;
- different service horizon or regional market.

Duplicate service should resolve to one of:

- `keep`: distinct service is proven;
- `keep-primary-review`: likely primary route, but peer treatment remains open;
- `merge-review`: branch, trunk sharing, or one-line schematic treatment needed;
- `demote-review`: route is mostly local or subordinate service.

Same-color loops, close parallel duplicates, and unseparated branch pairs should
never be solved by drawing harder. They need a service decision.

`data/t2-parallel-service-queue.csv` turns those close-parallel cases into an
explicit review surface. A route may remain visible while it has a distinct
service basis, but it cannot become an automatic keep or upward promotion until
the queue records whether to space it, split it, merge it, or keep it as a
documented parallel service.

## Relief Loops

Relief loops are not automatically T2. They require:

1. source-backed bottleneck, closure, resilience, or throughput evidence;
2. real contacts back to the system;
3. a service story distinct from local circulation;
4. a map treatment that does not create a same-color self-loop.

I285 and I405 are current examples of relief candidates with evidence/closure
history that now have Beck diagnostics, service classes, and bundle overlays,
while remaining review-treatment rows rather than unconditional T2 promotion.

## Regionalization Rules

T2 should be solved as full treatments inside T1-bounded regions:

```text
accepted T1 graph
  -> T1-bounded region or component
  -> candidate T2 service columns
  -> contact validation
  -> duplicate/parent-trunk review
  -> selected regional treatment
```

The first current regionalizer still carries a large bridged component. That is
a held-known state, not proof that one national T2 component is the desired
final form.

## Bubble-Up Rules

T2 can send pressure upward only through a named T1 dependency:

- a selected T1 SLA pair improves or becomes feasible;
- a T1 stop/contact/topology repair is required;
- a source-backed exception changes the national promise portfolio.

Otherwise, T2 pressure stays in T2 contact review or moves down into T3/T4
regional access. High score alone is not a T1 promotion path.

## Map And Game Semantics

T2 map line classes are service classes, not decoration:

| Class | Meaning |
|---|---|
| `connector` | ordinary regional bridge between parent trunks |
| `compact-service` | short, dense local T2 service |
| `transfer-spine` | transfer-heavy T2 service |
| `long-connector` | long regional connector needing condensed schematic treatment |

Game hooks should target service class and selection action, not just route
name. This lets incidents, upgrades, restitching, and special lanes act on
service concepts instead of hand-picked map lines.

## Gate Requirements

A T2 treatment gate should fail when:

- selected treatment lacks contact evidence;
- selected treatment lacks diagnostic-backed service action;
- duplicate or close-parallel service is kept without a distinct-service basis;
- parent-trunk color is used without parent-trunk lineage;
- a route terminates near T1 without stop/contact proof;
- a relief loop has no source-backed relief evidence;
- a lower-tier route is promoted upward without a named dependency.

Review rows may pass a bundle only when the manifest records them as held-known
or routes them to the next artifact.

## Current Status

Current implementation status:

- `data/t2-regionalizer.csv` emits selected and review treatments.
- `data/t2-service-selection.csv` joins service rows to Beck diagnostics.
- `data/t2-bubble-up-review.csv` prevents T3 pressure from reopening T2 without
  contact proof.
- `data/t1-feedback-docket.csv` prevents lower-tier pressure from reopening T1
  without a named T1 dependency.

Current gaps:

- T2 still lacks its own 24h/12h promise-pair portfolio.
- The large bridged component needs real T1-bounded region splitting.
- T2 stops are not yet selected with the same maturity as T1 stops.
- T2 relief rows such as I285 and I405 still need policy review before any T1
  promotion, even though their T2 Beck/service diagnostics now land cleanly.
- T3/T4 access is still intake pressure, not full regional optimization.
