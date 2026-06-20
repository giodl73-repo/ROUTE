---
name: International EU Rhine-Alpine Port Node Field Mapping 001
slug: international-eu-rhine-alpine-port-node-field-mapping-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_port_node_field_mapping.py
  - tools/check_eu_rhine_alpine_port_node_field_mapping.py
  - data/international-eu-rhine-alpine-port-node-field-mapping-001.csv
  - docs/reviews/international-eu-rhine-alpine-port-package-manifest-001.md
---

# International EU Rhine-Alpine Port Node Field Mapping 001

## Result

EU now has a node-candidate field mapping plan for GISCO Ports 2013.

The available DBF headers can map to node source ID, node name, jurisdiction
context, source-custody note, geometry reference, and node-class context. This
is a schema-planning step only.

## Boundary

No attribute records are accepted, no geometry is read or accepted, no port
nodes are selected, and no node fixture is replaced.

## Gate

Decision: **port_node_fields_mappable; record_sample_next**

Run:

```powershell
npm run check:eu:port-node-field-mapping
```
