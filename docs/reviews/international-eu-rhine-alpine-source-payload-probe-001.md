---
name: International EU Rhine-Alpine Source Payload Probe 001
slug: international-eu-rhine-alpine-source-payload-probe-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/probe_eu_rhine_alpine_source_payloads.py
  - tools/check_eu_rhine_alpine_source_payload_probe.py
  - data/international-eu-rhine-alpine-source-payload-probe-001.csv
  - data/international-eu-rhine-alpine-source-payload-access-001.csv
---

# International EU Rhine-Alpine Source Payload Probe 001

## Result

This records bounded HTTP reachability metadata for EU Rhine-Alpine URL source
candidates.

The probe samples response metadata only. It does not accept source evidence,
parse fields, validate source content, replace fixtures, accept geometry, or
promote official-corridor, member-state approval, SLA, ROI, construction,
validation, public-readiness, external-readiness, or external-validation claims.

## Gate

Decision: **eu_payload_probe_recorded; evidence_not_accepted**

Run:

```powershell
npm run check:eu:payload-probe
```
