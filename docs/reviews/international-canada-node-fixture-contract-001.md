---
name: International Canada Node Fixture Contract 001
slug: international-canada-node-fixture-contract-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_node_fixture_contract.py
  - tools/check_canada_node_fixture_contract.py
  - data/international-canada-node-fixture-contract-001.csv
  - data/international-canada-node-source-selection-001.csv
  - data/international-canada-node-source-probe-001.csv
---

# International Canada Node Fixture Contract 001

## Result

This contract allows selected and probed Canada port source rows to move toward
an internal node fixture replacement. It does not validate the node catalog.

Decision: **node_fixture_contract_ready_for_internal_closeout**

Blocked: port endorsement, terminal performance, node completeness, road-access
proof, throughput proof, construction readiness, guaranteed SLA, ROI,
compliance, endorsement, validation, public readiness, and external readiness.

Run:

```powershell
npm run check:canada:node-fixture-contract
```
