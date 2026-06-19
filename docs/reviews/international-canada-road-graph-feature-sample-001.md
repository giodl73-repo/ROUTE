---
name: International Canada Road Graph Feature Sample 001
slug: international-canada-road-graph-feature-sample-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/sample_canada_road_graph_features.py
  - tools/check_canada_road_graph_feature_sample.py
  - data/international-canada-road-graph-feature-sample-001.csv
  - data/international-canada-source-field-inventory-001.csv
  - data/international-canada-source-payload-resolution-001.csv
---

# International Canada Road Graph Feature Sample 001

## Result

This adds a bounded no-geometry feature sample from the resolved Canada
road-graph ESRI REST layer. The sample records five feature-attribute rows with
object ID, route-number/name fields, road class, type code, and NHS description.

The sample proves ROUTE can execute a bounded road-graph source query for
parser intake. It does not request geometry, cache full payloads, replace
fixtures, promote parsed adapter rows, validate source rows, or create official
Canadian network, route designation, Transport Canada/provincial/port approval,
guaranteed SLA, construction, ROI, eligibility, compliance, endorsement,
public-readiness, external-readiness, or external validation claims.

## Command Closeout

Run:

```powershell
npm run check:canada:sample
```

Expected gate result:

```text
Canada road-graph feature sample gate: PASS
  checked bounded row count, no-geometry posture, and not-accepted evidence status
```

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Build feature sample | `python tools\sample_canada_road_graph_features.py` | pass | `data/international-canada-road-graph-feature-sample-001.csv` written |
| Feature-sample gate | `python tools\check_canada_road_graph_feature_sample.py` | pass | bounded row count, no-geometry posture, and not-accepted evidence status checked |
| Package command | `npm run check:canada:sample` | pass | sample build and gate run together |
| Python compile | `python -m py_compile tools\sample_canada_road_graph_features.py tools\check_canada_road_graph_feature_sample.py` | pass | scripts compile |
| Claim-boundary scan | scan feature-sample artifacts and edited indexes | pass | hits are blocked, held, or do-not-infer contexts |
| Diff hygiene | `git diff --check` | pass | no whitespace errors |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_road_graph_feature_sample_passed; fixture_replacement_held**

Rationale: ROUTE can query bounded road-graph source attributes for Canada, but
feature extraction remains an intake sample until geometry policy, parser
mapping, role review, and validation closeout allow fixture replacement.
