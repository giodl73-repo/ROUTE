---
name: International Japan Parser Extraction Candidates 001
slug: international-japan-parser-extraction-candidates-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_japan_parser_extraction_candidates.py
  - tools/check_japan_parser_extraction_candidates.py
  - data/international-japan-parser-extraction-candidates-001.csv
  - data/international-japan-source-content-sample-001.csv
  - data/international-japan-parser-output-contract-001.csv
---

# International Japan Parser Extraction Candidates 001

## Result

Japan now has bounded parser extraction candidates from the source-content
sample and parser output contract.

The ledger covers one source-needed GSI road-feature blocker, three need/context
candidates, and two port-node context candidates. These rows are stronger than
the dry-run placeholders because they are tied to sampled source content, but
they still do not validate source rows or replace fixtures.

## Boundary

This is not source-row validation, fixture replacement, parsed-adapter
readiness, geometry acceptance, topology proof, map overlay, official Japanese
corridor designation, ministry approval, route designation, disaster readiness,
terminal performance, node completeness, road-access proof, throughput proof,
SLA proof, ROI proof, construction readiness, public readiness, external
readiness, or internal adapter proof.

## Gate

Decision: **japan_extraction_candidates_ready; link_source_needed**

Run:

```powershell
npm run check:japan:extract
```
