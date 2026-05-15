---
wave: priority-a-nm-pavement-fetch-repair
date_open: 2026-05-14
status: done
source: data/tier-pavement-source-fetch-review.csv
---

# Priority A NM Pavement Fetch Repair

## Mission

Repair the New Mexico HPMS fetch defect discovered by the priority-A pavement
fetch review and re-run the scoped pavement evidence rail without accepting
evidence or reducing asset-condition blockers.

## Opening Rule

This wave may fix the FHWA hosted-service name used by the HPMS fetcher and
rerun the governed NM fetch, road build, pavement docket, source-gap, debt, and
fetch review gates. It must not commit raw `data/cache/` payloads, accept
pavement evidence, or reduce publication, SLA, transit, or upgrade blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Fetch review decision | `data/tier-pavement-source-fetch-review.csv` |
| Source-access policy | `data/tier-pavement-source-access.csv` |
| Fetch cache policy | `docs/source-fetch-cache-policy.md`; `data/source-fetch-policy.csv` |
| Pavement source gaps | `data/tier-pavement-source-gaps.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - HPMS service-name repair | done | `crates/route-data/src/hpms_fetch.rs`; route-data test |
| 02 - Scoped NM fetch replay | done | `route fetch-hpms --states NM`; ignored cache files |
| 03 - Review and close | done | `CLOSE.md`; `panels/nm-fetch-repair/review.md`; final gates |

## Done Criteria

- NM HPMS service name resolves to the FHWA `NewMexico_2018_PR` hosted service.
- `route fetch-hpms --states NM` populates the NM state cache.
- Pavement source-fetch attempt and review artifacts reclassify NM from
  `fetch-failed-or-empty-cache` to `cache-populated-unreviewed`.
- NM remains `not-accepted` with `claim_blocker_delta = 0` until join/evidence
  review closes the current source gaps.
- Final gates pass before close.

## Non-Goals

- Do not commit raw `data/cache/` HPMS payloads.
- Do not accept NM pavement evidence from cache population alone.
- Do not reduce pavement evidence or repair debt.
