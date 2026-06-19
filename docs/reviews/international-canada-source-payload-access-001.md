---
name: International Canada Source Payload Access 001
slug: international-canada-source-payload-access-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/probe_canada_source_payloads.py
  - tools/check_canada_source_payload_probe.py
  - tools/build_canada_source_payload_access.py
  - tools/check_canada_source_payload_access.py
  - data/international-canada-source-payload-access-001.csv
  - data/international-canada-adapter-source-pack-001.csv
  - data/international-canada-parser-preflight-001.csv
---

# International Canada Source Payload Access 001

## Result

This adds a Canada source-payload access manifest and gate before any source
payload parsing. The manifest records which Canada source-pack rows are URL
payload candidates, which remain source-needed, and which remain held:

- `CAN-SRC-001`, `CAN-SRC-002`, `CAN-SRC-003`, and `CAN-SRC-004` are
  manual-or-FLETCH cache candidates with Canada-scoped pending cache targets.
- `CAN-SRC-005` remains source-selection-required for port/terminal access.
- `CAN-SRC-SLA-001` remains held with no payload.

The gate verifies source-pack coverage, held live-fetch status, Canada-scoped
cache targets, not-accepted evidence posture, and claim blockers. It does not
fetch source payloads, inspect fields, parse Canadian network data, replace the
dry-run fixture, promote a parsed adapter, or create official Canadian network,
route designation, Transport Canada/provincial/port approval, guaranteed SLA,
construction, ROI, eligibility, compliance, endorsement, public-readiness,
external-readiness, or external validation claims.

`npm run check:canada:probe` may be used after this gate to record live URL
reachability metadata. Probe output remains not accepted as evidence.

## Command Closeout

Run:

```powershell
npm run check:canada
```

Source-payload gate result:

```text
Canada source-payload access gate: PASS
  checked source-pack coverage, held fetch status, cache targets, and claim blockers
```

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Build source-payload access | `python tools\build_canada_source_payload_access.py` | pass | `data/international-canada-source-payload-access-001.csv` written |
| Source-payload gate | `python tools\check_canada_source_payload_access.py` | pass | source-pack coverage, held fetch status, cache targets, and blockers checked |
| Package command | `npm run check:canada` | pass | source-payload and dry-run gates run together |
| Python compile | `python -m py_compile tools\build_canada_source_payload_access.py tools\check_canada_source_payload_access.py` | pass | scripts compile |
| Claim-boundary scan | scan source-payload access artifacts and edited indexes | pass | hits are blocked, held, or do-not-infer contexts |
| Diff hygiene | `git diff --check` | pass | no whitespace errors |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_source_payload_access_ready; source_payload_fetch_held**

Rationale: Canada now has a source-payload access manifest and gate, but the
promotion boundary remains closed until payloads are fetched/cached, field
inventory is written, parser extraction is implemented, and role/validation
closeout passes.
