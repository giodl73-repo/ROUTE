---
wave: priority-a-pavement-funding-evidence-source-access
date_open: 2026-05-14
status: done
source: data/tier-pavement-funding-evidence-source-access.csv
---

# Priority A Pavement Funding Evidence Source Access

## Mission

Classify source access for priority-A pavement funding evidence acquisition
targets before any artifact collection, attachment, acceptance, or relief replay.

## Opening Rule

This wave may classify accepted funding artifact acquisition as manual or cached
source-needed. It must not attach funding evidence, accept funding evidence,
reduce blockers, or replay asset-condition relief.

## Inputs Inherited

| Input | Source |
|---|---|
| Funding evidence acquisition | `data/tier-pavement-funding-evidence-acquisition.csv` |
| Acquisition close | `waves/2026-05-14-priority-a-pavement-funding-evidence-acquisition/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Source access | done | `data/tier-pavement-funding-evidence-source-access.csv`; `CLOSE.md` |

## Done Criteria

- Every held priority-A funding evidence acquisition row has a source-access row.
- Access mode is `manual-or-cached-source-needed`.
- Live fetch status remains unsupported for funding commitments.
- Evidence artifact remains `source-needed`.
- Accepted evidence status remains `not-accepted`.
- Relief eligibility remains `not-eligible-for-relief`.
- Claim blocker delta remains `0`.
- Final gates pass before close.

## Non-Goals

- No funding artifact collection.
- No funding evidence acceptance.
- No asset-condition relief replay.
- No downgrade, exclusion, or funding mutation.
