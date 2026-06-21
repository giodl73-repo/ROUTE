---
name: International China Source Payload Access 001
slug: international-china-source-payload-access-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_china_source_payload_access.py
  - tools/check_china_source_payload_access.py
  - data/international-china-source-payload-access-001.csv
  - data/international-china-adapter-source-pack-001.csv
---

# International China Source Payload Access 001

## Result

China now has a payload-access manifest for the source-pack rows.

The manifest separates five official URL cache candidates from the local
hierarchy fixture and the held service-target row. Evidence remains not
accepted, no live fetch result is claimed, and all rows remain pre-parser.

## Boundary

This is not payload validation, source-row validation, parser promotion,
fixture replacement, parsed-adapter readiness, geometry acceptance, topology
proof, official Chinese corridor designation, policy alignment, terminal
performance, SLA proof, ROI proof, public readiness, external readiness, or
validation.

## Gate

Decision: **china_payload_access_manifest_ready; evidence_not_accepted**

Run:

```powershell
npm run check:china:payload-access
```
