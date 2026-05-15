---
wave: t4-terminal-access-map-exclusion
date_open: 2026-05-15
status: done
---

# T4 Terminal Access Map Exclusion

## Mission

Exclude unresolved seed-assigned T4 terminal-access overlays from current map
publication claims while preserving their upgrade/evidence holds.

## Opening Rule

This is not proof acceptance. The exclusion may remove `map` and `publication`
claims from unresolved T4 terminal-access rows, but it must keep the evidence
gap visible and preserve `upgrade` as blocked.

## Inputs Inherited

- `data/t3-t4-access-gaps.csv`
- `data/t4-terminal-access-map-exclusion.csv`
- `data/map-publication-scope-decision.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-constraint-budget.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| T4 terminal-access map exclusion | done | `data/t4-terminal-access-map-exclusion.csv`; refreshed optimizer ledger/budget |

## Done Criteria

- T4 terminal-access evidence gaps no longer block `map` or `publication`.
- T4 terminal-access evidence gaps still block `upgrade`.
- `data/intermodal_terminals.csv` remains seed-only and is not accepted as proof.
- Full map publication remains blocked by any remaining publication blockers.

## Non-goals

- Do not accept terminal-access proof.
- Do not clear T4 upgrade/evidence holds.
- Do not hide remaining publication blockers.
