---
name: International Canada Parser Mapping Ledger 001
slug: international-canada-parser-mapping-ledger-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_parser_mapping_ledger.py
  - tools/check_canada_parser_mapping_ledger.py
  - data/international-canada-road-graph-filtered-route-sample-001.csv
  - data/international-canada-parser-mapping-ledger-001.csv
  - data/international-canada-source-field-inventory-001.csv
  - data/international-canada-road-graph-feature-sample-001.csv
---

# International Canada Parser Mapping Ledger 001

## Result

This adds a Canada parser mapping ledger for `CAN-SRC-001`. The ledger maps
resolved ESRI REST layer fields into the `canada_source_link_candidates` output
contract:

- `roadclass`, `type_code`, and `desc_en` map to `source_class`;
- `rtnumber1` maps to `route_id`;
- `rtename1` maps to `route_name`;
- `Shape` maps only to `geometry_ref` and remains not requested in the sample.

The sampled rows show useful class values (`Expressway / Highway`, `Core`) but
route number/name are `None` in the bounded five-row sample. That is a parser
intake finding, not a failure: fixture replacement still requires broader
feature extraction, filtering, geometry policy, and review.

`data/international-canada-road-graph-filtered-route-sample-001.csv` now closes
that narrow intake question by showing usable route number/name attributes in a
bounded no-geometry route sample. The mapping ledger remains candidate-only.

The ledger does not replace dry-run fixtures, accept geometry, validate source
rows, promote a parsed adapter, or create official Canadian network, route
designation, Transport Canada/provincial/port approval, guaranteed SLA,
construction, ROI, eligibility, compliance, endorsement, public-readiness,
external-readiness, or external validation claims.

## Command Closeout

Run:

```powershell
npm run check:canada:mapping
```

Expected gate result:

```text
Canada parser mapping ledger gate: PASS
  checked target coverage, candidate status, no-geometry posture, and not-accepted evidence status
```

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Build mapping ledger | `python tools\build_canada_parser_mapping_ledger.py` | pass | `data/international-canada-parser-mapping-ledger-001.csv` written |
| Mapping gate | `python tools\check_canada_parser_mapping_ledger.py` | pass | target coverage, candidate status, no-geometry posture, and not-accepted evidence status checked |
| Package command | `npm run check:canada:mapping` | pass | mapping build and gate run together |
| Python compile | `python -m py_compile tools\build_canada_parser_mapping_ledger.py tools\check_canada_parser_mapping_ledger.py` | pass | scripts compile |
| Claim-boundary scan | scan mapping artifacts and edited indexes | pass | hits are blocked, held, or do-not-infer contexts |
| Diff hygiene | `git diff --check` | pass | no whitespace errors |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_parser_mapping_ready; fixture_replacement_held**

Rationale: ROUTE can map the resolved road-graph source fields to the dry-run
output contract, but source-row validation, route filtering, geometry handling,
and role review still block fixture replacement.
