---
wave: priority-a-pavement-funding-evidence-source-capture
date_open: 2026-05-14
status: done
source: data/tier-pavement-funding-evidence-source-capture.csv
---

# Priority A Pavement Funding Evidence Source Capture

## Mission

Record source-capture status for priority-A pavement funding evidence before
any artifact attachment, acceptance, or asset-condition relief replay.

## Opening Rule

This wave may add source-capture placeholders and doctrine references. It must
not attach funding evidence, accept funding evidence, reduce blockers, or replay
asset-condition relief.

## Inputs Inherited

| Input | Source |
|---|---|
| Funding evidence contract | `data/tier-pavement-funding-evidence-contract.csv` |
| Evidence contract close | `waves/2026-05-14-priority-a-pavement-funding-evidence-contract/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Source capture | done | `data/tier-pavement-funding-evidence-source-capture.csv`; `CLOSE.md` |

## Done Criteria

- Every held priority-A funding evidence contract row has a source-capture row.
- Captured artifact remains `none`.
- Accepted evidence status remains `not-accepted`.
- Relief eligibility remains `not-eligible-for-relief`.
- Claim blocker delta remains `0`.
- Final gates pass before close.

## Non-Goals

- No funding artifact attachment.
- No funding evidence acceptance.
- No asset-condition relief replay.
- No downgrade, exclusion, or funding mutation.
