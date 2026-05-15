---
name: Priority A HPMS Scope Broadening Policy Review
slug: priority-a-hpms-scope-broadening-policy
type: review
status: reviewed
rubric_version: v1.0
author: route-wave
created: 2026-05-14
updated: 2026-05-14
sources:
  - data/tier-pavement-hpms-scope-broadening.csv
  - data/tier-pavement-unmatched-join-review.csv
  - data/source-fetch-policy.csv
---

# Priority A HPMS Scope Broadening Policy Review

## Findings

1. The existing priority-A HPMS caches are populated but have zero IRI route
   records for the source-needed US-route members.
2. The broadening plan uses scoped state cache mutation and systems `1,2,3` to
   include principal arterial US-route evidence candidates.
3. Default HPMS fetch behavior remains system `1`; broadening requires an
   explicit `--functional-systems` argument and `--states`.
4. The broadening rows preserve `publication;sla;transit;upgrade` blockers with
   `claim_blocker_delta = 0`.

## Role Decision

The broadening plan is acceptable as source-acquisition policy. It is not
evidence acceptance. The broadened fetch must be run and reviewed separately
before pavement debt can be reduced.

## Required Next Action

Execute the governed broadened HPMS fetch for TX, LA, and NM, rebuild pavement
artifacts, and record a postfetch review before any asset-condition relief
replay.
