---
name: International India Parser Extraction Candidates 001
slug: international-india-parser-extraction-candidates-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_india_parser_extraction_candidates.py
  - tools/check_india_parser_extraction_candidates.py
  - data/international-india-parser-extraction-candidates-001.csv
  - data/international-india-source-content-sample-001.csv
---

# International India Parser Extraction Candidates 001

## Result

India now has bounded parser extraction candidates from the source-content
sample.

The ledger covers one NHAI authority-context link candidate, three major-port
node candidates, and one Basic Port Statistics publication lead. These rows are
stronger than the earlier dry-run placeholders because they are tied to sampled
source content, but they still do not replace any fixture table.

## Boundary

This is not source-row validation, fixture replacement, parsed-adapter
readiness, geometry acceptance, topology proof, map overlay, official Indian
corridor designation, national approval, state approval, route designation,
terminal performance, node completeness, road-access proof, throughput proof,
SLA proof, ROI proof, construction readiness, public readiness, external
readiness, or internal adapter proof.

## Gate

Decision: **india_extraction_candidates_ready; fixture_replacement_blocked**

Run:

```powershell
npm run check:india:extract
```
