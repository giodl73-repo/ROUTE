---
wave: priority-a-pavement-fetch-attempt
date_open: 2026-05-14
status: done
source: data/tier-pavement-source-access.csv
---

# Priority A Pavement Fetch Attempt

## Mission

Run the governed priority-A pavement source fetch rail and commit only the
reviewable fetch-attempt summary, not raw HPMS cache payloads.

## Opening Rule

This wave may run scoped HPMS fetches for TX, LA, and NM and summarize cache
record counts. It must not commit raw cache files, accept pavement evidence, or
reduce asset-condition debt before review.

## Inputs Inherited

| Input | Source |
|---|---|
| Priority-A pavement source access | `data/tier-pavement-source-access.csv` |
| Source fetch cache policy | `data/source-fetch-policy.csv` |
| Pavement acquisition docket | `data/tier-pavement-acquisition-docket.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Scoped fetch attempt | done | `route fetch-hpms --states TX,LA,NM`; ignored cache files |
| 02 - Attempt summary | done | `data/tier-pavement-source-fetch-attempt.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/fetch-attempt/review.md`; final gates |

## Done Criteria

- Priority-A fetch attempt results are summarized in a committed CSV.
- TX and LA cache record counts are recorded as populated but unreviewed.
- NM parse/fetch failure is recorded as an empty cache outcome.
- Blocker claims remain unchanged with `claim_blocker_delta = 0`.
- Final gates pass before close.

## Non-Goals

- Do not commit `data/cache/` HPMS payloads.
- Do not treat cache population as evidence acceptance.
- Do not reduce pavement evidence or repair debt.

