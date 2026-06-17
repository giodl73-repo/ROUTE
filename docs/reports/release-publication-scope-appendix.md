---
name: Release And Publication Scope Appendix
slug: release-publication-scope-appendix
type: report
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/map-publication-scope.md
  - docs/reviews/communications-external-rehearsal-readiness.md
  - docs/reports/maps-are-not-proof-report.md
  - docs/reports/simulation-game-evidence-boundary.md
  - docs/reports/asset-condition-evidence-appendix.md
  - data/map-atlas.csv
  - data/map-publication-scope-decision.csv
  - data/map-publication-readiness.csv
  - data/map-publication-inventory.csv
  - data/t4-terminal-access-map-exclusion.csv
  - data/source-snapshot-publication-exclusion.csv
  - data/t2-asset-condition-map-publication-exclusion.csv
  - data/release-manifest.csv
  - data/pressure-test-scenarios.csv
  - data/t4-terminal-scenario-readiness.csv
  - data/t2-game-publication-evidence-review.csv
  - data/t2-game-publication-evidence-policy.csv
  - data/t2-game-publication-evidence-blocker-relief.csv
---

# Release And Publication Scope Appendix

## Purpose

This appendix explains the difference between structural map publication,
release-manifest metadata, browser/game readiness, external rehearsal readiness,
and public claim readiness.

ROUTE can have artifacts that are valid to show in a narrow way while stronger
claims remain held. The core communications risk is treating one kind of
readiness as another.

## Core Rule

Publication scope is claim-specific.

```text
artifact exists
  -> render / metadata gate
  -> scope decision
  -> allowed use
  -> held claims
  -> next evidence step
```

An artifact may be release-candidate, structurally render-valid, or allowed as a
held-claim map without being SLA-valid, evidence-valid, transit-ready,
upgrade-ready, game-ready, browser-ready, externally validated, or public-ready.

## Current Artifact Inventory

These counts are local artifact counts from the current repo snapshot, not a
public release decision.

| Artifact | Rows | What It Shows | Boundary |
|---|---:|---|---|
| `data/map-atlas.csv` | 17 | Map ids, paths, types, render commands, dimensions, tier roles, and game use. | Atlas membership is not claim publication. |
| `data/map-publication-scope-decision.csv` | 3 | Scope decisions for map publication blockers and held claims. | Scope decisions do not close excluded evidence claims. |
| `data/map-publication-readiness.csv` | 1 | Structural T1-T4 map publication readiness with zero residual publication blockers and held non-publication claims. | Readiness is map-only and held-claim-labeled. |
| `data/map-publication-inventory.csv` | 17 | Release-facing inventory of structural maps with labels, allowed uses, and not-allowed claims. | Maps remain not evidence-valid, SLA-valid, transit-ready, upgrade-ready, or asset-condition-repaired. |
| `data/t4-terminal-access-map-exclusion.csv` | 1 | T4 terminal-access overlay publication exclusion. | Excludes map/publication claims only; terminal proof and upgrade work remain held. |
| `data/source-snapshot-publication-exclusion.csv` | 1 | Source snapshot publication exclusion. | Excludes current map publication only; live-event evidence remains held. |
| `data/t2-asset-condition-map-publication-exclusion.csv` | 1 | T2 asset-condition map publication exclusion. | Excludes map publication only; SLA, transit, upgrade, and repair obligations remain held. |
| `data/release-manifest.csv` | 199 | Release surface ownership, public status, verification command, and notes. | Manifest metadata is not public endorsement or claim validity. |
| `data/pressure-test-scenarios.csv` | 8 | L2 pressure scenarios and blockers. | Current scenarios remain heuristic unless evidence closes. |
| `data/t4-terminal-scenario-readiness.csv` | 1 held docket | T4 terminal scenario readiness remains empty until source-backed contact proof exists. | No T4 terminal row is scenario-ready or release-ready. |
| `data/t2-game-publication-evidence-review.csv` | 3 | Game publication evidence review rows. | Review does not make game scenarios public-ready. |
| `data/t2-game-publication-evidence-policy.csv` | 3 | Game publication evidence policy rows. | Policy is not release approval. |
| `data/t2-game-publication-evidence-blocker-relief.csv` | 3 | Bounded blocker relief rows. | Relief is limited and does not close browser/game L2 readiness. |

## Map Publication Scope

The current map posture is deliberately narrow:

- 17 structural T1-T4 maps can be treated as publication-ready held-claim maps.
- Render gates pass.
- Residual publication blockers are zero for the structural map claim.
- Evidence, SLA, transit, and upgrade claims remain held.
- T2 asset-condition debt remains outside map publication scope.
- T4 terminal-access proof remains outside map publication scope.
- Source snapshot/live-event evidence remains outside map publication scope.

The required caption pattern is:

```text
Structural T1-T4 map; evidence/SLA/transit/upgrade claims held.
```

Allowed use is structural explanation, campaign atlas context, and bounded
planning discussion. Not-allowed use is proof of service readiness, terminal
access, repairs, public approval, construction, ROI, or SLA performance.

## Release Manifest Boundary

`data/release-manifest.csv` is an ownership and verification surface. It tells
reviewers which artifacts are release candidates, held, public, held-public, and
which command or manual review owns verification.

That is useful metadata, but it should not be described as proof that every
artifact is externally ready. A manifest row can be public while its notes,
verification command, or companion evidence still restrict the claims that can
be made from it.

## Browser, Game, And Scenario Boundary

Browser/game claims need separate evidence:

- L2 browser/game readiness is not closed by map publication.
- Pressure scenarios remain heuristic until their blocking gaps close.
- T4 terminal scenario readiness remains held because no source-backed contact
  rows exist.
- T2 game publication evidence rows preserve labels, policies, and bounded
  relief, but do not make a browser prototype public-ready.

Game or scenario material may be used internally as a teaching or pressure-test
surface only when captions preserve heuristic/source-needed/held labels.

## External Rehearsal Boundary

The communications package currently has an internal rehearsal posture, not an
external rehearsal pass.

External rehearsal readiness remains held until a named venue, selected audience
lane, populated source-backed stakeholder fixture, source custody, affected role
review, selected-material prohibited-claim scan, and L0 closeout exist.

Passing internal pressure tests, adding appendices, or showing structural maps
does not create external endorsement, agency approval, public readiness, or
stakeholder validation.

## Reviewer Pressure Questions

- Is this a map publication claim, release-manifest metadata claim, browser/game
  claim, scenario claim, or external rehearsal claim?
- Which row names the allowed use and held claims?
- Does the caption preserve the required held-claim label?
- Are terminal access, source snapshot, and asset-condition exclusions being
  treated as exclusions rather than proof?
- Does the release manifest row name a verification command and public status?
- Is the scenario still heuristic or source-needed?
- Has browser/game L2 actually closed, or is it scoped out?
- Has a named external venue and source-backed stakeholder fixture been
  recorded?

## Safe Language

| Use This | Avoid This |
|---|---|
| "These structural maps are publication-ready only with held-claim labels." | "The maps prove the plan is ready." |
| "Publication scope excludes specific unresolved claims." | "Excluded claims are solved." |
| "The release manifest records ownership and verification status." | "The manifest proves public readiness." |
| "Scenario and game artifacts remain labeled by evidence posture." | "The game/browser prototype is public-ready." |
| "External rehearsal is held until a named source-backed fixture exists." | "The internal simulation is stakeholder validation." |

## Non-Goals

- This appendix does not make any browser, game, scenario, release, or external
  rehearsal claim public-ready.
- This appendix does not close L2 readiness.
- This appendix does not approve any map as evidence-valid, SLA-valid,
  transit-ready, upgrade-ready, terminal-access-ready, or asset-condition
  repaired.
- This appendix does not claim agency endorsement, legal eligibility,
  compliance, official-plan status, construction readiness, guaranteed service,
  positive ROI, or stakeholder approval.

## Gate

Decision: pass_with_risk for internal communications review.

Rationale: ROUTE has enough publication-scope machinery to explain the
difference between structural map publication and stronger public claims.
External/public use remains gated by selected materials, source custody,
affected role review, prohibited-claim scan, L0/L1/L2 as applicable, and the
specific held-claim rows named by each artifact.
