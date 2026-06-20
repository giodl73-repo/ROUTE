---
name: International Canada Node Fixture Replacement Closeout 001
slug: international-canada-node-fixture-replacement-closeout-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_parser_dry_run.py
  - tools/check_canada_parser_dry_run.py
  - tools/build_canada_node_fixture_replacement_closeout.py
  - tools/check_canada_node_fixture_replacement_closeout.py
  - data/canada_source_node_candidates.csv
  - data/international-canada-node-fixture-replacement-closeout-001.csv
---

# International Canada Node Fixture Replacement Closeout 001

## Result

This closes the narrow internal replacement of
`data/canada_source_node_candidates.csv`.

Allowed language:

- Canada node fixture rows now carry selected public port source-custody
  candidates for internal adapter proof.

Do not say:

- Canada node catalog validation is complete.
- The rows prove terminal performance, road-access adequacy, throughput, SLA,
  ROI, endorsement, public readiness, external readiness, or external
  validation.

Run:

```powershell
npm run check:canada:node-fixture-replacement
```
