---
name: International Map PNG Export 001
slug: international-map-png-export-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/international-map-png-export-001.csv
  - maps/international/canada-service-network.svg
  - maps/international/eu-rhine-alpine-region.svg
  - maps/international/india-logistics-spine.svg
  - maps/international/japan-pacific-belt.svg
  - maps/international/china-logistics-spine.svg
---

# International Map PNG Export 001

## Scope

This export creates PNG previews for the international SVG map set so the maps
can be used in decks, media folders, and quick client review surfaces.

The PNG files are render previews of existing SVG artifacts. They do not promote
any official-network, guaranteed-SLA, construction, ROI, eligibility,
compliance, endorsement, external-validation, or public-readiness claim.

## Output

The export writes PNG files beside their source SVGs under
`maps/international/`.

The manifest is `data/international-map-png-export-001.csv`.

## Validation

| Check | Result |
|---|---|
| All international SVG files rendered to PNG. | pass |
| PNG dimensions match source SVG dimensions. | pass |
| PNG files remain preview artifacts with held claims preserved by source maps and manifest. | pass |

## Gate

Decision: **png_previews_generated; validation_held**
