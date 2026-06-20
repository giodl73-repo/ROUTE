---
name: International EU Rhine-Alpine Port Node Source Row Validation 001
slug: international-eu-rhine-alpine-port-node-source-row-validation-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_port_node_source_row_validation.py
  - tools/check_eu_rhine_alpine_port_node_source_row_validation.py
  - data/international-eu-rhine-alpine-port-node-source-row-validation-001.csv
  - data/international-eu-rhine-alpine-port-node-record-sample-001.csv
  - data/international-eu-rhine-alpine-port-node-role-review-001.csv
---

# International EU Rhine-Alpine Port Node Source Row Validation 001

## Result

EU now has source-row validation for bounded port-node attribute records.

The validated rows are candidate attribute rows only. Geometry remains held, and
the output is not node fixture replacement, terminal performance proof, road
access proof, node completeness proof, validation, or public readiness.

## Gate

Decision: **port_node_attribute_rows_validated; geometry_and_replacement_held**

Run:

```powershell
npm run check:eu:port-node-source-row-validation
```
