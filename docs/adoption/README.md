---
name: ROUTE Open Adoption Guide
slug: route-open-adoption-guide
type: adoption
status: draft
rubric_version: v1.0
author: codex
created: 2026-07-22
updated: 2026-07-22
sources:
  - README.md
  - docs/media/README.md
  - docs/decks/route-one-page.md
  - docs/briefs/route-first-client-wedge-package.md
  - docs/briefs/route-operating-layer-value.md
  - docs/reports/route-evidence-posture.md
  - docs/map-publication-scope.md
---

# ROUTE Open Adoption Guide

## Purpose

ROUTE is public and open to use. This guide gives readers a practical path for
using it without waiting for a formal briefing, sales process, or pilot.

Use ROUTE as:

- a reference model for service-based highway planning;
- a source-backed research corpus for Interstate 2.0 arguments;
- a diagnostic pattern for a state, region, port, corridor, or freight network;
- a map and communications package for explaining service hierarchy;
- a review discipline for separating strong claims from held claims.

Do not treat public availability as official-plan status, agency endorsement,
construction readiness, guaranteed service, numeric ROI, funding eligibility, or
external validation.

## Fast Paths

| If You Are | Start With | What You Can Do |
|---|---|---|
| Public reader | [`README.md`](../../README.md) | Understand the service-promise model and evidence boundary. |
| Reporter or editor | [`docs/media/README.md`](../media/README.md) | Quote ROUTE safely and avoid official-plan drift. |
| Researcher | [`research/publications/`](../../research/publications/) | Inspect papers, bibliographies, panel reviews, revisions, and rechecks. |
| State, MPO, port, or authority planner | [`docs/briefs/route-first-client-wedge-package.md`](../briefs/route-first-client-wedge-package.md) | Reuse the diagnostic shape for a local network question. |
| Freight, logistics, or industry reader | [`docs/briefs/industry-value-brief.md`](../briefs/industry-value-brief.md) | Translate operating pain into service requirements and evidence asks. |
| Funder or sponsor | [`docs/decks/route-one-page.md`](../decks/route-one-page.md) | Understand the strategic case before any corridor-specific claim. |
| Operator or resilience team | [`docs/briefs/route-operating-layer-value.md`](../briefs/route-operating-layer-value.md) | Adapt the DCR idea as advisory decision support. |
| Builder or contributor | [`docs/route-architecture.md`](../route-architecture.md) | Work from bundle identity, source custody, and evidence labels. |
| External reviewer | [`reviewer-guide.md`](reviewer-guide.md) | Submit bounded review notes that name a lens, concern, evidence, and disposition. |

If you are starting a local adaptation, use the
[`local adaptation worksheet`](local-adaptation-worksheet.md) before creating a
map, issue, pull request, or public readout.

## What To Reuse

| Asset | Reuse Pattern |
|---|---|
| Service tiers | Use T1/T2/T3/T4 as role labels for what a road, corridor, or access link is supposed to do. |
| Promise backlog | Convert goals into candidate promises, then mark legal SLA, ROI, construction, and validation claims as held until proven. |
| Evidence labels | Keep claims marked as implemented, heuristic, planned, held, source-needed, or confidence-limited. |
| Research tables | Link papers and panel reviews directly instead of asking readers to trust a summary. |
| Maps | Use maps to explain structure, not to prove official status, SLA, ROI, or construction readiness. |
| Diagnostic package | Ask for places, failures, source owners, current data surfaces, and claims that cannot be made yet. |
| DCR operating layer | Treat monitoring, simulation, and switch playbooks as advisory decision support unless an operator separately grants authority. |

## First Local Adaptation

For a local adaptation, keep the first pass deliberately small.

1. Pick a bounded scope: one state slice, port region, freight corridor,
   terminal district, rural access problem, or resilience scenario.
2. Name the service question: what should this network promise, and what breaks
   that promise?
3. List source owners and evidence surfaces: GIS, traffic, asset, incident,
   freight, port, toll, safety, planning, or local knowledge.
4. Draft candidate roles: T1 spine, T2 regional connector, T3 access mesh, T4
   terminal/local access, resilience overlay, maintenance-only, or explicit
   non-promotion.
5. Mark claim status before making a claim: source-backed, heuristic, held,
   source-needed, or confidence-limited.
6. Produce a short readout: service hierarchy, failure modes, evidence holds,
   next source asks, and questions for review.

The first output does not need to be a finished plan. It should make the
network job, evidence posture, and next review step visible.

The worksheet version of this sequence is
[`local-adaptation-worksheet.md`](local-adaptation-worksheet.md).

## Safe External Language

Use language like:

> ROUTE is an open research and tooling project for service-based highway
> planning. It explores Interstate 2.0 as a hierarchy of freight, regional,
> access, terminal, resilience, and operating promises.

Avoid language like:

> ROUTE proves which roads should be built or designates an official Interstate
> 2.0 network.

## Contribution And Review Targets

Useful external contributions include:

- source inventories for a state, corridor, port, terminal, or region;
- corrections to source interpretation, geography, or claim labels;
- role-review notes from freight, DOT, planning, community, resilience, transit,
  construction, finance, labor, safety, or environmental perspectives;
- reproducible map or diagnostic fixtures;
- research-paper review comments tied to a specific publication directory;
- safer public-language suggestions that reduce overclaiming.

Use the GitHub issue templates for local adaptation proposals and source/claim
corrections. If you are reviewing a claim, paper, map, fixture, or local
adaptation, use [`reviewer-guide.md`](reviewer-guide.md). Pull requests should
use the claim-boundary checklist in `.github/PULL_REQUEST_TEMPLATE.md`.

## Adoption Gate

Decision: **open_for_reference_review_and_adaptation**

Rationale: ROUTE can be used publicly as an inspectable research, mapping,
diagnostic, and review system. Public use remains bounded by the evidence
posture: no official-plan, endorsement, guaranteed-SLA, construction, ROI,
funding, compliance, procurement, or external-validation claim is created by
reuse alone.
