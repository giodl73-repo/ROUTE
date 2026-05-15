---
wave: priority-a-pavement-funding-evidence-acquisition
date_open: 2026-05-14
status: done
source: data/tier-pavement-funding-evidence-acquisition.csv
---

# Priority A Pavement Funding Evidence Acquisition

## Mission

Turn held priority-A pavement funding evidence review rows into acquisition
targets for accepted full-cost programming or state DOT commitment artifacts.

## Opening Rule

This wave may create acquisition targets for accepted funding artifacts. It must
not attach funding evidence, accept funding evidence, reduce blockers, or replay
asset-condition relief.

## Inputs Inherited

| Input | Source |
|---|---|
| Funding evidence review docket | `data/tier-pavement-funding-evidence-review-docket.csv` |
| Review docket close | `waves/2026-05-14-priority-a-pavement-funding-evidence-review-docket/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Acquisition targets | done | `data/tier-pavement-funding-evidence-acquisition.csv`; `CLOSE.md` |

## Done Criteria

- Every held priority-A funding evidence review row has an acquisition target.
- Required artifact type is accepted full-cost programming or DOT commitment.
- Acquisition status remains `source-needed`.
- Accepted evidence status remains `not-accepted`.
- Relief eligibility remains `not-eligible-for-relief`.
- Claim blocker delta remains `0`.
- Final gates pass before close.

## Non-Goals

- No funding artifact attachment.
- No funding evidence acceptance.
- No asset-condition relief replay.
- No downgrade, exclusion, or funding mutation.
