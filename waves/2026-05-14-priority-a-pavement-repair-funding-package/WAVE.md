---
wave: priority-a-pavement-repair-funding-package
date_open: 2026-05-14
status: done
source: data/tier-pavement-repair-funding-package.csv
---

# Priority A Pavement Repair Funding Package

## Mission

Package the four priority-A pavement repair rows as an unfunded repair package
without granting asset-condition relief.

## Opening Rule

This wave may add a funding-package artifact and doctrine references. It must
not mark funding committed, accept evidence, downgrade or exclude bundles, or
replay asset-condition relief.

## Inputs Inherited

| Input | Source |
|---|---|
| Repair disposition | `data/tier-pavement-repair-disposition.csv` |
| Disposition close | `waves/2026-05-14-priority-a-pavement-repair-disposition/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Funding package | done | `data/tier-pavement-repair-funding-package.csv`; `CLOSE.md` |

## Done Criteria

- Every priority-A disposition row has a funding-package row.
- Funding package status is `package-required`.
- Funding commitment status is `unfunded`.
- Relief eligibility remains `not-eligible-for-relief`.
- Final gates pass before close.

## Non-Goals

- No accepted funding commitment.
- No downgrade or exclusion decision.
- No asset-condition relief replay.
