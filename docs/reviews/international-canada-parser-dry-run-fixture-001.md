---
name: International Canada Parser Dry Run Fixture 001
slug: international-canada-parser-dry-run-fixture-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_parser_dry_run.py
  - data/international-canada-parser-output-contract-001.csv
  - docs/reviews/international-canada-parser-output-contract-001.md
  - data/canada_source_link_candidates.csv
  - data/canada_source_need_candidates.csv
  - data/canada_source_node_candidates.csv
  - data/canada_service_target_candidates.csv
  - data/canada_adapter_evidence_labels.csv
  - data/canada_adapter_review_backlog.csv
---

# International Canada Parser Dry Run Fixture 001

## Result

This creates a contract-conforming Canada parser dry-run fixture. The fixture
can be regenerated with `python tools\build_canada_parser_dry_run.py`, which
emits candidate, gap, held, evidence-label, and review-backlog tables using
existing declared source custody. It does not download sources, implement a
source parser, replace fixture links, or promote a parsed Canada adapter.

It does not create official Canadian network, route designation, Transport
Canada/provincial/port approval, guaranteed SLA, construction, ROI,
eligibility, compliance, endorsement, public-readiness, external-readiness, or
external validation claims.

## Fixture Tables

| Output Table | Path | Label Posture |
|---|---|---|
| `canada_source_link_candidates` | `data/canada_source_link_candidates.csv` | parse-ready-candidate |
| `canada_source_need_candidates` | `data/canada_source_need_candidates.csv` | source-candidate |
| `canada_source_node_candidates` | `data/canada_source_node_candidates.csv` | source-needed |
| `canada_service_target_candidates` | `data/canada_service_target_candidates.csv` | held |
| `canada_adapter_evidence_labels` | `data/canada_adapter_evidence_labels.csv` | carry-forward |
| `canada_adapter_review_backlog` | `data/canada_adapter_review_backlog.csv` | carry-forward |

## What This Tests

The dry run tests whether the output contract can be satisfied without
promoting claims:

- candidate link rows are restricted to CAN-SRC-001 and CAN-SRC-003;
- candidate need rows are restricted to CAN-SRC-002 and CAN-SRC-004;
- node and service-target outputs remain gap/held rows;
- every emitted candidate/gap/held row has an evidence-label row;
- role-review backlog exists before any output can be used beyond internal
  parser inspection.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Contract conformance | compare fixture tables against output contract columns, labels, and acceptance rules | pass | dry-run tables use contract columns and required labels |
| Generator run | `python tools\build_canada_parser_dry_run.py` | pass | generator rewrites all six dry-run fixture tables |
| Evidence-label coverage | compare emitted rows to `data/canada_adapter_evidence_labels.csv` | pass | candidate, gap, and held rows have matching evidence-label rows |
| Claim-boundary scan | scan dry-run fixture and edited index surfaces | pass | hits are guardrail, held, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_parser_dry_run_fixture_emitted; parser_implementation_held**

Rationale: The dry-run fixture proves the output contract can be represented as
tables while keeping source download, parser implementation, parsed-adapter,
fixture replacement, official-network, SLA, construction, ROI, compliance,
endorsement, public-readiness, external-readiness, and validation claims held.
