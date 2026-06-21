---
name: International China Parser Dry Run 001
slug: international-china-parser-dry-run-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_china_parser_dry_run.py
  - tools/check_china_parser_dry_run.py
  - data/china_source_link_candidates.csv
  - data/china_source_need_candidates.csv
  - data/china_source_node_candidates.csv
  - data/china_service_target_candidates.csv
  - data/china_adapter_evidence_labels.csv
  - data/china_adapter_review_backlog.csv
---

# International China Parser Dry Run 001

## Result

This emits China parser dry-run tables that match the parser output contract.
Rows are context-only, source-candidate, heuristic-held, held, or carry-forward.
The tables are for internal parser inspection only.

The link table keeps highway standards as context-only and carries forward the
China hierarchy fixture as heuristic-held. Need and node tables contain bounded
context rows only.

## Boundary

This does not implement a parser, parse accepted source rows, validate source
rows, replace fixtures, accept geometry, prove topology, promote an official
Chinese corridor, claim policy alignment, prove terminal performance, prove SLA
or ROI, or claim public, external, or internal adapter readiness.

## Generated Tables

| Table | Status |
| --- | --- |
| `data/china_source_link_candidates.csv` | context-only standards row plus heuristic-held carry-forward |
| `data/china_source_need_candidates.csv` | source-candidate context rows |
| `data/china_source_node_candidates.csv` | source-candidate port/waterway context row |
| `data/china_service_target_candidates.csv` | held target assumption row |
| `data/china_adapter_evidence_labels.csv` | carry-forward evidence labels |
| `data/china_adapter_review_backlog.csv` | pending role-review backlog |

## Gate

Decision: **china_parser_dry_run_ready; fixture_replacement_held**

Run:

```powershell
npm run check:china:parser-dry-run
```
