---
wave: priority-a-pavement-source-access
date_open: 2026-05-14
status: done
source: data/tier-pavement-acquisition-docket.csv
---

# Priority A Pavement Source Access

## Mission

Classify the priority-A pavement acquisition tasks before any scoped HPMS/state
pavement fetch mutates cache-backed inputs.

## Opening Rule

This wave may create source-access policy rows for priority-A pavement
acquisition tasks. It must not fetch HPMS/state data, rebuild road artifacts,
reduce asset-condition debt, or change selector outputs.

## Inputs Inherited

| Input | Source |
|---|---|
| Pavement acquisition docket | `data/tier-pavement-acquisition-docket.csv` |
| Residual backlog | `data/optimizer-residual-blocker-backlog.csv` |
| Source fetch doctrine | `docs/source-fetch-cache-policy.md`; `data/source-fetch-policy.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Source-access surface | done | `data/tier-pavement-source-access.csv`; CLI gate |
| 02 - Review and close | done | `CLOSE.md`; `panels/source-access/review.md`; final gates |

## Done Criteria

- Every priority-A pavement acquisition docket row has one source-access row.
- Rows require `hpms-scoped-fetch` with `scoped-cache-merge`.
- Rows preserve `publication;sla;transit;upgrade` blockers and carry
  `claim_blocker_delta = 0`.
- Optimizer and release manifests register the source-access artifact.
- Final gates pass before close.

## Non-Goals

- Do not fetch live HPMS/state pavement data.
- Do not mark pavement evidence accepted.
- Do not reduce asset-condition debt.

