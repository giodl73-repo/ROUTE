# International India Parser Preflight 001

Status: draft; parser contract only.

## Result

This defines the bounded India parser preflight and output contract after the
field inventory. It names the future no-geometry output tables for link
candidates, need candidates, node candidates, held service targets, evidence
labels, and role-review backlog.

This does not implement a parser, parse source content, validate source rows,
replace fixtures, accept geometry, promote an official Indian corridor, claim
national or state approval, prove SLA or ROI, or claim public/external
readiness.

## Output Tables

| Table | Label |
| --- | --- |
| `india_source_link_candidates` | source-candidate or heuristic-held |
| `india_source_need_candidates` | source-candidate |
| `india_source_node_candidates` | source-candidate |
| `india_service_target_candidates` | held |
| `india_adapter_evidence_labels` | carry-forward |
| `india_adapter_review_backlog` | carry-forward |

## Gate

Decision: **india_parser_preflight_ready; implementation_held**

Run:

```powershell
npm run check:india:parser-preflight
```
