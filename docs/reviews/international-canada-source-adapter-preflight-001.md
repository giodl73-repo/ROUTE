---
name: International Canada Source Adapter Preflight 001
slug: international-canada-source-adapter-preflight-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/reports/international-network-inference-portability-report.md
  - docs/reviews/international-portability-pilot-map-run-001.md
  - data/international-canada-source-custody-preflight.csv
  - data/international-canada-adapter-coverage-preflight.csv
  - data/international-portability-pilot-nodes.csv
  - data/international-portability-pilot-inference.csv
  - tools/render_canada_source_preflight_map.py
  - maps/international/canada-source-custody-preflight.svg
  - https://app.geo.ca/en-ca/map-browser/record/c5c249c4-dea6-40a6-8fae-188a42030908
  - https://tc.canada.ca/en/corporate-services/transparency/corporate-management-reporting/transportation-canada-annual-reports/2020-2021/canada-s-road-system
  - https://www12.statcan.gc.ca/census-recensement/2021/geo/sip-pis/rnf-frr/index-eng.cfm
  - https://tc.canada.ca/en/corporate-services/transparency/briefing-documents-transport-canada/2023/current-topics/transport-canada-s-national-trade-corridors-fund-ntcf
---

# International Canada Source Adapter Preflight 001

## Purpose

This preflight promotes the Canada pilot from a compact map fixture toward a
source-backed adapter workflow by naming candidate public sources, adapter
coverage gaps, and the next evidence steps required before a stronger Canada
claim can be made.

It is not an official Canadian network, Transport Canada review, provincial
review, port review, foreign agency review, international validation,
construction plan, guaranteed SLA, numeric ROI, eligibility finding, compliance
finding, endorsement, public-readiness gate, or external-readiness gate.

## Source Custody Preflight

| Source ID | Adapter Field | Candidate Use | Current Status | Claim Boundary |
|---|---|---|---|---|
| CAN-SRC-001 | road graph | NHS route geometry and classification candidate | source-candidate | does not prove ROUTE role assignment or official ROUTE network |
| CAN-SRC-002 | road context | NHS classes and road-system freight/passenger framing | source-candidate | does not authorize service target or SLA claim |
| CAN-SRC-003 | base roads | road-network geometry/name candidate | source-candidate | not engineering, emergency, surveying, or legal precision proof |
| CAN-SRC-004 | trade need | trade-corridor need vocabulary for flow, resilience, and bottleneck rows | source-candidate | does not create funding eligibility, ROI, or project priority |
| CAN-SRC-005 | port node source pack | port-specific node and terminal access source custody | source-needed | does not prove port endorsement, throughput, road access, or terminal service |

## Adapter Coverage Result

| Adapter Field | Result | Meaning |
|---|---|---|
| jurisdiction scope | preflight-ready | Canada can be scoped as a bounded adapter packet, with no approval claim. |
| road graph | source-candidate-found | A public NHS dataset candidate exists, but field mapping and download inspection remain open. |
| node catalog | source-needed | Current city/port centroids are still fixture rows until node-specific sources are attached. |
| need surfaces | source-candidate-found | Road-system and trade-corridor sources can seed broad need vocabulary only. |
| service target set | held | 48h/36h/12h/1h targets remain planning assumptions. |
| constraint ledger | source-needed | Climate, northern access, border, port, and bottleneck constraints need their own rows. |
| evidence labels | preflight-ready | Existing labels can travel through source-backed adapter work. |
| review roles | preflight-ready | Internal roles can pressure-test the adapter, but they do not replace external review. |

## Map Output

`maps/international/canada-source-custody-preflight.svg` overlays the Canada
pilot with adapter-source coverage:

- green: preflight-ready adapter field;
- amber: candidate source found but not bound into the adapter;
- red: source-needed;
- gray: held planning assumption.

The overlay is a review surface. It does not prove the Canada network, the
service targets, public readiness, or any agency acceptance.

## Requirement-To-Refinement Rows

| Row ID | Requirement | Current Evidence | Refinement Applied | Remaining Hold |
|---|---|---|---|---|
| CAN-REQ-001 | Road graph rows need local source custody before service roles can be promoted. | Canada pilot links are `source-needed`. | Added NHS and road-network source candidates and a coverage row. | field mapping and route extraction remain open |
| CAN-REQ-002 | Need surfaces need source owners and bounded vocabulary. | Need classes are heuristic fixture labels. | Added road-system and trade-corridor source candidates. | no freight volume, bottleneck proof, ROI, or project priority |
| CAN-REQ-003 | Node catalog needs source-backed ports/hubs rather than fixture centroids. | Canada pilot nodes are centroid fixtures. | Added explicit source-needed row for port and terminal node custody. | no port authority, terminal access, or throughput claim |
| CAN-REQ-004 | Service targets must remain assumptions until adopted by a scoped source. | 48h/36h/12h/1h values exist in fixture links. | Added held target coverage row. | no guaranteed SLA, travel-time proof, or delivery commitment |

## Role Review

| Role Lane | Result | Finding / Hold |
|---|---|---|
| Scope Keeper | pass | The preflight says candidate adapter, not official Canadian network. |
| Citation Auditor | pass_with_risk | Public source candidates are named with URLs and owners; source extraction and field-level custody are not yet closed. |
| Numeracy Checker | pass | No numeric ROI, cost, benefit, volume, or service-performance claim is promoted. |
| Schematic Cartographer | pass_with_risk | The overlay makes source gaps visible, but remains a non-proof map. |
| Freight / logistics reviewer | pass_with_risk | Trade-corridor language can seed needs, but no freight performance, bottleneck, or market-benefit proof is claimed. |
| Local / rural access reviewer | hold | Northern, rural, and remote access need dedicated source rows before claims. |
| Technical reviewer | pass_with_risk | Renderer is reproducible, but source adapters are not yet parsed or integrated into graph inference. |

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Renderer run | `python tools\render_canada_source_preflight_map.py` | pass | source-custody overlay SVG generated |
| Output inspection | compare source custody, coverage matrix, and SVG overlay | pass | source IDs, coverage results, blocked claims, and map legend align |
| Prohibited-claim scan | scan preflight, source rows, coverage rows, and generated map for promoted prohibited claims | pass | hits are guardrail, held, source-needed, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_source_adapter_preflight_ready; validation_held**

Rationale: Canada is now the first international pilot with a named
source-custody preflight and adapter coverage overlay. The work demonstrates
the next repeatable step after maps: source candidates and source gaps become
visible adapter rows. It does not validate the Canada network or promote any
official-plan, guaranteed-SLA, construction, ROI, eligibility, compliance,
endorsement, public-readiness, or external-readiness claim.
