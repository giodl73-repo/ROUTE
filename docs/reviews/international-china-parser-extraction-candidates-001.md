---
name: International China Parser Extraction Candidates 001
slug: international-china-parser-extraction-candidates-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_china_parser_extraction_candidates.py
  - tools/check_china_parser_extraction_candidates.py
  - data/international-china-parser-extraction-candidates-001.csv
  - data/international-china-source-content-sample-001.csv
  - data/international-china-parser-output-contract-001.csv
---

# International China Parser Extraction Candidates 001

## Result

China now has bounded parser extraction candidates from the source-content
sample.

The candidate ledger records one standards-context link candidate, three
need/context candidates, and one port/waterway node-context candidate. The
standards row remains context-only. The other rows remain source candidates.
No row is promoted.

## Boundary

This is not source-row validation, fixture replacement, parsed-adapter
readiness, geometry acceptance, topology proof, map overlay, official Chinese
corridor designation, policy alignment, route designation, terminal
performance, node completeness, road-access proof, throughput proof, SLA proof,
ROI proof, construction readiness, public readiness, external readiness, or
internal adapter proof.

## Gate

Decision: **china_extraction_candidates_ready; source_row_validation_blocked**

Run:

```powershell
npm run check:china:extract
```
