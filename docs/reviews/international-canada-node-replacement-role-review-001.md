---
name: International Canada Node Replacement Role Review 001
slug: international-canada-node-replacement-role-review-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_node_replacement_role_review.py
  - tools/check_canada_node_replacement_role_review.py
  - data/international-canada-node-replacement-role-review-001.csv
---

# International Canada Node Replacement Role Review 001

## Result

The internal role review passes with holds for replacing Canada node gap rows
with selected source-custody port candidates.

Decision: **pass_with_holds; node_fixture_replacement_allowed_for_internal_use**

Do not say the node catalog is validated, complete, endorsed, public-ready, or
externally reviewed.

Run:

```powershell
npm run check:canada:node-replacement-review
```
