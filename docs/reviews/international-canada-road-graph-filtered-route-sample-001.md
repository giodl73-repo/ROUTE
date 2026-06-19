---
name: International Canada Road Graph Filtered Route Sample 001
slug: international-canada-road-graph-filtered-route-sample-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - data/international-canada-parser-extraction-candidates-001.csv
  - tools/sample_canada_road_graph_filtered_routes.py
  - tools/check_canada_road_graph_filtered_route_sample.py
  - data/international-canada-road-graph-filtered-route-sample-001.csv
  - data/international-canada-parser-mapping-ledger-001.csv
  - data/international-canada-source-payload-resolution-001.csv
---

# International Canada Road Graph Filtered Route Sample 001

## Result

This adds a bounded filtered route-identifier sample from the resolved Canada
road-graph ESRI REST layer. The sample queries a five-row object-ID window with
no geometry and records usable route identifiers:

- `rtnumber1` values include `5` and `1`;
- `rtename1` includes `Autoroute Transcanadienne`;
- class values include `Freeway`, `Expressway / Highway`, and `Core`.

This improves the parser intake story: route number/name fields are not merely
present in metadata; they can be sampled from source attributes. The result
still does not validate source rows, accept geometry, replace dry-run fixtures,
promote a parsed adapter, or create official Canadian network, route
designation, Transport Canada/provincial/port approval, guaranteed SLA,
construction, ROI, eligibility, compliance, endorsement, public-readiness,
external-readiness, or external validation claims.

`data/international-canada-parser-extraction-candidates-001.csv` uses this
filtered sample to produce separate no-geometry extraction candidates. Those
rows remain candidate-only and do not replace the dry-run fixture.

## Command Closeout

Run:

```powershell
npm run check:canada:filtered
```

Expected gate result:

```text
Canada road-graph filtered route sample gate: PASS
  checked route identifiers, bounded row count, no-geometry posture, and not-accepted evidence status
```

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Build filtered route sample | `python tools\sample_canada_road_graph_filtered_routes.py` | pass | `data/international-canada-road-graph-filtered-route-sample-001.csv` written |
| Filtered route gate | `python tools\check_canada_road_graph_filtered_route_sample.py` | pass | route identifiers, bounded row count, no-geometry posture, and not-accepted evidence status checked |
| Package command | `npm run check:canada:filtered` | pass | filtered sample build and gate run together |
| Python compile | `python -m py_compile tools\sample_canada_road_graph_filtered_routes.py tools\check_canada_road_graph_filtered_route_sample.py` | pass | scripts compile |
| Claim-boundary scan | scan filtered sample artifacts and edited indexes | pass | hits are blocked, held, or do-not-infer contexts |
| Diff hygiene | `git diff --check` | pass | no whitespace errors |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_filtered_route_sample_passed; parser_extraction_held**

Rationale: ROUTE can obtain usable Canada road-graph route identifiers through
a bounded no-geometry source query. Promotion remains blocked until broader
extraction policy, geometry handling, role review, source-row validation, and
fixture replacement closeout are completed.
