---
wave: us30-us6-pavement-evidence-replay
date_open: 2026-05-15
status: done
---

# US30 US6 Pavement Evidence Replay

## Mission

Burn down the remaining T2 asset-condition evidence-debt rows by refreshing HPMS
coverage for the US30 evidence states and replaying pavement artifacts through
the optimizer.

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
| Fetch and replay US30/US6 pavement evidence | done | HPMS-backed pavement replay removed the remaining evidence-debt budget rows |

## Done Criteria

- US30 and US6 are absent from `data/tier-pavement-debt-budget.csv`.
- T2 asset-condition debt decreases from eight to six budget-debt rows.
- T2 asset-condition debt cost decreases from $86.8M to $75.0M.
- Remaining pavement rows are repair-debt rows only.

## Non-goals

- Do not claim full T2 asset-condition repair.
- Do not relieve I110 or I220 repair-debt rows without funding, downgrade, exclusion, or accepted repair relief.
- Do not promote T1 live-source evidence.
