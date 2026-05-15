---
wave: priority-a-pavement-funding-evidence-accepted-artifact-acquisition
date_open: 2026-05-15
status: done
source: data/tier-pavement-funding-evidence-accepted-artifact-acquisition.csv
---

# Priority A Pavement Funding Evidence Accepted Artifact Acquisition

## Mission

Turn held accepted-attachment review rows into explicit accepted funding
artifact acquisition/cache targets before any evidence acceptance or relief
replay.

## Opening Rule

This wave may identify source-needed acquisition/cache targets. It must not
cache artifacts, attach artifacts, accept funding evidence, reduce blockers, or
replay asset-condition relief.

## Inputs Inherited

| Input | Source |
|---|---|
| Accepted attachment review | `data/tier-pavement-funding-evidence-accepted-attachment-review.csv` |
| Accepted attachment review close | `waves/2026-05-15-priority-a-pavement-funding-evidence-accepted-attachment-review/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Accepted artifact acquisition targets | done | `data/tier-pavement-funding-evidence-accepted-artifact-acquisition.csv`; `CLOSE.md` |

## Done Criteria

- Every held priority-A accepted-attachment review row has an acquisition row.
- Acquisition status is `source-needed`.
- Cache status is `not-cached`.
- Candidate source owner names the state DOT or accepted programming authority.
- Accepted evidence status remains `not-accepted`.
- Relief eligibility remains `not-eligible-for-relief`.
- Claim blocker delta remains `0`.
- Final gates pass before close.

## Non-Goals

- No artifact caching.
- No funding evidence acceptance.
- No asset-condition relief replay.
- No downgrade, exclusion, or funding mutation.
