# International India Parser Dry Run 001

Status: draft; deterministic dry-run fixture only.

## Result

This emits India parser dry-run tables that match the parser output contract.
Rows are source-candidate, heuristic-held, held, or carry-forward. The tables
are for internal parser inspection only.

This does not implement a parser, parse accepted source rows, validate source
rows, replace fixtures, accept geometry, promote an official Indian corridor,
claim national or state approval, prove SLA or ROI, or claim public/external
readiness.

## Generated Tables

| Table | Status |
| --- | --- |
| `data/india_source_link_candidates.csv` | source candidate plus heuristic-held carry-forward |
| `data/india_source_need_candidates.csv` | source-candidate context rows |
| `data/india_source_node_candidates.csv` | source-candidate port-node context row |
| `data/india_service_target_candidates.csv` | held target assumption row |
| `data/india_adapter_evidence_labels.csv` | carry-forward evidence labels |
| `data/india_adapter_review_backlog.csv` | pending role-review backlog |

## Gate

Decision: **india_parser_dry_run_ready; fixture_replacement_held**

Run:

```powershell
npm run check:india:parser-dry-run
```
