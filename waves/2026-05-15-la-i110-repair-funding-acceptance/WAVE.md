---
wave: 2026-05-15-la-i110-repair-funding-acceptance
date_open: 2026-05-15
status: done
---

# LA I-110 Repair Funding Acceptance

## Mission

Accept only source-backed, full-cost pavement repair funding that can remove a
priced T2 asset-condition debt row without weakening the remaining repair holds.

## Opening Rule

No pavement repair debt leaves the optimizer budget unless a governed acceptance
row matches state, tier, route, bundle id, validation status, and funding
coverage at or above the planning repair cost.

## Inputs Inherited

- Residual T2 asset-condition repair debt: LA I-220, CA I-110, and LA I-110;
  3 rows / $55.0M before this wave.
- Louisiana DOTD current STIP project H.010319 for I-110 North St. - Plank Rd.
- Existing pavement route-state exclusion overlay for unsupported route/state
  pairs.

## Pulse Status

| Pulse | Status | Deliverable |
|---|---|---|
| 01 | done | Add accepted LA I-110 full-cost repair funding overlay and replay optimizer debt artifacts. |

## Done Criteria

- `data/tier-pavement-repair-funding-acceptance.csv` records the accepted
  Louisiana DOTD STIP evidence.
- `tier-pavement-debt-budget` excludes only fully funded repair rows and leaves
  unfunded repair debt in review.
- `tier-pavement-repair-debt-review` uses the same acceptance overlay when
  checking residual repair member counts.
- Optimizer ledger, budget, residual backlog, optimizer manifest, and release
  manifest gates pass.

## Non-Goals

- No pavement condition relief replay.
- No downgrade or exclusion for CA I-110 or LA I-220.
- No acceptance of Caltrans I-110 work that does not cover the planning repair
  debt row.
