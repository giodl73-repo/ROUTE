---
name: International Canada Fixture Replacement Contract 001
slug: international-canada-fixture-replacement-contract-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_fixture_replacement_contract.py
  - tools/check_canada_fixture_replacement_contract.py
  - data/international-canada-fixture-replacement-contract-001.csv
  - data/international-canada-parser-extraction-candidates-001.csv
  - data/international-canada-source-row-validation-001.csv
  - data/international-canada-geometry-policy-001.csv
  - docs/reviews/international-canada-fixture-replacement-role-review-001.md
---

# International Canada Fixture Replacement Contract 001

## Result

This contract resolves the geometry/no-geometry question for the next Canada
fixture step.

Decision: no-geometry source-derived rows may be considered for an internal
parser link-candidate fixture replacement closeout. That permission is narrow:
it applies only to `data/canada_source_link_candidates.csv` as internal
candidate rows. It does not allow map overlays, topology proof, parsed-adapter
promotion, public/external use, official-network claims, route designation,
engineering precision, agency approval, construction readiness, guaranteed
SLA, ROI, eligibility, compliance, endorsement, validation, public readiness,
or external readiness.

Map or adapter surfaces still require a separate geometry intake fixture with
topology QA and role review.

## Command Closeout

Run:

```powershell
npm run check:canada:replacement-contract
```

Expected gate result:

```text
Canada fixture replacement contract gate: PASS
  checked no-geometry link contract, map/adapter exclusion, required inputs, and claim holds
```

## Gate

Decision: **replacement_contract_ready_for_internal_link_fixture_closeout**

Rationale: source-derived no-geometry rows can proceed to a fixture replacement
closeout for internal parser link candidates only. Map, topology, adapter,
official, operational, approval, ROI, public-readiness, and external-readiness
uses remain blocked.
