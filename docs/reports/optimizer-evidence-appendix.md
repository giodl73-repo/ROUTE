---
name: Optimizer Evidence Appendix
slug: optimizer-evidence-appendix
type: report
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/reviews/communications-crate-coverage-audit.md
  - docs/tier-optimizer-design.md
  - docs/optimizer-constraint-ledger-spec.md
  - docs/optimizer-artifact-manifest.md
  - docs/reviews/milepost-10-optimizer-review.md
  - docs/route-architecture.md
  - docs/national-segment-identity-spec.md
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
  - docs/reports/route-evidence-posture.md
---

# Optimizer Evidence Appendix

## Purpose

This appendix explains the ROUTE optimizer evidence chain for technical
reviewers and sponsor analysts.

The current communications demo shows one threshold-sensitive before/after
fixture. The optimizer system is broader: it turns service promises, bundle
identity, tier regions, candidate columns, stops, contacts, constraints, held
blockers, map hooks, and game hooks into reviewable artifacts.

This appendix does not claim mathematical optimality, construction readiness,
official-plan status, guaranteed SLA, numeric ROI, eligibility, compliance,
endorsement, or public readiness.

## Communications Thesis

ROUTE is not a route ranker. It is an evidence-bounded optimizer loop.

The useful claim is not "the optimizer found the final national plan." The
useful claim is:

> ROUTE can make each optimizer decision inspectable: what promise forced it,
> what bundle or stop it touched, what constraint blocked it, what artifact
> records it, and what review or source step comes next.

## Optimizer Chain

| Stage | Artifact / Command Surface | What It Shows | Claim Boundary |
|---|---|---|---|
| Promise portfolio | `data/t1-sla-candidate-pairs.csv`; `route t1-sla-candidate-pairs --gate` | Which T1 service-promise pairs are selected, ranked, or dropped. | Planning promise portfolio, not operating SLA proof. |
| Candidate columns | `data/tier-candidate-columns.csv`; T1/T2/T3/T4 selector rows | Candidate services, review rows, demotions, and blocked rows. | Candidate row is not a construction recommendation. |
| Stop selection | `data/t1-stop-selector.csv`; `route t1-stop-selector --gate` | Stop chains, split boundaries, and selected service nodes. | Stops are planning/service nodes until source and delivery proof close. |
| Contact and topology repair | `data/t1-topology-repairs.csv`; T2 contact closure artifacts | Real contacts, held contact rows, demotion pressure, and repair actions. | Graph contact validity is not design-standard compliance. |
| Lower-tier pressure | `data/lower-tier-pressure-witnesses.csv`; `data/t3-t4-access-gaps.csv`; `data/t1-feedback-docket.csv` | T3/T4 and T2 pressure can reopen higher-tier questions only through named witnesses. | Lower-tier pressure does not override T1 by score or map convenience. |
| Constraint ledger | `data/optimizer-constraint-ledger.csv`; `route optimizer-constraint-ledger --gate` | Normalized blockers, debts, penalties, repair actions, and blocked claims. | Ledger visibility does not resolve the constraint. |
| Constraint budget | `data/optimizer-constraint-budget.csv`; `route optimizer-constraint-budget --gate` | Rollups by candidate, bundle, tier, and region. | Budget/debt rollups are planning evidence unless sourced and reviewed. |
| Optimizer manifest | `data/tier-optimizer-runs.csv`; `route tier-optimize --all-tiers --gate`; `route optimizer-manifest --gate` | Which stages passed, which are held-known, and which artifacts carry blockers. | Manifest proves bundle shape and gate status, not domain truth or optimality. |
| Map/game hooks | `data/optimizer-map-hooks.csv`; game overlay ledgers | Which map and game artifacts can consume optimizer outputs. | Hooks do not make maps or game surfaces publication-ready. |

## Fixed-Point Story

The optimizer is recursive. It moves down the service hierarchy and allows
source-backed lower-tier failures to bubble back upward:

```text
T1 national promise spine
  -> T2 regional connectors and relief
  -> T3 zone feeders and regional access
  -> T4 local access and terminal obligations
  -> upward pressure when lower tiers cannot attach cleanly
```

A valid optimizer story must therefore show both directions:

| Direction | Meaning |
|---|---|
| Downward | Higher-tier promises create candidate lines, stops, contacts, and lower-tier attachment obligations. |
| Upward | Lower-tier access failures can reopen higher-tier questions only through named evidence, contact, SLA, stop, or topology witnesses. |

## Constraint Discipline

The optimizer's core communications value is constraint visibility.

| Constraint Concept | Communications Translation |
|---|---|
| `identity-blocker` | The candidate cannot be treated as a real service object until identity is repaired. |
| `selection-hard` | The candidate violates a hard rule unless an explicit exception row exists. |
| `claim-blocker` | The bundle can remain visible but cannot support named claims such as SLA, publication, transit, or game readiness. |
| `budget-debt` | The candidate is valid enough to carry forward but has source, repair, upgrade, lifecycle, or maintenance debt. |
| `penalty-soft` | The candidate is worse than an alternative but not automatically forbidden. |
| `review` | The next decision requires a human, policy, domain, or source review artifact. |

This is the main difference between ROUTE and a hand-shaped map: a weak or
blocked route does not disappear. It becomes a typed row with an owner, next
artifact, blocked claim, and reason.

## Held-Known Is A Feature

The optimizer manifest allows `held-known` rows. That is deliberate.

| Held-Known Means | Does Not Mean |
|---|---|
| The blocker is visible and carried forward. | The blocker is solved. |
| Downstream repair or source artifacts can consume the blocker. | The route, map, game, or release claim is public-ready. |
| The optimizer bundle can remain inspectable despite unresolved evidence. | The optimizer has proven the final answer. |

Current held-known examples include T2 region/contact holds and T4 terminal
contact source/proof holds. These are useful because they prevent silent
promotion while still allowing repair, demotion, pressure, source acquisition,
and feedback artifacts to run.

## Safe Story

| Safe Message | Do Not Say |
|---|---|
| ROUTE emits optimizer artifacts that show selected, rejected, held, and repair-needed rows. | ROUTE has selected the final national construction plan. |
| The manifest records passing and held-known optimizer stages. | The manifest proves mathematical optimality or public readiness. |
| Constraint ledgers make blockers, debt, penalties, and repairs visible. | Constraint rows are resolved because they appear in a ledger. |
| Lower-tier pressure can reopen higher-tier decisions through named witnesses. | Any local or high-scoring route can override the T1 promise portfolio. |
| Map and game hooks consume optimizer outputs under evidence labels. | Map/game hooks prove publication, gameplay, or scenario readiness. |

## Current Communications Gap Closed

The 225-mile demo is a good threshold fixture, but it is too small to represent
the full optimizer. This appendix adds the missing technical story:

- ROUTE optimizes service objects through bundle identity, not route labels;
- selected rows and held rows are both valuable evidence;
- constraints are typed and cannot be hidden;
- the manifest records pass and held-known stages;
- maps and game overlays receive optimizer hooks only through labeled artifacts;
- lower-tier access pressure has a disciplined path back upward.

## Remaining Holds

| Hold | Why It Remains |
|---|---|
| Final optimizer proof | Current artifacts are deterministic and auditable, but they do not prove a globally optimal national plan. |
| Construction readiness | Optimizer rows do not close design, ROW, funding, environmental, safety, or delivery review. |
| Guaranteed SLA | Promise portfolios and stop chains are planning artifacts until reliability evidence closes. |
| Public/map/game readiness | Map/game hooks still require publication, browser/game, evidence, and release gates. |
| External rehearsal | A named venue still needs a populated source-backed fixture and affected role review. |

## Recommended Presenter Use

Use this appendix only after the audience asks how ROUTE keeps the vision honest.

Lead with:

> The optimizer is not a black box that says "build this." It emits the claims,
> blockers, debts, held rows, and next artifacts that make a plan reviewable.

Then show only the relevant chain:

| Audience | Show |
|---|---|
| Technical sponsor | Manifest, constraint ledger, map hooks, held-known rows. |
| State DOT | Delivery/asset/source holds, contact validity, and lower-tier pressure. |
| Rural/access reviewer | T3/T4 access gaps, lower-tier pressure, and feedback docket. |
| Funder | Constraint debt and Blueprint downgrade linkage, without numeric ROI promotion. |
| Map/game reviewer | Optimizer hooks and publication/game readiness holds. |

## Gate

Decision: **story_ready_as_technical_appendix; optimizer_claims_held**

Rationale: The optimizer evidence chain can be safely communicated as an
inspectability and governance feature. It should not be used to claim final
route selection, construction readiness, guaranteed service, ROI, eligibility,
compliance, endorsement, or public readiness.
