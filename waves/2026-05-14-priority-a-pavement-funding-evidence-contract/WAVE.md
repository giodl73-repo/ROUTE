---
wave: priority-a-pavement-funding-evidence-contract
date_open: 2026-05-14
status: done
source: data/tier-pavement-funding-evidence-contract.csv
---

# Priority A Pavement Funding Evidence Contract

## Mission

Define the accepted funding evidence required before priority-A pavement repair
rows can become eligible for asset-condition relief replay.

## Opening Rule

This wave may add an evidence contract and doctrine references. It must not
accept funding evidence, reduce blockers, or replay asset-condition relief.

## Inputs Inherited

| Input | Source |
|---|---|
| Downgrade/exclusion decision | `data/tier-pavement-downgrade-exclusion-decision.csv` |
| Downgrade/exclusion close | `waves/2026-05-14-priority-a-pavement-downgrade-exclusion-decision/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Evidence contract | done | `data/tier-pavement-funding-evidence-contract.csv`; `CLOSE.md` |

## Done Criteria

- Every held priority-A downgrade/exclusion row has a funding evidence contract.
- Required evidence names an accepted programming document or state DOT
  commitment covering full repair cost.
- Minimum commitment amount equals or exceeds the repair cost proxy.
- Accepted evidence status remains `source-needed`.
- Final gates pass before close.

## Non-Goals

- No accepted funding evidence.
- No asset-condition relief replay.
- No downgrade, exclusion, or funding mutation.
