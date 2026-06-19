---
name: International Canada Adapter Promotion Preflight 001
slug: international-canada-adapter-promotion-preflight-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_adapter_promotion_preflight.py
  - tools/check_canada_adapter_promotion_preflight.py
  - data/international-canada-adapter-promotion-preflight-001.csv
  - data/canada_source_link_candidates.csv
  - data/international-canada-link-fixture-replacement-closeout-001.csv
---

# International Canada Adapter Promotion Preflight 001

## Result

This preflight records that Canada has one narrow closed surface:
`data/canada_source_link_candidates.csv` is an internal source-derived
no-geometry link-candidate fixture.

It also records that parsed adapter promotion remains held.

Allowed language:

- The Canada internal parser link fixture is ready for internal parser-contract
  work.
- Parsed Canada adapter promotion is held pending a separate adapter contract,
  geometry/topology disposition, source-backed need/node/target rows, and
  authority/public-use review.

Do not say:

- ROUTE has a parsed Canada adapter.
- The Canada rows prove a map overlay, topology, official network, route
  designation, operating service, construction readiness, guaranteed SLA, ROI,
  eligibility, compliance, agency approval, endorsement, public readiness,
  external readiness, or external validation.

## Command Closeout

Run:

```powershell
npm run check:canada:adapter-promotion
```

Expected gate result:

```text
Canada adapter promotion preflight gate: PASS
  checked surfaces, link-fixture readiness, promotion hold, and claim blocks
```

## Gate

Decision: **internal_link_fixture_ready; parsed_adapter_promotion_held**

Rationale: link-candidate rows have passed the internal replacement closeout,
but geometry, topology, need/node/target source rows, authority review, public
use, external use, and parsed adapter contract gates are not closed.
