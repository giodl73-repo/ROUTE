---
wave: priority-a-pavement-unmatched-join-review
date_open: 2026-05-14
status: done
source: data/tier-pavement-source-fetch-review.csv
---

# Priority A Pavement Unmatched Join Review

## Mission

Explain why populated TX/LA/NM HPMS caches still leave priority-A pavement
source gaps open, separating missing source evidence from repair debt before
any asset-condition relief replay.

## Opening Rule

This wave may compare current fetch-review rows, pavement source gaps, pavement
docket rows, and per-state HPMS caches. It must not accept pavement evidence,
replay relief, or reduce publication, SLA, transit, or upgrade blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Fetch review | `data/tier-pavement-source-fetch-review.csv` |
| Pavement source gaps | `data/tier-pavement-source-gaps.csv` |
| Pavement docket | `data/tier-pavement-docket.csv` |
| Per-state HPMS caches | `data/cache/hpms_tx.csv`; `data/cache/hpms_la.csv`; `data/cache/hpms_nm.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Unmatched join review artifact | done | `data/tier-pavement-unmatched-join-review.csv`; CLI gate |
| 02 - Review and close | done | `CLOSE.md`; `panels/unmatched-join/review.md`; final gates |

## Done Criteria

- Every priority-A fetch-review state has one unmatched-join review row.
- Source-needed members are separated from repair-required members.
- HPMS route-record coverage is reported for source-needed routes.
- All rows preserve `publication;sla;transit;upgrade` blockers with
  `claim_blocker_delta = 0`.
- Optimizer and release manifests register the unmatched-join review artifact.
- Final gates pass before close.

## Non-Goals

- Do not commit `data/cache/` HPMS payloads.
- Do not accept HPMS cache rows as pavement evidence.
- Do not reduce pavement evidence or repair debt.
