---
name: Priority A Pavement Unmatched Join Review
slug: priority-a-pavement-unmatched-join-review
type: review
status: reviewed
rubric_version: v1.0
author: route-wave
created: 2026-05-14
updated: 2026-05-14
sources:
  - data/tier-pavement-unmatched-join-review.csv
  - data/tier-pavement-source-fetch-review.csv
  - data/tier-pavement-source-gaps.csv
---

# Priority A Pavement Unmatched Join Review

## Findings

1. TX, LA, and NM have populated HPMS caches, so the remaining source-needed
   issue is not cache absence.
2. The current source-needed priority-A members are US-route bundles. The
   current per-state HPMS cache scope has zero IRI route records for those
   source-needed routes.
3. Repair-required interstate members are already evidence-classified as repair
   debt and should not be routed through source acquisition.
4. The review artifact preserves `publication;sla;transit;upgrade` blockers
   with `claim_blocker_delta = 0`.

## Role Decision

Do not replay pavement relief. Cache population plus absent route records for
source-needed US-route members is not accepted pavement evidence. The correct
next action is state DOT pavement condition evidence attachment or a broader
HPMS fetch/join contract that covers the affected US routes.

## Required Next Action

Create a state DOT pavement evidence attachment wave or broaden the HPMS fetch
scope for TX/LA/NM source-needed US-route members, then review evidence before
any asset-condition debt relief.
