---
wave: priority-a-pavement-downgrade-exclusion-decision
date_open: 2026-05-14
status: done
source: data/tier-pavement-downgrade-exclusion-decision.csv
---

# Priority A Pavement Downgrade Exclusion Decision

## Mission

Decide whether unfunded priority-A pavement repair rows should be downgraded or
excluded before relief. With no accepted funding evidence and no authorization
to remove current-tier service, the conservative decision is no downgrade and
no exclusion.

## Opening Rule

This wave may add a downgrade/exclusion decision artifact and doctrine
references. It must not reduce blockers, grant relief, downgrade service, or
exclude bundles without a separate authorization.

## Inputs Inherited

| Input | Source |
|---|---|
| Funding commitment review | `data/tier-pavement-funding-commitment-review.csv` |
| Commitment review close | `waves/2026-05-14-priority-a-pavement-funding-commitment-review/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Downgrade/exclusion decision | done | `data/tier-pavement-downgrade-exclusion-decision.csv`; `CLOSE.md` |

## Done Criteria

- Every priority-A commitment-review row has a downgrade/exclusion row.
- Downgrade decision is `no-downgrade-selected`.
- Exclusion decision is `no-exclusion-selected`.
- Service status is `held-at-current-tier`.
- Final gates pass before close.

## Non-Goals

- No funding commitment acceptance.
- No asset-condition relief replay.
- No service downgrade or bundle exclusion.
