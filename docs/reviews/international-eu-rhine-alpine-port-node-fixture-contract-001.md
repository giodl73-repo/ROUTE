---
name: International EU Rhine-Alpine Port Node Fixture Contract 001
slug: international-eu-rhine-alpine-port-node-fixture-contract-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_port_node_fixture_contract.py
  - tools/check_eu_rhine_alpine_port_node_fixture_contract.py
  - data/international-eu-rhine-alpine-port-node-fixture-contract-001.csv
  - data/international-eu-rhine-alpine-port-node-source-row-validation-001.csv
---

# International EU Rhine-Alpine Port Node Fixture Contract 001

## Result

EU now has a no-geometry node fixture contract for internal closeout planning.

The contract names Rotterdam, Antwerpen, Genova, Basel, and Duisburg as required
candidate rows, but it does not replace the node fixture by itself.

## Boundary

This is not geometry acceptance, topology proof, map proof, node completeness
proof, terminal performance proof, road-access proof, internal adapter proof,
validation, public readiness, or external readiness.

## Gate

Decision: **port_node_contract_ready_for_closeout; replacement_not_yet_done**

Run:

```powershell
npm run check:eu:port-node-fixture-contract
```
