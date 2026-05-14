---
wave: t2-beck-label-density-review
pulse: 02
status: done
---

# Pulse 02 - Label-Density Review Artifact

## Deliverables

- Add `route t2-beck-label-density-review --gate`.
- Emit `data/t2-beck-label-density-review.csv`.
- Register the review artifact in optimizer and release manifests.

## Gates

- The route-level artifact has five rows.
- `claim_blocker_delta` is `0` for every row.
- The artifact preserves five claim blockers and names label policy as the next
  artifact.

## Roles

- Numeracy Checker
- Scope Keeper
