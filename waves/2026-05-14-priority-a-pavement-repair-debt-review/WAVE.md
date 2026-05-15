---
wave: priority-a-pavement-repair-debt-review
date_open: 2026-05-14
status: done
source: data/tier-pavement-repair-debt-review.csv
---

# Priority A Pavement Repair Debt Review

## Mission

Confirm that the remaining priority-A TX, LA, and NM pavement blockers are
repair debt, not source debt, and preserve all SLA, transit, upgrade, and
publication claims before any asset-condition relief replay.

## Opening Rule

This wave may add a gated repair-debt review artifact and doctrine references.
It must not mark repairs paid, accept pavement evidence, mutate `data/cache/`,
or reduce optimizer claim blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Broad fetch closeout | `waves/2026-05-14-priority-a-hpms-broad-fetch-execution/CLOSE.md` |
| Unmatched join review | `data/tier-pavement-unmatched-join-review.csv` |
| Pavement debt budget | `data/tier-pavement-debt-budget.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Repair debt review artifact | done | `data/tier-pavement-repair-debt-review.csv`; `panels/repair-debt/review.md` |

## Done Criteria

- TX, LA, and NM priority-A repair debt rows are represented by bundle and
  state.
- Repair member counts reconcile to `data/tier-pavement-unmatched-join-review.csv`.
- Claim blockers remain unchanged with `claim_blocker_delta = 0`.
- Release/spec indexes name the new artifact.
- Final gates pass before close.

## Non-Goals

- Do not replay asset-condition relief.
- Do not mark pavement repair as funded, complete, or accepted.
- Do not change T1 selection or map publication status.
