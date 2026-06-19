---
name: International Canada Geometry Policy 001
slug: international-canada-geometry-policy-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_geometry_policy.py
  - tools/check_canada_geometry_policy.py
  - data/international-canada-geometry-policy-001.csv
  - data/international-canada-parser-extraction-candidates-001.csv
  - data/international-canada-source-row-validation-001.csv
  - docs/reviews/international-canada-fixture-replacement-role-review-001.md
---

# International Canada Geometry Policy 001

## Result

This policy keeps Canada geometry rejected for the current source-derived
candidate set. The extraction candidates and source-row validation close
attribute matching only; they do not accept geometry, prove topology, support a
map overlay, replace the dry-run fixture, or promote a parsed adapter.

Geometry acceptance requires a separate geometry intake fixture with bounded
fetch, coordinate reference system, license/access note, row-level geometry
join, topology QA, Schematic Cartographer review, and Traffic Engineer
no-operational-claim review.

## Command Closeout

Run:

```powershell
npm run check:canada:geometry-policy
```

Expected gate result:

```text
Canada geometry policy gate: PASS
  checked no-geometry posture, acceptance prerequisites, blocked uses, and claim holds
```

## Gate

Decision: **geometry_rejected_for_current_candidates; fixture_replacement_held**

Rationale: Canada candidate rows can remain useful for parser review, but
geometry, topology, map overlay, fixture replacement, parsed-adapter promotion,
official-network, operational, approval, SLA, ROI, public-readiness, and
external-readiness claims remain blocked.
