---
wave: source-snapshot-publication-exclusion
date_open: 2026-05-15
status: done
---

# Source Snapshot Publication Exclusion

## Mission

Exclude the live source snapshot guard from current map publication claims while
preserving its evidence hold.

## Opening Rule

This is not evidence acceptance. The exclusion may remove `publication` from the
live snapshot guard, but it must keep `evidence` blocked until repeat-window or
archive-history proof exists.

## Inputs Inherited

- `data/source-fetch-policy.csv`
- `data/source-snapshot-publication-exclusion.csv`
- `data/map-publication-scope-decision.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Source snapshot publication exclusion | done | `data/source-snapshot-publication-exclusion.csv`; refreshed optimizer ledger/budget/backlog |

## Done Criteria

- Source snapshot guard no longer blocks `publication`.
- Source snapshot guard still blocks `evidence`.
- Full map publication remains blocked by any remaining publication blockers.
- No live source evidence is accepted by this wave.

## Non-goals

- Do not accept live-event snapshot evidence.
- Do not clear T4 terminal-access upgrade holds.
- Do not clear T2 asset-condition debt.
