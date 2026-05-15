---
wave: priority-a-hpms-broad-fetch-execution
date_open: 2026-05-14
status: done
source: data/tier-pavement-hpms-scope-broadening.csv
---

# Priority A HPMS Broad Fetch Execution

## Mission

Execute the governed broadened HPMS fetch for TX, LA, and NM, rebuild pavement
artifacts, and record whether the priority-A source-needed US-route members
remain source holds or become repair debt.

## Opening Rule

This wave may run `route fetch-hpms --states TX,LA,NM --functional-systems
1,2,3`, rebuild road and pavement artifacts, and update review ledgers. It
must not commit raw `data/cache/` payloads or replay asset-condition relief.

## Inputs Inherited

| Input | Source |
|---|---|
| Scope-broadening plan | `data/tier-pavement-hpms-scope-broadening.csv` |
| Fetch cache policy | `data/source-fetch-policy.csv`; `docs/source-fetch-cache-policy.md` |
| Unmatched join review | `data/tier-pavement-unmatched-join-review.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Broadened fetch execution | done | `route fetch-hpms --states TX,LA,NM --functional-systems 1,2,3`; ignored cache files |
| 02 - Pavement artifact replay | done | `data/tier-pavement-docket.csv`; `data/tier-pavement-source-gaps.csv`; `data/tier-pavement-debt-budget.csv` |
| 03 - Review and close | done | `CLOSE.md`; `panels/broad-fetch/review.md`; final gates |

## Done Criteria

- TX, LA, and NM state caches are populated under the broader functional-system
  scope.
- Pavement docket/source-gap/debt artifacts are regenerated after `route build
  --all-roads`.
- Priority-A unmatched join review shows no remaining source-needed members for
  TX, LA, or NM.
- Remaining priority-A blockers are classified as repair debt, not source debt.
- Final gates pass before close.

## Non-Goals

- Do not commit `data/cache/` HPMS payloads.
- Do not replay asset-condition relief.
- Do not treat repair debt as paid or accepted.
