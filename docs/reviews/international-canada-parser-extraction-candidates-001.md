---
name: International Canada Parser Extraction Candidates 001
slug: international-canada-parser-extraction-candidates-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_parser_extraction_candidates.py
  - tools/check_canada_parser_extraction_candidates.py
  - data/international-canada-parser-extraction-candidates-001.csv
  - data/international-canada-road-graph-filtered-route-sample-001.csv
  - data/international-canada-parser-output-contract-001.csv
---

# International Canada Parser Extraction Candidates 001

## Result

This adds source-bound Canada parser extraction candidates from the filtered
road-graph sample. The candidate table is separate from the dry-run fixture and
contains five bounded no-geometry link candidates with route/class values:

- route `5`, class `Freeway | Core`;
- route `1`, name `Autoroute Transcanadienne`, class `Expressway / Highway | Core`;
- `geometry_ref` remains `not_requested:*`.

The candidates satisfy the link-candidate output shape, but they are not
promoted. They do not replace `data/canada_source_link_candidates.csv`, accept
geometry, validate source rows, promote a parsed adapter, or create official
Canadian network, route designation, Transport Canada/provincial/port approval,
guaranteed SLA, construction, ROI, eligibility, compliance, endorsement,
public-readiness, external-readiness, or external validation claims.

## Command Closeout

Run:

```powershell
npm run check:canada:extract
```

Expected gate result:

```text
Canada parser extraction candidate gate: PASS
  checked candidate values, no-geometry posture, evidence labels, and not-promoted status
```

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Build extraction candidates | `python tools\build_canada_parser_extraction_candidates.py` | pass | `data/international-canada-parser-extraction-candidates-001.csv` written |
| Extraction candidate gate | `python tools\check_canada_parser_extraction_candidates.py` | pass | candidate values, no-geometry posture, evidence labels, and not-promoted status checked |
| Package command | `npm run check:canada:extract` | pass | extraction build and gate run together |
| Python compile | `python -m py_compile tools\build_canada_parser_extraction_candidates.py tools\check_canada_parser_extraction_candidates.py` | pass | scripts compile |
| Claim-boundary scan | scan extraction artifacts and edited indexes | pass | hits are blocked, held, or do-not-infer contexts |
| Diff hygiene | `git diff --check` | pass | no whitespace errors |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_extraction_candidates_ready; fixture_replacement_held**

Rationale: ROUTE now has source-bound candidate rows derived from live Canada
road-graph attributes, but role review, geometry policy, source-row validation,
and fixture replacement closeout remain required before promotion.
