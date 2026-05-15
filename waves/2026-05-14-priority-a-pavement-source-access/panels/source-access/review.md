---
wave: priority-a-pavement-source-access
type: review
status: reviewed
rubric_version: v1.0
author: route-wave
created: 2026-05-14
updated: 2026-05-14
sources:
  - data/tier-pavement-acquisition-docket.csv
  - data/tier-pavement-source-access.csv
  - data/source-fetch-policy.csv
---

# Source-Access Review - Priority A Pavement

## Findings

1. Priority-A pavement work touches TX, LA, and NM, the broadest
   asset-condition evidence/debt acquisition states in the docket.
2. Scoped HPMS fetches must use the source-fetch cache doctrine: replace only
   requested state rows and preserve unrelated cache rows.
3. Source access is not proof acceptance. The blocker claims remain unchanged
   until pavement rows are fetched, rebuilt, reviewed, and replayed.

## Verdict

The source-access policy is accepted. It creates a safe execution rail for the
first pavement fetches without reducing debt prematurely.

