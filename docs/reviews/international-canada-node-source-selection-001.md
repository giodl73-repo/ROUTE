---
name: International Canada Node Source Selection 001
slug: international-canada-node-source-selection-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_node_source_selection.py
  - tools/check_canada_node_source_selection.py
  - data/international-canada-node-source-selection-001.csv
  - data/international-canada-adapter-promotion-preflight-001.csv
---

# International Canada Node Source Selection 001

## Result

This selects three public port source-custody candidates for the Canada node
catalog blocker:

- Port of Vancouver
- Port of Montreal
- Port of Halifax

Allowed language:

- ROUTE has selected public port source-custody candidates for a future Canada
  node-catalog fixture.
- The current Canada adapter promotion preflight still holds need/node/target
  table promotion.

Do not say:

- The Canada node catalog has been replaced or validated.
- The selected source rows prove terminal performance, node completeness, road
  access adequacy, throughput, construction readiness, guaranteed SLA, ROI,
  compliance, endorsement, public readiness, external readiness, or external
  validation.

## Command Closeout

Run:

```powershell
npm run check:canada:node-source-selection
```

Expected gate result:

```text
Canada node source-selection gate: PASS
  checked named port sources, source-custody status, promotion hold, and claim blocks
```

## Gate

Decision: **node_sources_selected; node_fixture_replacement_held**

Rationale: the selected rows close a source-selection step for CAN-SRC-005, but
terminal facility inspection, field extraction, node fixture replacement, and
role review remain open before any node catalog promotion.
