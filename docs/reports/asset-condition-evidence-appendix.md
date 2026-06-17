---
name: Asset Condition Evidence Appendix
slug: asset-condition-evidence-appendix
type: report
status: draft
rubric_version: v1.0
author: copilot
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/tier-pavement-standards.md
  - data/tier-pavement-standards.csv
  - data/tier-pavement-docket.csv
  - data/tier-pavement-source-gaps.csv
  - data/tier-pavement-debt-budget.csv
  - data/tier-pavement-acquisition-plan.csv
  - data/tier-pavement-acquisition-docket.csv
  - data/tier-pavement-source-access.csv
  - data/tier-pavement-source-fetch-attempt.csv
  - data/tier-pavement-source-fetch-review.csv
  - data/tier-pavement-unmatched-join-review.csv
  - data/tier-pavement-repair-debt-review.csv
  - data/tier-pavement-repair-disposition.csv
  - data/tier-pavement-repair-funding-package.csv
  - data/tier-pavement-funding-commitment-review.csv
  - data/tier-pavement-downgrade-exclusion-decision.csv
  - data/tier-pavement-funding-evidence-contract.csv
  - data/tier-pavement-funding-evidence-source-access.csv
  - data/standards-l1-inventory.csv
  - docs/reports/standards-blueprint-gates-appendix.md
  - docs/reports/route-evidence-posture.md
---

# Asset Condition Evidence Appendix

## Purpose

This appendix explains how ROUTE handles pavement, bridge, and asset-condition
evidence without turning a visible service bundle into an SLA-ready, transit-
ready, upgrade-ready, map-ready, or construction-ready claim.

The important distinction is simple: a bundle can remain addressable while asset
debt remains unpaid. ROUTE keeps the service object visible so reviewers can see
the debt, source gap, repair task, funding evidence need, or downgrade decision.
Visibility is not readiness.

## Core Rule

Asset condition is a service-claim constraint, not decoration.

```text
tier pavement / bridge standard
  -> segment-level asset evidence
  -> bundle-level source or repair gap
  -> optimizer debt budget
  -> source / funding / repair docket
  -> downgrade, exclusion, or accepted evidence replay
```

No asset ledger row proves construction readiness, operating SLA, transit
readiness, upgrade readiness, positive ROI, legal eligibility, agency
compliance, public release readiness, or endorsement.

## Current Artifact Inventory

These counts are local artifact counts from the current repo snapshot, not
claims that asset evidence is complete nationally.

| Artifact | Rows | What It Shows | Boundary |
|---|---:|---|---|
| `data/tier-pavement-standards.csv` | 4 | T1/T2/T3/T4 pavement and ride-quality floors, source contracts, and validation status. | Planning thresholds are gates, not engineering design approval. |
| `data/tier-pavement-docket.csv` | 3,947 | Segment/member pavement status joined to tier candidates by bundle, stitch group, national segment id, edge id, state, IRI, repair action, and next artifact. | Docket visibility does not prove SLA, transit, upgrade, or publication readiness. |
| `data/tier-pavement-source-gaps.csv` | 6 | Bundle-level repair/source gaps with blocked member counts, affected states/edges, source action, and optimizer effect. | A bundle can remain service-addressable while pavement debt is held. |
| `data/tier-pavement-debt-budget.csv` | 2 | Optimizer-facing evidence and repair debt costs for affected bundles. | Cost units are planning penalties, not budget authority or ROI. |
| `data/tier-pavement-acquisition-plan.csv` | 5 | State-level acquisition plan for pavement source coverage. | Acquisition plan does not accept source evidence. |
| `data/tier-pavement-acquisition-docket.csv` | 28 | Runnable fetch/rebuild/verify tasks for pavement source acquisition. | A task is not a completed fetch or accepted evidence row. |
| `data/tier-pavement-source-access.csv` | 3 | Source-access policy rows for scoped pavement acquisition tasks. | Access policy does not mutate evidence or reduce blockers. |
| `data/tier-pavement-source-fetch-attempt.csv` | 3 | Source-fetch attempt summaries. | Fetch attempt does not automatically accept evidence. |
| `data/tier-pavement-source-fetch-review.csv` | 3 | Fetch outcomes reviewed against current source gaps. | Review preserves unresolved blockers until join/evidence closure. |
| `data/tier-pavement-unmatched-join-review.csv` | 3 | Unmatched priority-A pavement join review. | Cached source records are not enough if joins still fail. |
| `data/tier-pavement-repair-debt-review.csv` | 1 | Priority-A repair debt review before relief. | Repair debt review does not grant relief. |
| `data/tier-pavement-repair-disposition.csv` | 1 | Repair disposition and relief eligibility. | Disposition can hold, downgrade, or route evidence; it is not repair completion. |
| `data/tier-pavement-repair-funding-package.csv` | 1 | Funding evidence package for a repair row. | Package status is not a funding commitment. |
| `data/tier-pavement-funding-commitment-review.csv` | 1 | Review of accepted funding commitment artifact status. | No commitment is accepted without artifact evidence. |
| `data/tier-pavement-downgrade-exclusion-decision.csv` | 1 | Downgrade or exclusion decision for unfunded repair rows. | Downgrade/exclusion prevents overclaiming; it does not repair the asset. |
| `data/tier-pavement-funding-evidence-contract.csv` | 2 | Required evidence for pavement funding relief. | Evidence contract is not accepted evidence. |
| `data/tier-pavement-funding-evidence-source-access.csv` | 3 | Source-access policy for funding evidence targets. | Source access remains policy until accepted evidence is attached. |
| `data/standards-l1-inventory.csv` | 13 | L1 source inventory rows, including WIM, rest, bridge, pavement, T3/T4, and maintenance evidence gaps. | Inventory rows are roadmap/source posture, not proof closure. |

## Pavement Evidence Chain

`docs/tier-pavement-standards.md` defines pavement as a ride-quality floor for
service claims. It is deliberately stricter than a visual map check:

- T1 national promise spine cannot claim timed freight or intercity coach
  readiness over rough member segments.
- T2 connectors cannot claim 24h/12h promise support when ride quality forces
  schedule padding.
- T3/T4 feeders and terminal access rows remain maintenance-first when local
  asset sources are missing or poor.

`data/tier-pavement-docket.csv` is the segment/member surface. It can mark rows
as `pavement-floor-pass`, `pavement-repair-required`,
`pavement-source-needed`, `missing-tier-standard`, or `missing-graph-edge`.
Rows that fail or need sources are not errors to hide; they are the evidence
debt that must stay visible.

## Bundle-Level Debt

`data/tier-pavement-source-gaps.csv` rolls member debt up to service bundles.
This is where the communications story should be precise:

| Safe To Say | Do Not Say |
|---|---|
| "The bundle remains service-addressable while pavement debt is priced and paid." | "The bundle is SLA-ready." |
| "Blocked member counts and affected edge ids are visible." | "Every member is ride-quality ready." |
| "The next artifact names a source, repair, or payment action." | "The repair is funded or complete." |
| "The optimizer carries debt penalties." | "The debt budget is an ROI or construction estimate." |

This lets ROUTE keep reviewing the service object without laundering asset debt
into readiness.

## Source Acquisition And Fetch Review

The acquisition chain converts bundle gaps into state-level work:

1. `data/tier-pavement-acquisition-plan.csv` groups affected routes, bundles,
   and blocked member counts by state/source family.
2. `data/tier-pavement-acquisition-docket.csv` names fetch, rebuild, and verify
   commands.
3. `data/tier-pavement-source-access.csv` records source-access policy before
   scoped fetches.
4. `data/tier-pavement-source-fetch-attempt.csv` records fetch attempt results.
5. `data/tier-pavement-source-fetch-review.csv` reviews outcomes without
   granting automatic relief.
6. `data/tier-pavement-unmatched-join-review.csv` keeps join failures visible
   when cached source records still do not attach to the affected members.

The discipline matters: a source fetch is not accepted evidence until it joins
to the right segment/member and survives review.

## Repair Funding And Downgrade

Pavement repair debt has its own evidence chain:

- repair debt review;
- repair disposition;
- funding package;
- funding commitment review;
- downgrade or exclusion decision;
- funding evidence contract;
- funding source access.

This prevents a common overclaim: "we found the repair need, therefore the
route can be promoted." ROUTE instead requires accepted funding evidence,
artifact attachment, metadata capture, and replay before relief can affect the
shared constraint ledger.

Downgrade/exclusion rows are evidence discipline. They keep unsupported asset
readiness claims out of maps, overlays, SLA surfaces, and Blueprint packages.

## Bridge And Other Asset Evidence

Bridge, WIM, rest, parking, charger, and local maintenance evidence currently
sit mainly in `data/standards-l1-inventory.csv` as source-inventory work:

| Standard | Current Posture |
|---|---|
| `T1-BRIDGE` | NBI bridge records are cached but not joined to graph/route standards; clearance, posting, and condition blockers remain L1 work. |
| `T2-RESILIENCE` | Alternate-route bridge and capacity evidence is missing. |
| `T4-MAINTENANCE` | Local asset-condition data is not joined to T4 maintenance claims. |
| `T1-REST` | Rest area and truck-parking inventory and occupancy history are missing. |
| `T1-OPS-WIM` | Weigh-station/WIM facility inventory and delay distribution are missing. |

The communications package should name these as source-roadmap and standard-gate
items. It should not imply that bridge, WIM, rest, parking, charging, or local
maintenance readiness is closed.

## Reviewer Pressure Questions

- Which bundle, member segments, and edge ids carry the asset blocker?
- Is the issue missing source evidence, failed pavement floor, unmatched join,
  repair debt, funding evidence, bridge clearance/posting, or local asset data?
- Does the row remain service-addressable but claim-held?
- Which next artifact closes the blocker?
- Has a source fetch been accepted and joined to the right member rows?
- Has repair funding evidence been accepted, attached, and replayed?
- Are map, SLA, transit, upgrade, game, and Blueprint claims still held where
  asset debt remains?
- Are bridge and local maintenance claims still labeled as L1 inventory work
  when joins are missing?

## Safe Language

| Use This | Avoid This |
|---|---|
| "Asset evidence is carried as source, repair, funding, or downgrade debt." | "Asset readiness is closed." |
| "A service bundle can stay addressable while asset claims are held." | "A visible bundle is ready for service." |
| "Pavement debt blocks SLA, transit, upgrade, and publication claims." | "A map-ready corridor is pavement-ready." |
| "Bridge and clearance evidence remain L1 inventory work until joined." | "Bridge standards are satisfied nationally." |
| "Funding evidence must be accepted before repair relief." | "A repair package means the repair is funded." |

## Non-Goals

- This appendix does not close any pavement, bridge, WIM, rest, parking,
  charging, or local maintenance evidence gap.
- This appendix does not claim construction readiness, operating SLA, transit
  readiness, map publication readiness, positive ROI, eligibility, compliance,
  public release readiness, or agency endorsement.
- This appendix does not turn planning debt units into budget authority,
  procurement scope, or engineering cost estimates.
- This appendix does not claim a bridge or pavement standard is nationally
  satisfied because an L1 inventory row exists.

## Gate

Decision: pass_with_risk for internal communications review.

Rationale: ROUTE has a concrete pavement and asset-condition evidence chain
that should be visible to technical reviewers. Stronger asset, map, SLA,
transit, upgrade, Blueprint, game, funding, construction, or release claims
remain gated by accepted source joins, repair/funding evidence, replayed blocker
relief, bridge/clearance joins, role review, and publication-scope validation.
