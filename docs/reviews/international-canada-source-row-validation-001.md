---
name: International Canada Source Row Validation 001
slug: international-canada-source-row-validation-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_source_row_validation.py
  - tools/check_canada_source_row_validation.py
  - data/international-canada-source-row-validation-001.csv
  - data/international-canada-parser-extraction-candidates-001.csv
  - data/international-canada-road-graph-filtered-route-sample-001.csv
---

# International Canada Source Row Validation 001

## Result

This validates the five Canada parser extraction candidates against the five
bounded filtered source sample rows that produced them.

Each candidate row matches its source sample row on:

- object ID;
- route ID;
- route name, including generated `route-number-*` labels when the source name
  field is empty;
- source class;
- source ID;
- no-geometry posture.

This closes candidate source-row matching for the bounded extraction table. It
does not accept geometry, replace the dry-run fixture, promote a parsed adapter,
or validate an official Canadian network, route designation, engineering
precision, agency approval, construction readiness, guaranteed SLA, ROI,
eligibility, compliance, endorsement, public readiness, external readiness, or
external validation claim.

## Command Closeout

Run:

```powershell
npm run check:canada:source-row-validation
```

Expected gate result:

```text
Canada source-row validation gate: PASS
  checked extracted rows against filtered source sample and preserved promotion holds
```

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Build source-row validation | `python tools\build_canada_source_row_validation.py` | pass | `data/international-canada-source-row-validation-001.csv` written |
| Source-row validation gate | `python tools\check_canada_source_row_validation.py` | pass | five candidate rows matched to five filtered sample rows |
| Package command | `npm run check:canada:source-row-validation` | pass | build and gate run together |
| Python compile | `python -m py_compile tools\build_canada_source_row_validation.py tools\check_canada_source_row_validation.py` | pass | scripts compile |
| Claim-boundary scan | scan validation artifacts and edited indexes | pass | hits are blocked, held, or do-not-infer contexts |
| Diff hygiene | `git diff --check` | pass | no whitespace errors |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **candidate_source_rows_validated; fixture_replacement_held**

Rationale: source-row matching is closed for the bounded extraction table, but
geometry policy, fixture replacement closeout, parsed-adapter promotion,
operational posture, authority posture, and external validation remain held.
