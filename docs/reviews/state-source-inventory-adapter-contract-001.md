---
name: State Source Inventory Adapter Contract 001
slug: state-source-inventory-adapter-contract-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-source-inventory-adapter-field-contract-001.csv
  - data/state-source-inventory-adapter-row-contract-001.csv
  - data/state-source-inventory-adapter-precheck-001.csv
  - data/state-tierization-fit-role-vector-profile-001.csv
  - docs/reviews/state-tierization-fit-kernel-001.md
---

# State Source Inventory Adapter Contract 001

## Scope

This contract defines the first generic bridge from a state road inventory into
the ROUTE full-state tierization fit kernel. It is designed for state DOT,
turnpike, port, airport, MPO, or consultant source packages.

## Contract Result

| Check | Result |
|---|---|
| Required source fields | 11 |
| Input row surfaces | 5 |
| Precheck gates | 4 |
| Initial ingest posture | source-needed |

## Product Use

The adapter tells a client what data ROUTE needs before it can apply the fitted
T1/T2/T3/T4/R/M/X roles to a real inventory. It also keeps every unsupported
role, SLA, ROI, construction, approval, validation, and full-inventory claim
held until the source package passes review.

## Gate

Decision: **state_source_inventory_adapter_contract_ready_for_client_payload**
