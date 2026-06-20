---
name: International Canada Media Proof Card 001
slug: international-canada-media-proof-card-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - docs/media/canada-internal-proof-brief.md
  - data/international-canada-media-proof-card-001.csv
  - tools/build_canada_media_proof_card.py
  - tools/check_canada_media_proof_card.py
---

# International Canada Media Proof Card 001

## Result

This adds a media-safe Canada proof card and brief.

Allowed language:

- Canada is internally proven as a source-backed evidence-gated ROUTE adapter
  workflow.

Do not say:

- Canada is externally validated, official, approved, public-ready, or
  operationally guaranteed.

Run:

```powershell
npm run check:canada:media-proof
```

Decision: **canada_media_proof_card_ready_external_validation_held**
