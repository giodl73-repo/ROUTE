---
name: International EU Rhine-Alpine Port Package Manifest 001
slug: international-eu-rhine-alpine-port-package-manifest-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_port_package_manifest.py
  - tools/check_eu_rhine_alpine_port_package_manifest.py
  - data/international-eu-rhine-alpine-port-package-manifest-001.csv
  - docs/reviews/international-eu-rhine-alpine-port-package-access-001.md
---

# International EU Rhine-Alpine Port Package Manifest 001

## Result

EU now has package-manifest and DBF-header evidence for the GISCO Ports 2013
node source candidate.

The SHP package contains the `PORT_PT_2013` point-layer files and a DBF header
with `PORT_ID`, `DATA_SRC_C`, and `PORT_COOR_` fields. This is enough to plan a
node-candidate field mapping gate.

## Boundary

The manifest gate reads ZIP manifests and the DBF header only. It does not read
or accept geometry, select node rows, prove terminal performance, prove road
access, replace fixtures, or promote internal adapter proof.

## Gate

Decision: **port_manifest_ready; node_field_mapping_next**

Run:

```powershell
npm run check:eu:port-package-manifest
```
