---
wave: t3-lower-tier-feeder-gap-policy
pulse: 02
status: done
---

# Pulse 02 - Feeder Policy Artifact

## Deliverables

- Add `route t3-lower-tier-feeder-gap-policy --gate`.
- Emit `data/t3-lower-tier-feeder-gap-policy.csv`.
- Register the policy artifact in optimizer and release manifests.

## Gates

- The policy artifact has six rows.
- `claim_blocker_delta` is `0` for every row.
- Every row points to policy acceptance as the next artifact.

## Roles

- Numeracy Checker
- Scope Keeper
