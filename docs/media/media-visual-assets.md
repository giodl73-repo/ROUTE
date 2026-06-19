---
name: ROUTE Media Visual Assets
slug: route-media-visual-assets
type: media
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/media/README.md
  - docs/media/media-claim-guide.md
  - docs/media/media-source-index.md
  - docs/reports/maps-are-not-proof-report.md
  - docs/map-publication-scope.md
  - data/map-publication-inventory.csv
  - data/map-publication-readiness.csv
  - data/international-portability-pilot-map-index.csv
  - docs/reviews/international-portability-pilot-map-run-001.md
  - docs/reviews/international-canada-source-adapter-preflight-001.md
  - data/international-canada-adapter-coverage-preflight.csv
  - data/international-canada-candidate-hierarchy.csv
  - docs/reviews/international-canada-candidate-hierarchy-map-001.md
  - data/international-canada-candidate-hierarchy-v2.csv
  - docs/reviews/international-canada-hierarchy-iteration-001.md
  - docs/reviews/international-eu-rhine-alpine-hierarchy-iteration-001.md
  - docs/reviews/international-india-hierarchy-iteration-001.md
  - docs/reports/route-evidence-posture.md
---

# ROUTE Media Visual Assets

## Purpose

This guide tells reporters and editors which ROUTE visual assets can be used
for media context and what captions must say.

It is not an image license, brand guide, map-publication approval, public
release gate, agency approval, or proof package. It does not create official
plan status, construction readiness, guaranteed service, numeric ROI,
eligibility, compliance, agency endorsement, stakeholder validation,
public-readiness, or external-readiness.

## Current Visual Posture

ROUTE map images can be used as structural service visuals with held-claim
captions.

The controlling map-publication posture is:

| Artifact | Current Decision |
|---|---|
| `data/map-publication-readiness.csv` | 17 T1-T4 structural maps pass render/publication scope with held claims. |
| `data/map-publication-inventory.csv` | Each map lists allowed use, required label, and not-allowed claims. |
| `docs/map-publication-scope.md` | Structural maps may be published as maps only when labeled as held-claim surfaces. |
| `docs/reports/maps-are-not-proof-report.md` | Maps are useful visual argument surfaces, not proof substitutes. |

## Required Caption Pattern

Use this pattern for any ROUTE map:

> ROUTE structural service map. This image visualizes Interstate 2.0 service
> roles; it does not prove evidence validity, SLA readiness, transit readiness,
> upgrade readiness, asset-condition repair, terminal-access proof,
> construction readiness, ROI, agency approval, or official adoption.

If space is tight:

> ROUTE structural service map; evidence, SLA, upgrade, terminal, asset,
> construction, ROI, and agency-approval claims remain held.

## Recommended Media Visuals

| Use Case | Asset | Safe Use | Required Boundary |
|---|---|---|---|
| Project overview | `maps/all-tiers.png` | Show the national T1-T4 service hierarchy. | Structural map only; evidence/SLA/transit/upgrade claims held. |
| Service hierarchy explainer | `maps/beck-schematic.png` | Explain T1-T4 as a promise network. | Schematic visualization, not proof of service performance. |
| Regional connector explainer | `maps/beck-schematic-t2.png` | Explain T2 as regional service treatment inside the national spine. | T2 shape is not asset, SLA, or upgrade readiness. |
| T2-only explainer | `maps/beck-schematic-t2-only.png` | Isolate the regional layer for a sidebar or explainer. | T2 map is not a funding, construction, or operating claim. |
| Rural/access explainer | `maps/t3-great-lakes.png`, `maps/t3-southeast.png`, `maps/t3-texas-border.png`, `maps/t3-mountain-west.png`, `maps/t3-mid-south.png` | Show feeder/access zones and lower-tier visibility. | Local access and terminal proof remain source-gated. |
| Corridor context | `maps/i5.png`, `maps/i10.png`, `maps/i35.png`, `maps/i40.png`, `maps/i75.png`, `maps/i80.png`, `maps/i90.png`, `maps/i95.png` | Give context for named T1 corridor surfaces. | Corridor maps are context, not construction or ROI recommendations. |
| International portability fixture | `maps/international/canada-service-network.svg`, `maps/international/eu-rhine-alpine-region.svg`, `maps/international/india-logistics-spine.svg`, `maps/international/japan-pacific-belt.svg`, `maps/international/china-logistics-spine.svg` | Show that the same adapter-to-inference-to-map workflow can generate comparable held-claim service-network fixtures across regions. | Replicability fixture only; no official network, country/regional approval, SLA, construction, ROI, compliance, endorsement, validation, or public-readiness claim. |
| Canada source-custody overlay | `maps/international/canada-source-custody-preflight.svg` | Show how a country pilot moves from map fixture to visible source-candidate/source-needed adapter coverage. | Preflight review surface only; no official Canadian network, Transport Canada/provincial/port review, guaranteed SLA, construction, ROI, eligibility, compliance, endorsement, validation, or public-readiness claim. |
| Canada candidate hierarchy | `maps/international/canada-candidate-hierarchy.svg` | Show a concrete T1/T2/T3 candidate hierarchy result for Canada from ROUTE-held rows. | Candidate result only; no official Canadian network, Transport Canada/provincial/port review, guaranteed SLA, construction, ROI, eligibility, compliance, endorsement, validation, or public-readiness claim. |
| Canada candidate hierarchy v2 | `maps/international/canada-candidate-hierarchy-v2.svg` | Show the score-repair-render iteration result for Canada, including tier and coverage repairs. | Candidate iteration only; no official Canadian network, Transport Canada/provincial/port review, guaranteed SLA, construction, ROI, eligibility, compliance, endorsement, validation, or public-readiness claim. |
| EU Rhine-Alpine candidate hierarchy v2 | `maps/international/eu-rhine-alpine-candidate-hierarchy-v2.svg` | Show the score-repair-render iteration result for a second region, including cross-border spine, port connectors, and alternate access candidates. | Candidate iteration only; no official EU corridor designation, member-state approval, guaranteed SLA, construction, ROI, eligibility, compliance, endorsement, validation, or public-readiness claim. |
| India candidate hierarchy v2 | `maps/international/india-candidate-hierarchy-v2.svg` | Show score-repair-render iteration for a third region, including logistics spine repair and port feeder separation. | Candidate iteration only; no official Indian corridor, national/state approval, guaranteed SLA, construction, ROI, eligibility, compliance, endorsement, validation, or public-readiness claim. |

## Do Not Use Visuals To Claim

- The map is an official plan.
- The map selects final routes or construction projects.
- The map proves a service window can be met.
- The map proves terminal, port, warehouse, rural, or local access.
- The map proves asset condition, environmental readiness, or resilience.
- The map proves positive ROI or funding eligibility.
- The map shows agency, state, stakeholder, or port endorsement.
- The map is a public-ready proof product.

## Visual Source Checklist

Before publishing a ROUTE visual, check:

| Check | Required Result |
|---|---|
| Is the asset listed in `data/map-publication-inventory.csv`? | yes |
| Is the asset used for the allowed use in that inventory? | yes or explain narrower use |
| Does the caption include held-claim language? | yes |
| Does the story avoid map-proof wording? | yes |
| Are any stronger claims backed by separate source rows? | yes, otherwise omit |
| Does the image imply endorsement by an agency, state, port, or stakeholder? | no |

## Gate

Decision: **media_visual_assets_with_held_claim_captions**

Rationale: ROUTE visuals are useful for media explanation when captions preserve
their structural status and held claims. This guide does not promote
official-plan, construction, guaranteed-service, numeric ROI, eligibility,
compliance, endorsement, stakeholder-validation, approval, public-readiness,
external-readiness, or publication-ready proof claims.
