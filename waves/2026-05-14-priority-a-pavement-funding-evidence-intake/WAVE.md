---
wave: priority-a-pavement-funding-evidence-intake
date_open: 2026-05-14
status: done
source: data/tier-pavement-funding-evidence-intake.csv
---

# Priority A Pavement Funding Evidence Intake

## Mission

Define intake requirements for priority-A pavement funding evidence artifacts
before any artifact capture, attachment, acceptance, or relief replay.

## Opening Rule

This wave may define required metadata for accepted funding artifacts. It must
not capture funding evidence, attach funding evidence, accept funding evidence,
reduce blockers, or replay asset-condition relief.

## Inputs Inherited

| Input | Source |
|---|---|
| Funding evidence source access | `data/tier-pavement-funding-evidence-source-access.csv` |
| Source access close | `waves/2026-05-14-priority-a-pavement-funding-evidence-source-access/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Intake requirements | done | `data/tier-pavement-funding-evidence-intake.csv`; `CLOSE.md` |

## Done Criteria

- Every held priority-A funding evidence source-access row has an intake row.
- Intake status is `artifact-required`.
- Evidence artifact remains `source-needed`.
- Evidence review status remains `not-reviewed`.
- Accepted evidence status remains `not-accepted`.
- Relief eligibility remains `not-eligible-for-relief`.
- Claim blocker delta remains `0`.
- Final gates pass before close.

## Non-Goals

- No funding artifact capture.
- No funding evidence acceptance.
- No asset-condition relief replay.
- No downgrade, exclusion, or funding mutation.
