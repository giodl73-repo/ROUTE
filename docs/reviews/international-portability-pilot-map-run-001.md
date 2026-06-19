---
name: International Portability Pilot Map Run 001
slug: international-portability-pilot-map-run-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/reports/international-network-inference-portability-report.md
  - data/international-portability-pilot-nodes.csv
  - data/international-portability-pilot-links.csv
  - data/international-portability-pilot-inference.csv
  - data/international-portability-pilot-map-index.csv
  - tools/render_international_pilot_maps.py
  - maps/international/canada-service-network.svg
  - maps/international/eu-rhine-alpine-region.svg
  - maps/international/india-logistics-spine.svg
  - maps/international/japan-pacific-belt.svg
  - maps/international/china-logistics-spine.svg
  - docs/reports/route-evidence-posture.md
  - docs/reports/maps-are-not-proof-report.md
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
  - docs/vtrace/VERIFICATION.md
---

# International Portability Pilot Map Run 001

## Scope

This run tests whether ROUTE's service-network inference pattern can be
repeated across multiple countries or regions using the same adapter-shaped
inputs, role inference rules, evidence labels, and held-claim map captions.

It is not an official network, foreign agency review, international validation,
construction plan, guaranteed SLA, numeric ROI, eligibility finding, compliance
finding, endorsement, public-readiness gate, or external-readiness gate.

## Run Command

```powershell
python tools\render_international_pilot_maps.py
```

The command reads:

- `data/international-portability-pilot-nodes.csv`
- `data/international-portability-pilot-links.csv`

The command writes:

- `data/international-portability-pilot-inference.csv`
- `maps/international/canada-service-network.svg`
- `maps/international/eu-rhine-alpine-region.svg`
- `maps/international/india-logistics-spine.svg`
- `maps/international/japan-pacific-belt.svg`
- `maps/international/china-logistics-spine.svg`

## Pilot Set

| Pilot | Why Selected | Output Map | Current Posture |
|---|---|---|---|
| Canada service network | Tests long-distance freight, ports, prairie spine, Great Lakes/St. Lawrence, and Atlantic resilience access. | `maps/international/canada-service-network.svg` | replicability fixture generated; validation held |
| EU Rhine-Alpine region | Tests port clusters, cross-border corridors, dense inland hubs, Alpine gateway, and multi-jurisdiction constraints. | `maps/international/eu-rhine-alpine-region.svg` | replicability fixture generated; validation held |
| India logistics spine | Tests national/industrial spine, port hinterland, metro relief, and west/south market access. | `maps/international/india-logistics-spine.svg` | replicability fixture generated; validation held |
| Japan Pacific Belt | Tests dense metro reliability, port cluster access, western Honshu/Kitakyushu access, and Tohoku resilience. | `maps/international/japan-pacific-belt.svg` | replicability fixture generated; validation held |
| China logistics spine | Tests coastal manufacturing spine, Yangtze inland access, capital-port terminal access, and Pearl River export gateway. | `maps/international/china-logistics-spine.svg` | replicability fixture generated; validation held |

## Inference Rule

The first-pass rule is deliberately simple:

| Input Pattern | Inferred Role |
|---|---|
| `terminal_connector`, target at or below 1 hour, or port access need | T4 candidate terminal/local |
| `national_spine` with target at or below 48 hours | T1 candidate spine |
| `regional_connector` with target at or below 36 hours | T2 candidate connector |
| everything else | T3 candidate access |

This proves repeatability of the adapter and map pipeline. It does not prove
that the rule is sufficient for country-specific planning, policy, engineering,
or operations.

## Output Inspection

| Check | Result | Evidence |
|---|---|---|
| Five pilots render from one command. | pass | all five SVG files exist under `maps/international/` |
| Inference table records role, evidence label, source status, and boundary. | pass | `data/international-portability-pilot-inference.csv` |
| Map index links each map to inputs, command, posture, and held claims. | pass | `data/international-portability-pilot-map-index.csv` |
| Every generated map contains a held-claim caption. | pass | SVG footer blocks official-plan, construction, guaranteed-SLA, ROI, eligibility, compliance, endorsement, and external-validation claims |
| All rows preserve source-needed posture. | pass | pilot node/link rows are `heuristic-held` or `source-needed` |

## What This Proves

| Claim | Status |
|---|---|
| ROUTE can run the same adapter-shaped input schema across multiple jurisdictions. | supported by fixture |
| ROUTE can infer candidate service roles from declared road class, need class, and service target fields. | supported by fixture |
| ROUTE can generate comparable held-claim service-network maps for different regions. | supported by fixture |
| ROUTE can preserve evidence labels and blocked claims on the generated outputs. | supported by fixture |

## What This Does Not Prove

| Non-Claim | Reason |
|---|---|
| The Canada, EU, India, Japan, or China maps are official or correct networks. | No local source custody, agency review, or validation exists. |
| The service targets are guaranteed SLAs. | They are planning assumptions in fixture rows. |
| The inferred roles are construction, funding, or policy priorities. | The maps are review surfaces only. |
| The pilot uses complete or authoritative local road data. | The current inputs are compact adapter fixtures. |
| The renderer is ready for publication. | Public readiness requires separate source, map, role, and validation closeout. |

## Repairs / Next Work

1. Promote one pilot to a source-backed adapter by adding local road graph,
   node-catalog, demand/need, constraint, and source-custody rows.
2. Add country-specific role review lanes before stronger claims are made.
3. Keep all official-network, guaranteed-SLA, construction, ROI, eligibility,
   compliance, endorsement, external-validation, public-readiness, and
   external-readiness claims held.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Renderer run | `python tools\render_international_pilot_maps.py` | pass | five SVG maps and inference table generated |
| PNG preview conversion | `npx playwright screenshot` for each generated SVG | blocked | local Chromium binary for Playwright is missing; SVG maps remain the committed artifacts |
| Output inspection | compare nodes, links, inference, map index, and SVG captions | pass | five SVG maps exist; inference table and map index link inputs, command, posture, and held claims |
| Prohibited-claim scan | scan run record, data rows, generated maps, and linked edited surfaces for promoted prohibited claims | pass | hits are guardrail, held, do-not-infer, or non-approved contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **international_pilot_maps_generated; validation_held**

Rationale: The pilot demonstrates a repeatable adapter-to-inference-to-map
workflow across five jurisdictions. It does not validate any country or region
network and does not promote official-plan, construction, guaranteed-SLA, ROI,
eligibility, compliance, endorsement, public-readiness, or external-readiness
claims.
