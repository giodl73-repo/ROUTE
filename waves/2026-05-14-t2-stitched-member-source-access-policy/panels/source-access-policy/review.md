---
wave: t2-stitched-member-source-access-policy
type: review
status: reviewed
reviewers:
  - optimization-methodologist
  - scope-keeper
  - traffic-engineer
---

# Stitched Member Source Access Policy Review

## Finding

Accept the source-access policy surface. It names manual/cached access rules for
DOT route-geometry targets without enabling live fetches or treating a source
target as collected evidence.

## Role Notes

| Role | Finding |
|---|---|
| Optimization Methodologist | Manual/cached source-needed policy preserves the staged replay chain and avoids source laundering. |
| Scope Keeper | The wave classifies access only; it does not fetch, cache, accept proof, select candidates, reject candidates, or mutate membership. |
| Traffic Engineer | Route logs, GIS centerlines, and official route descriptions need explicit metadata before they can support continuity claims. |

## Required Follow-Up

A later proof-intake docket must define the artifact fields for manual/cached
route-geometry evidence before collection starts.
