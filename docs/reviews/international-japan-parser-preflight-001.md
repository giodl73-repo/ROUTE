# International Japan Parser Preflight 001

Status: draft; parser contract only.

## Result

This defines the bounded Japan parser preflight and output contract after the
field inventory and content-sample gate. It names future no-geometry output
tables for link candidates, need candidates, node candidates, held service
targets, evidence labels, and role-review backlog.

GSI road-feature intake remains source-needed. The contract allows source-needed
link blockers and heuristic-held hierarchy carry-forward rows, but it does not
accept geometry or replace any fixture.

## Boundary

This does not implement a parser, validate source rows, replace fixtures,
accept geometry, prove topology, promote an official Japanese corridor, claim
ministry approval, prove disaster readiness, prove SLA or ROI, or claim public,
external, or internal adapter readiness.

## Output Tables

| Table | Label |
| --- | --- |
| `japan_source_link_candidates` | source-needed or heuristic-held |
| `japan_source_need_candidates` | source-candidate |
| `japan_source_node_candidates` | source-candidate |
| `japan_service_target_candidates` | held |
| `japan_adapter_evidence_labels` | carry-forward |
| `japan_adapter_review_backlog` | carry-forward |

## Gate

Decision: **japan_parser_preflight_ready; implementation_held**

Run:

```powershell
npm run check:japan:parser-preflight
```
