---
name: International EU Rhine-Alpine Parity Gap 001
slug: international-eu-rhine-alpine-parity-gap-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_parity_gap.py
  - tools/check_eu_rhine_alpine_parity_gap.py
  - data/international-eu-rhine-alpine-parity-gap-001.csv
  - docs/reviews/international-canada-internal-adapter-proof-001.md
  - docs/reviews/international-eu-rhine-alpine-source-row-validation-001.md
---

# International EU Rhine-Alpine Parity Gap 001

## Result

EU Rhine-Alpine is not yet at Canada-level internal proof.

It has reached the source-pack, parser dry-run, payload probe, field inventory,
bounded row-validation, no-geometry node fixture, and target-posture layers. It
has not reached Canada parity for source-derived link fixture replacement or
internal adapter proof.

## Current Parity

| Surface | EU Status |
|---|---|
| Source pack and payload probe | parity reached; evidence not accepted |
| Parser preflight and dry run | parity reached for dry-run layer |
| Field inventory / source-row validation | partial; metadata validation only |
| Link fixture replacement | blocked by missing exact road-link endpoint |
| Node fixture replacement | reached for internal no-geometry node fixture only |
| Target posture / internal proof | target posture reached; internal proof blocked by link fixture |

## Gate

Decision: **eu_not_yet_canada_internal_proof; next_source_content_step_named**

Run:

```powershell
npm run check:eu:parity-gap
```

Rationale: EU has advanced substantially up the same proof ladder, but claiming
Canada-level parity now would be false. The next required step is source-derived
EU link candidates before link fixture replacement and any internal proof.
