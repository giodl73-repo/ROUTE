---
name: Priority A NM Pavement Fetch Repair Review
slug: priority-a-nm-pavement-fetch-repair
type: review
status: reviewed
rubric_version: v1.0
author: route-wave
created: 2026-05-14
updated: 2026-05-14
sources:
  - crates/route-data/src/hpms_fetch.rs
  - data/tier-pavement-source-fetch-attempt.csv
  - data/tier-pavement-source-fetch-review.csv
---

# Priority A NM Pavement Fetch Repair Review

## Findings

1. The NM fetch failure was caused by a source service-name defect. FHWA hosted
   services for multi-word states use concatenated names such as `NewMexico`,
   not underscore names such as `New_Mexico`.
2. The repaired scoped fetch produced 12,020 NM HPMS cache records.
3. Cache population did not close the pavement source gap. NM still has 23
   unresolved priority-A members and remains `not-accepted`.
4. The review artifact preserves `publication;sla;transit;upgrade` blockers
   with `claim_blocker_delta = 0`.

## Role Decision

The source-access blocker is repaired, but the evidence blocker is not. NM may
move from fetch repair to unmatched-join or alternate-source review. It may not
move to relief replay until a reviewable pavement evidence artifact explains
which member segments have accepted IRI or condition evidence.

## Required Next Action

Create a TX/LA/NM unmatched-join review or attach state DOT pavement evidence
for the unresolved priority-A members before any asset-condition debt relief.
