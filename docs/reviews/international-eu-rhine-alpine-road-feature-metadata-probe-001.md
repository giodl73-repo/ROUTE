---
name: International EU Rhine-Alpine Road Feature Metadata Probe 001
slug: international-eu-rhine-alpine-road-feature-metadata-probe-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_road_feature_metadata_probe.py
  - tools/check_eu_rhine_alpine_road_feature_metadata_probe.py
  - data/international-eu-rhine-alpine-road-feature-metadata-probe-001.csv
  - data/international-eu-rhine-alpine-road-feature-source-selection-001.csv
---

# International EU Rhine-Alpine Road Feature Metadata Probe 001

## Result

EU now has a bounded metadata-probe result after source selection.

The probe distinguishes three facts:

- The GISCO public transport-networks page gives a concrete Ports 2013
  port-node package lead.
- The road-feature path is plausible through Eurostat GISCO Transport version 3
  road-link documentation, but the exact road-link download or API endpoint is
  still not selected.
- The current European Transport Corridors page remains scope context only.

## Boundary

This does not download or accept geometry, validate road rows, replace fixtures,
prove nodes, prove terminal performance, prove SLA or ROI, or promote internal
adapter proof.

## Gate

Decision: **port_node_probe_lead_ready; road_feature_endpoint_still_needed**

Run:

```powershell
npm run check:eu:road-metadata-probe
```
