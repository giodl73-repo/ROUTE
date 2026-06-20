---
name: International Canada Internal Adapter Proof 001
slug: international-canada-internal-adapter-proof-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_internal_adapter_proof.py
  - tools/check_canada_internal_adapter_proof.py
  - data/international-canada-internal-adapter-proof-001.csv
  - data/canada_source_link_candidates.csv
  - data/canada_source_node_candidates.csv
  - data/canada_source_need_candidates.csv
  - data/canada_service_target_candidates.csv
---

# International Canada Internal Adapter Proof 001

## Result

Canada is internally proven as a source-backed, evidence-gated adapter workflow.

This means ROUTE can show a non-U.S. country pilot moving through source
selection, bounded probing, parser-shaped fixtures, replacement closeouts, role
review, target holds, and proof closeout.

It does not mean Canada has an official network, route designation, geometry or
topology proof, map overlay proof, agency or port approval, operating service,
construction readiness, guaranteed SLA, ROI, eligibility, compliance,
endorsement, validation, public readiness, or external readiness.

Decision: **internal_adapter_proof_ready_external_validation_held**

Run:

```powershell
npm run check:canada:internal-proof
```
