---
wave: t2-stitched-member-selection-docket
type: review
status: reviewed
reviewers:
  - optimization-methodologist
  - scope-keeper
  - traffic-engineer
---

# Stitched Member Selection Docket Review

## Finding

Accept the selection docket surface. It prevents state-scoped candidate bundle
rows from becoming selected membership without a separate evidence contract.

## Role Notes

| Role | Finding |
|---|---|
| Optimization Methodologist | Evidence-needed rows preserve the replay chain and avoid silent blocker reduction. |
| Scope Keeper | The wave classifies selection requirements only; it does not select, reject, or mutate candidate, registry, bundle, or game/ops rows. |
| Traffic Engineer | Candidate bundle membership still needs route-family service continuity proof before it can represent a stitched service. |

## Required Follow-Up

A later evidence contract must define sufficient proof for moving a candidate
bundle from evidence-needed to in-scope or rejected.
