---
wave: priority-a-pavement-repair-disposition
date_open: 2026-05-14
status: done
source: data/tier-pavement-repair-disposition.csv
---

# Priority A Pavement Repair Disposition

## Mission

Turn the priority-A repair-debt review into an explicit disposition: TX I220,
LA I220, NM I110, and LA I110 require repair funding and are not eligible for
asset-condition relief.

## Opening Rule

This wave may add a disposition artifact and doctrine references. It must not
reduce blockers, mark repair funding complete, downgrade a bundle, exclude a
bundle, or replay asset-condition relief.

## Inputs Inherited

| Input | Source |
|---|---|
| Repair debt review | `data/tier-pavement-repair-debt-review.csv` |
| Repair review close | `waves/2026-05-14-priority-a-pavement-repair-debt-review/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Repair disposition | done | `data/tier-pavement-repair-disposition.csv`; `CLOSE.md` |

## Done Criteria

- Every priority-A repair review row has a disposition row.
- Disposition is `repair-funding-required`.
- Relief eligibility is `not-eligible-for-relief`.
- `claim_blocker_delta = 0` for every row.
- Final gates pass before close.

## Non-Goals

- No asset-condition relief replay.
- No funding package acceptance.
- No downgrade or exclusion mutation.
