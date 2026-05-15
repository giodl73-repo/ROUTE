---
wave: t2-asset-condition-map-publication-exclusion
date_open: 2026-05-15
status: done
---

# T2 Asset-Condition Map Publication Exclusion

## Mission

Exclude T2 asset-condition debt from current map publication claims while
preserving SLA, transit, upgrade, source-evidence, and repair obligations.

## Opening Rule

This is not repair funding, pavement evidence acceptance, or service-readiness
promotion. The exclusion may remove `publication` from T2 asset-condition debt,
but it must keep `sla`, `transit`, and `upgrade` blocked until the debt is paid
or accepted evidence lands.

## Inputs Inherited

- `data/tier-pavement-debt-budget.csv`
- `data/t2-asset-condition-map-publication-exclusion.csv`
- `data/map-publication-scope-decision.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| T2 asset-condition map publication exclusion | done | `data/t2-asset-condition-map-publication-exclusion.csv`; refreshed optimizer ledger/budget/backlog |

## Done Criteria

- T2 asset-condition debt no longer blocks `publication`.
- T2 asset-condition debt still blocks `sla`, `transit`, and `upgrade`.
- Full structural T1-T4 map publication has no residual `publication` blockers.
- No pavement repair, source evidence, or funding claim is accepted by this wave.

## Non-goals

- Do not accept pavement source evidence.
- Do not fund or clear pavement repair debt.
- Do not clear T4 terminal-access upgrade holds.
- Do not clear source snapshot evidence holds.
