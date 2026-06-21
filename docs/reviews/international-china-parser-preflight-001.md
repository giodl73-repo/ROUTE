---
name: International China Parser Preflight 001
slug: international-china-parser-preflight-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - tools/build_china_parser_preflight.py
  - tools/check_china_parser_preflight.py
  - data/international-china-parser-preflight-001.csv
  - data/international-china-parser-output-contract-001.csv
  - data/international-china-source-payload-access-001.csv
---

# International China Parser Preflight 001

## Result

This defines the bounded China parser preflight and output contract after the
source-pack and payload-access gates.

The contract names future no-geometry output tables for link candidates, need
candidates, node candidates, held service targets, evidence labels, and
role-review backlog. Highway standards remain context-only, the hierarchy
fixture remains heuristic-held, and no payload evidence is accepted.

## Boundary

This does not implement a parser, validate source rows, replace fixtures,
accept geometry, prove topology, promote an official Chinese corridor, claim
policy alignment, prove terminal performance, prove SLA or ROI, or claim
public, external, or internal adapter readiness.

## Output Tables

| Table | Label |
| --- | --- |
| `china_source_link_candidates` | context-only or heuristic-held |
| `china_source_need_candidates` | source-candidate |
| `china_source_node_candidates` | source-candidate |
| `china_service_target_candidates` | held |
| `china_adapter_evidence_labels` | carry-forward |
| `china_adapter_review_backlog` | carry-forward |

## Gate

Decision: **china_parser_preflight_ready; implementation_held**

Run:

```powershell
npm run check:china:parser-preflight
```
