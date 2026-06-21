---
name: International Market-System Map Upgrade 001
slug: international-market-system-map-upgrade-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/international-market-system-map-v1.csv
  - data/international-market-system-map-export-001.csv
  - tools/render_international_market_system_maps.py
  - maps/international/china-market-system-v1.svg
  - maps/international/china-market-system-v1.png
  - maps/international/india-market-system-v1.svg
  - maps/international/india-market-system-v1.png
---

# International Market-System Map Upgrade 001

## Scope

This upgrade replaces sparse country corridor diagrams with richer 2D
market-system presentation maps for China and India.

The maps show trunk promises, regional market connectors, port and terminal
feeders, lateral resilience paths, and held proof gaps. They are client
discovery surfaces, not validated national plans.

## Why The Earlier Maps Were Insufficient

The earlier international maps proved renderer repeatability, but many examples
looked like linear strings. That does not sell ROUTE as a product because a
buyer needs to see how the method handles real market systems: dense gateways,
inland hubs, lateral choices, terminal access, and regional coverage.

## Outputs

| Country | SVG | Intended use |
|---|---|---|
| China | `maps/international/china-market-system-v1.svg` | Presentation-grade market-system discovery surface |
| India | `maps/international/india-market-system-v1.svg` | Presentation-grade market-system discovery surface |

PNG previews are written beside the SVGs and indexed in
`data/international-market-system-map-export-001.csv`.

## Validation

| Check | Result |
|---|---|
| China and India SVGs render from one command. | pass |
| PNG previews exist at 2200 x 1320. | pass |
| Maps show multiple market layers instead of sparse linear corridor strings. | pass |
| Held-claim posture is visible in each map and export manifest. | pass |

## Held Claims

The maps do not claim official network designation, legal SLA, construction
readiness, cost, numeric ROI, funding eligibility, compliance, endorsement,
external validation, public readiness, or country agency approval.

## Gate

Decision: **country_market_system_maps_created; validation_held**
