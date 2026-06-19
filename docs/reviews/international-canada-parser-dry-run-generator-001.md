---
name: International Canada Parser Dry Run Generator 001
slug: international-canada-parser-dry-run-generator-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_parser_dry_run.py
  - data/international-canada-adapter-source-pack-001.csv
  - data/international-canada-parser-preflight-001.csv
  - data/international-canada-parser-output-contract-001.csv
  - data/canada_source_link_candidates.csv
  - data/canada_source_need_candidates.csv
  - data/canada_source_node_candidates.csv
  - data/canada_service_target_candidates.csv
  - data/canada_adapter_evidence_labels.csv
  - data/canada_adapter_review_backlog.csv
---

# International Canada Parser Dry Run Generator 001

## Result

This adds `tools/build_canada_parser_dry_run.py`, a reproducible generator for
the Canada parser dry-run fixture. The generator reads the Canada source pack,
parser preflight ledger, and output contract, then writes the six contract
tables used by the dry run:

- `data/canada_source_link_candidates.csv`
- `data/canada_source_need_candidates.csv`
- `data/canada_source_node_candidates.csv`
- `data/canada_service_target_candidates.csv`
- `data/canada_adapter_evidence_labels.csv`
- `data/canada_adapter_review_backlog.csv`

The generated rows are still candidate, gap, held, evidence-label, and review
backlog rows. The generator does not fetch or cache Canadian sources, parse
source payloads, replace the Canada hierarchy fixture, promote a parsed adapter,
or create official Canadian network, route designation, Transport
Canada/provincial/port approval, guaranteed SLA, construction, ROI,
eligibility, compliance, endorsement, public-readiness, external-readiness, or
external validation claims.

## Working Definition

For this step, "Canada is working" means the dry-run adapter surface is
reproducible from declared inputs:

- source-pack rows define source family, owner/publisher, access date, and next
  action;
- parser-preflight rows define task IDs, allowed labels, blockers, and claim
  boundaries;
- output-contract rows define required columns, minimum row posture, acceptance
  rules, and blocked values;
- the generator emits deterministic candidate/gap/held tables and matching
  evidence-label rows;
- review backlog rows stay pending before any use beyond internal inspection.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Generator run | `python tools\build_canada_parser_dry_run.py` | pass | six dry-run CSVs written |
| Determinism | inspect `git diff -- data\canada_*.csv` after generator run | pass | generated CSVs match committed fixture rows |
| Claim-boundary scan | scan generator, review, and edited index surfaces | pass | hits are blocked, held, or do-not-infer contexts |
| Diff hygiene | `git diff --check` | pass | no whitespace errors |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_parser_dry_run_generated; source_parser_held**

Rationale: The Canada dry-run fixture is now reproducible machinery instead of
a hand-maintained table set. Promotion remains blocked until source download,
field inventory, source payload parsing, fixture replacement review,
Canada-specific role review, and validation closeout are completed.
