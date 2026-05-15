---
wave: us2-pavement-evidence-replay
date_open: 2026-05-15
status: done
---

# US2 Pavement Evidence Replay

## Mission

Burn down the smallest T2 asset-condition evidence-debt row by refreshing HPMS
coverage for the US2 pavement evidence states and replaying pavement artifacts
through the optimizer.

## Opening Rule

Evidence-debt relief may land only when the pavement docket no longer emits a
source-needed debt row. Repair-debt rows remain held unless a funding
commitment, downgrade, exclusion, or accepted repair relief is attached.

## Inputs Inherited

- `data/tier-pavement-docket.csv`
- `data/tier-pavement-source-gaps.csv`
- `data/tier-pavement-debt-budget.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Fetch and replay US2 pavement evidence | done | HPMS-backed pavement replay removed the US2 evidence-debt budget row |

## Done Criteria

- US2 is absent from `data/tier-pavement-debt-budget.csv`.
- T2 asset-condition debt decreases from nine to eight budget-debt rows.
- T2 asset-condition debt cost decreases from $87.2M to $86.8M.
- Repair-debt rows remain held and relief-ineligible.

## Non-goals

- Do not claim full T2 asset-condition repair.
- Do not relieve I110 or I220 repair-debt rows without funding, downgrade, exclusion, or accepted repair relief.
- Do not promote T1 live-source evidence.
