---
name: International Canada Source Payload Probe 001
slug: international-canada-source-payload-probe-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/probe_canada_source_payloads.py
  - tools/check_canada_source_payload_probe.py
  - data/international-canada-source-payload-probe-001.csv
  - data/international-canada-source-payload-resolution-001.csv
  - data/international-canada-source-payload-access-001.csv
  - docs/reviews/international-canada-source-payload-access-001.md
---

# International Canada Source Payload Probe 001

## Result

This adds a bounded live probe for Canada source-payload candidates. The probe
uses `data/international-canada-source-payload-resolution-001.csv` when a source
needs a better access URL than the source-pack landing page, then records HTTP
status, final URL, content type, bytes sampled, not-accepted evidence posture,
blocked claims, and next action. It does not cache full payloads, parse source
fields, promote fixture replacement, or validate Canadian network, approval,
SLA, construction, ROI, compliance, endorsement, public-readiness,
external-readiness, or external validation claims.

Probe result summary:

- `CAN-SRC-001` resolves to the Canada NHS ESRI REST service metadata endpoint
  and returns HTTP 200 for the bounded sampler.
- `CAN-SRC-002`, `CAN-SRC-003`, and `CAN-SRC-004` return HTTP 200 HTML samples
  and remain not accepted as evidence until field inventory and parser review.
- `CAN-SRC-005` remains source-needed.
- `CAN-SRC-SLA-001` remains held.

## Command Closeout

Run:

```powershell
npm run check:canada:probe
```

Expected gate result:

```text
Canada source-payload probe gate: PASS
  checked probe coverage, HTTP metadata posture, and not-accepted evidence status
```

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Probe URL candidates | `python tools\probe_canada_source_payloads.py` | pass | `data/international-canada-source-payload-probe-001.csv` written |
| Probe gate | `python tools\check_canada_source_payload_probe.py` | pass | probe coverage, HTTP metadata posture, and not-accepted evidence status checked |
| Package command | `npm run check:canada:probe` | pass | live probe and gate run together |
| Python compile | `python -m py_compile tools\probe_canada_source_payloads.py tools\check_canada_source_payload_probe.py` | pass | scripts compile |
| Claim-boundary scan | scan probe artifacts and edited indexes | pass | hits are blocked, held, or do-not-infer contexts |
| Diff hygiene | `git diff --check` | pass | no whitespace errors |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_source_payload_probe_passed; field_inventory_held**

Rationale: The probe confirms the road-graph source has a usable metadata
endpoint and the other URL candidates are reachable by bounded HTTP sampling.
The result is useful parser intake evidence, not source validation or adapter
promotion.
