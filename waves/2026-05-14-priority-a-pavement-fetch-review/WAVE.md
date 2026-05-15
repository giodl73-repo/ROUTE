---
wave: priority-a-pavement-fetch-review
date_open: 2026-05-14
status: done
source: data/tier-pavement-source-fetch-attempt.csv
---

# Priority A Pavement Fetch Review

## Mission

Review the priority-A pavement fetch attempts against the current pavement
source-gap surface and decide whether any cache population is sufficient for
asset-condition blocker relief.

## Opening Rule

This wave may compare TX/LA/NM fetch-attempt rows with current source-gap and
acquisition-docket rows. It must not accept pavement evidence, replay relief,
or reduce publication, SLA, transit, or upgrade blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Pavement fetch attempts | `data/tier-pavement-source-fetch-attempt.csv` |
| Pavement acquisition docket | `data/tier-pavement-acquisition-docket.csv` |
| Pavement source gaps | `data/tier-pavement-source-gaps.csv` |
| Pavement debt budget | `data/tier-pavement-debt-budget.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Fetch review artifact | done | `data/tier-pavement-source-fetch-review.csv`; CLI gate |
| 02 - Review and close | done | `CLOSE.md`; `panels/fetch-review/review.md`; final gates |

## Done Criteria

- Every priority-A fetch attempt has one review row.
- TX and LA populated caches are recorded as still-open source gaps, not
  accepted evidence.
- NM is recorded as fetch-repair-needed.
- All rows preserve `publication;sla;transit;upgrade` blockers with
  `claim_blocker_delta = 0`.
- Optimizer and release manifests register the fetch-review artifact.
- Final gates pass before close.

## Non-Goals

- Do not commit `data/cache/` HPMS payloads.
- Do not treat cache population as evidence acceptance.
- Do not reduce pavement evidence or repair debt.
