---
name: International EU Rhine-Alpine Port Node Role Review 001
slug: international-eu-rhine-alpine-port-node-role-review-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_port_node_role_review.py
  - tools/check_eu_rhine_alpine_port_node_role_review.py
  - data/international-eu-rhine-alpine-port-node-role-review-001.csv
  - data/international-eu-rhine-alpine-port-node-record-sample-001.csv
---

# International EU Rhine-Alpine Port Node Role Review 001

## Result

The sampled EU port-node records pass internal role review only with holds.

Roles accept the records as node-candidate planning inputs, not as node fixture
replacement, terminal performance proof, access proof, geometry proof, topology
proof, or validation.

## Gate

Decision: **port_node_role_review_pass_with_holds**

Run:

```powershell
npm run check:eu:port-node-role-review
```
