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
| `data/t2-contact-resolutions.csv` | Contact resolution and demotion decisions |
| `data/t2-blocker-closure.csv` | Consolidated held blocker dispositions joined to bundle identity/status |
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

## Relief Loops

Relief loops are not automatically T2. They require:

1. source-backed bottleneck, closure, resilience, or throughput evidence;
2. real contacts back to the system;
3. a service story distinct from local circulation;
4. a map treatment that does not create a same-color self-loop.

I285 and I405 are current examples of relief candidates with evidence/closure
history but missing Beck diagnostic completion before they can land cleanly as
T2 service.

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
- I285 and I405 still need Beck diagnostics before map/service landing.
- T3/T4 access is still intake pressure, not full regional optimization.
