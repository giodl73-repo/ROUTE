---
name: International EU Rhine-Alpine Port Node Record Sample 001
slug: international-eu-rhine-alpine-port-node-record-sample-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_port_node_record_sample.py
  - tools/check_eu_rhine_alpine_port_node_record_sample.py
  - data/international-eu-rhine-alpine-port-node-record-sample-001.csv
  - docs/reviews/international-eu-rhine-alpine-port-node-field-mapping-001.md
---

# International EU Rhine-Alpine Port Node Record Sample 001

## Result

EU now has bounded GISCO Ports 2013 attribute records for Rhine-Alpine-relevant
anchor nodes: Rotterdam, Antwerpen, Genova, Basel, and Duisburg.

Each row joins to a point-layer record by `PORT_ID`, but geometry is not read or
accepted.

## Boundary

This is an attribute sample only. It does not prove node completeness, terminal
performance, road access, throughput, geometry, map topology, fixture
replacement, official corridor status, SLA, ROI, validation, public readiness,
or external readiness.

## Gate

Decision: **port_node_records_sampled; node_replacement_still_held**

Run:

```powershell
npm run check:eu:port-node-record-sample
```
