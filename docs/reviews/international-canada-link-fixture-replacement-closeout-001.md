---
name: International Canada Link Fixture Replacement Closeout 001
slug: international-canada-link-fixture-replacement-closeout-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_parser_dry_run.py
  - tools/check_canada_parser_dry_run.py
  - tools/build_canada_link_fixture_replacement_closeout.py
  - tools/check_canada_link_fixture_replacement_closeout.py
  - data/canada_source_link_candidates.csv
  - data/international-canada-link-fixture-replacement-closeout-001.csv
  - data/international-canada-parser-extraction-candidates-001.csv
  - data/international-canada-source-row-validation-001.csv
  - data/international-canada-fixture-replacement-contract-001.csv
---

# International Canada Link Fixture Replacement Closeout 001

## Result

This closes the narrow internal replacement of
`data/canada_source_link_candidates.csv`.

The Canada dry-run generator now writes the link-candidate fixture from the
validated source-derived extraction candidates instead of the earlier metadata
placeholder rows. The replaced fixture remains a no-geometry internal parser
fixture only.

Allowed language:

- The Canada internal parser link-candidate fixture now uses source-derived
  no-geometry candidate rows.
- Candidate source-row matching, geometry policy, role review, and replacement
  contract gates exist for that narrow replacement.

Do not say:

- ROUTE has a parsed Canada adapter.
- The rows support a map overlay, topology proof, official Canadian network,
  route designation, operating service, construction readiness, guaranteed SLA,
  ROI, eligibility, compliance, agency approval, endorsement, public readiness,
  external readiness, or external validation.

## Command Closeout

Run:

```powershell
npm run check:canada
npm run check:canada:link-fixture-replacement
```

Expected gate result:

```text
Canada parser dry-run gate: PASS
  checked columns, labels, source limits, evidence coverage, and review backlog
Canada link-fixture replacement closeout gate: PASS
  checked internal replacement, no-geometry rows, allowed use, and claim holds
```

## Gate

Decision: **internal_link_fixture_replaced; adapter_and_map_use_held**

Rationale: `data/canada_source_link_candidates.csv` now uses source-derived
candidate rows, but the replacement contract limits use to internal parser
link-candidate fixtures. Geometry, map, topology, adapter, official,
operational, approval, ROI, public-readiness, external-readiness, and external
validation claims remain blocked.
