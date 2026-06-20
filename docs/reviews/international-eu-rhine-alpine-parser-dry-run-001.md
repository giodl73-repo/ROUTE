---
name: International EU Rhine-Alpine Parser Dry Run 001
slug: international-eu-rhine-alpine-parser-dry-run-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_parser_dry_run.py
  - tools/check_eu_rhine_alpine_parser_dry_run.py
  - data/eu_rhine_alpine_source_link_candidates.csv
  - data/eu_rhine_alpine_source_need_candidates.csv
  - data/eu_rhine_alpine_source_node_candidates.csv
  - data/eu_rhine_alpine_service_target_candidates.csv
  - data/eu_rhine_alpine_adapter_evidence_labels.csv
  - data/eu_rhine_alpine_adapter_review_backlog.csv
---

# International EU Rhine-Alpine Parser Dry Run 001

## Result

This creates contract-shaped EU Rhine-Alpine dry-run tables from the declared
source pack and parser preflight.

The dry run emits metadata/no-geometry link candidates, bounded need/context
rows, source-needed node rows, held target rows, evidence labels, and a pending
role-review backlog. It does not parse live source data, accept geometry,
replace hierarchy fixtures, validate an EU corridor, or create member-state
approval, official designation, SLA, ROI, construction, public-readiness,
external-readiness, or external-validation claims.

## Dry-Run Tables

| Table | Current Content | Status |
|---|---|---|
| `eu_rhine_alpine_source_link_candidates.csv` | TENtec and GISCO metadata candidates | source-candidate |
| `eu_rhine_alpine_source_need_candidates.csv` | TEN-T and Rhine-Alpine bounded context rows | source-candidate |
| `eu_rhine_alpine_source_node_candidates.csv` | node/terminal source gap row | source-needed |
| `eu_rhine_alpine_service_target_candidates.csv` | target basis gap row | held |
| `eu_rhine_alpine_adapter_evidence_labels.csv` | matching labels for every emitted row | carry-forward |
| `eu_rhine_alpine_adapter_review_backlog.csv` | pending Scope, Citation, Cartography, and V&V roles | carry-forward |

## Gate

Decision: **eu_parser_dry_run_ready; fixture_replacement_held**

Run:

```powershell
npm run check:eu:parser-dry-run
```

Rationale: EU now has the same parser-contract/dry-run surface shape that
Canada used before source-row validation and fixture replacement. EU remains
short of Canada-level internal proof until payload access, field inventory,
source-row validation, geometry policy, role review, fixture replacement, target
posture, and internal proof close.
