---
wave: priority-a-pavement-funding-evidence-metadata-capture
date_open: 2026-05-14
status: done
source: data/tier-pavement-funding-evidence-metadata-capture.csv
---

# Priority A Pavement Funding Evidence Metadata Capture

## Mission

Record metadata-capture placeholders for priority-A pavement funding evidence
artifacts before any artifact attachment, acceptance, or relief replay.

## Opening Rule

This wave may capture the absence of accepted funding artifact metadata. It must
not attach funding evidence, accept funding evidence, reduce blockers, or replay
asset-condition relief.

## Inputs Inherited

| Input | Source |
|---|---|
| Funding evidence intake | `data/tier-pavement-funding-evidence-intake.csv` |
| Intake close | `waves/2026-05-14-priority-a-pavement-funding-evidence-intake/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Metadata capture placeholders | done | `data/tier-pavement-funding-evidence-metadata-capture.csv`; `CLOSE.md` |

## Done Criteria

- Every held priority-A funding evidence intake row has a metadata-capture row.
- Metadata capture status remains `source-needed`.
- Captured artifact remains `none`.
- Captured source title, URL, and commitment amount remain `source-needed`.
- Evidence review status remains `not-reviewed`.
- Accepted evidence status remains `not-accepted`.
- Relief eligibility remains `not-eligible-for-relief`.
- Claim blocker delta remains `0`.
- Final gates pass before close.

## Non-Goals

- No funding artifact attachment.
- No funding evidence acceptance.
- No asset-condition relief replay.
- No downgrade, exclusion, or funding mutation.
