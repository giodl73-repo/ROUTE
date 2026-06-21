# International Japan Parser Dry Run 001

Status: draft; deterministic dry-run fixture only.

## Result

This emits Japan parser dry-run tables that match the parser output contract.
Rows are source-needed, source-candidate, heuristic-held, held, or
carry-forward. The tables are for internal parser inspection only.

The link table deliberately keeps GSI road-feature intake as source-needed and
carries forward the Japan hierarchy fixture as heuristic-held. Need and node
tables contain bounded context rows only.

## Boundary

This does not implement a parser, parse accepted source rows, validate source
rows, replace fixtures, accept geometry, prove topology, promote an official
Japanese corridor, claim ministry approval, prove disaster readiness, prove SLA
or ROI, or claim public, external, or internal adapter readiness.

## Generated Tables

| Table | Status |
| --- | --- |
| `data/japan_source_link_candidates.csv` | source-needed blocker plus heuristic-held carry-forward |
| `data/japan_source_need_candidates.csv` | source-candidate context rows |
| `data/japan_source_node_candidates.csv` | source-candidate port context rows |
| `data/japan_service_target_candidates.csv` | held target assumption row |
| `data/japan_adapter_evidence_labels.csv` | carry-forward evidence labels |
| `data/japan_adapter_review_backlog.csv` | pending role-review backlog |

## Gate

Decision: **japan_parser_dry_run_ready; fixture_replacement_held**

Run:

```powershell
npm run check:japan:parser-dry-run
```
